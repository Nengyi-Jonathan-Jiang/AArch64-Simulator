use crate::alloc_interface::{IAlloc, IAllocation};
use crate::components::lru_cache::FixedSizeLRUCache;
use crate::components::sizes::Addr;
use crate::components::MemoryAccess;
use crate::zero_init::{zeroInit, ZeroInit};
use crate::{Alloc, Allocation};
use core::mem::transmute;
use core::ops::{Div, Mul};
use hybrid_array::{Array, ArraySize};
use macro_rules_attribute::derive;
use typenum::{Prod, Quot};

// WASM pages are 64 KiB per page, with a maximum of 65536 pages (4GiB).
const PAGE_SIZE_LOG: usize = 16;
const PAGE_SIZE: usize = 1 << PAGE_SIZE_LOG; // (2 ^ 16) bytes = 65536 bytes = 64 KiB

const NUM_PAGES: usize = 65536;

#[repr(align(65536))] // align to PAGE_SIZE = 2 ^ 16 = 65536
struct GrowableMemory {
    page_table: [u16; NUM_PAGES], // This will take 2 pages (128 KiB) of memory
}

impl Default for GrowableMemory {
    fn default() -> Self {
        GrowableMemory {
            page_table: [0; NUM_PAGES],
        }
    }
}

unsafe impl ZeroInit for GrowableMemory {}

impl GrowableMemory {
    pub fn get_page_for(&mut self, addr: Addr) -> &mut [u8; PAGE_SIZE] {
        let page_index = (addr >> PAGE_SIZE_LOG) as usize;
        let page_entry = &mut self.page_table[page_index];

        let page_ptr: *mut [u8; PAGE_SIZE];

        if *page_entry == 0 {
            // Impossible; this means we just haven't allocated this page yet

            unsafe {
                let page: Allocation<[u8; PAGE_SIZE]> = Alloc::alloc_raw(PAGE_SIZE, PAGE_SIZE)
                    // Safety: size is correct
                    .to_uninit()
                    .init_with([0u8; PAGE_SIZE]);

                // This is safe by requirements on IAllocation as long as we transmute back into an
                // Allocation and drop it exactly when the GrowableMemory is dropped
                page_ptr = transmute(page);
            }

            *page_entry = (page_ptr as usize >> PAGE_SIZE_LOG) as _;
        } else {
            page_ptr = (((*page_entry) as usize) << PAGE_SIZE_LOG) as *mut [u8; PAGE_SIZE];
        }

        // Safety: page is a valid pointer. In addition, since this function takes &mut
        // self and is the only function able to access page_ptrs, it is impossible for this
        // pointer to alias.
        unsafe { page_ptr.as_mut_unchecked() }
    }
}

impl Drop for GrowableMemory {
    fn drop(&mut self) {
        for i in self.page_table {
            if i != 0 {
                unsafe {
                    let page: *mut [u8; PAGE_SIZE] = (i as usize * PAGE_SIZE) as _;
                    let page: Allocation<[u8; PAGE_SIZE]> = transmute(page);
                    drop(page);
                }
            }
        }
    }
}

#[derive(Default, zeroInit!)]
pub struct DirectMemoryAccess {
    memory: GrowableMemory,
}

impl MemoryAccess for DirectMemoryAccess {
    fn get(&mut self, addr: Addr) -> Result<&mut u8, ()> {
        let page = self.memory.get_page_for(addr);
        Ok(unsafe {
            page.as_mut_ptr()
                .offset((addr as usize & !(PAGE_SIZE - 1)) as isize)
                .as_mut_unchecked()
        })
    }
}

struct CacheLine<N: ArraySize> {
    tag: Addr,
    bytes: Array<u8, N>,
    dirty: bool,
    write_back_ptr: *mut Array<u8, N>,
}

/// Parameters:
/// - A: Associativity
/// - B: Bytes per cache line
/// - S: number of Sets == total Capacity / (Associativity * Bytes per cache line)
pub struct CachedMemoryAccess<A: ArraySize, B: ArraySize, S: ArraySize> {
    memory: GrowableMemory,
    cache: Array<FixedSizeLRUCache<Option<CacheLine<B>>, A>, S>,
}
