use crate::alloc_interface::{IAllocation, IAlloc};
use crate::wasm::wasm_mutex::Mutex;
use core::ops::{Deref, DerefMut};

pub struct WASMAllocation<T: ?Sized> {
    /// Invariant: it is always safe to call
    /// [`as_mut_unchecked`](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.as_mut_unchecked)
    /// on this pointer
    ptr: *mut T,
}

impl<T: ?Sized> Deref for WASMAllocation<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: by invariant
        unsafe { self.ptr.as_ref_unchecked() }
    }
}
impl<T: ?Sized> DerefMut for WASMAllocation<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: by invariant
        unsafe { self.ptr.as_mut_unchecked() }
    }
}

unsafe impl<T: ?Sized> IAllocation for WASMAllocation<T> {
    type MapResult<U: ?Sized> = WASMAllocation<U>;

    unsafe fn unsized_map<U: ?Sized>(
        self,
        f: impl FnOnce(&mut Self::Target) -> &mut U,
    ) -> Self::MapResult<U> {
        unsafe {
            #[cfg(debug_assertions)]
            {
                *NUM_ACTIVE_ALLOCATIONS.lock() += 1;
            }

            // Invariant: guaranteed by f
            WASMAllocation {
                // Safety: by invariant
                ptr: f(self.ptr.as_mut_unchecked()),
            }
        }
    }
}

#[cfg(debug_assertions)]
impl<T: ?Sized> Drop for WASMAllocation<T> {
    fn drop(&mut self) {
        unsafe { self.ptr.drop_in_place() };

        let mut guard = NUM_ACTIVE_ALLOCATIONS.lock();
        if *guard == 0 {
            panic!("Something really bad happened; num active allocations became negative");
        }
        *guard -= 1;
    }
}

#[cfg(debug_assertions)]
static NUM_ACTIVE_ALLOCATIONS: Mutex<usize> = Mutex::new(0);

pub struct WASMAlloc;

unsafe impl IAlloc for WASMAlloc {
    type Allocation<T: ?Sized> = WASMAllocation<T>;

    fn alloc_raw(num_bytes: usize, alignment: usize) -> Self::Allocation<()> {
        #[cfg(debug_assertions)]
        {
            *NUM_ACTIVE_ALLOCATIONS.lock() += 1;
        }

        WASMAllocation {
            ptr: wasm_alloc_internals::append_raw(num_bytes, alignment),
        }
    }

    fn reset() {
        #[cfg(debug_assertions)]
        if *NUM_ACTIVE_ALLOCATIONS.lock() != 0 {
            panic!("Attempted to reset memory when not all allocations are dropped");
        }

        unsafe {
            // Safety: when debug_assertions is enabled, by previous if statement; otherwise, just
            // trust me :)
            wasm_alloc_internals::reset();
        }
    }
}

// VERY BAD unsafe code.
mod wasm_alloc_internals {
    use crate::wasm::wasm_mutex::Mutex;

    static CURR_LIMIT: Mutex<*mut ()> = Mutex::new(heap_base());
    static CURR_END: Mutex<*mut ()> = Mutex::new(heap_base());

    pub fn append_raw(num_bytes: usize, alignment: usize) -> *mut () {
        debug_assert!(alignment.is_power_of_two());

        let mut curr_end = CURR_END.lock();
        let mut curr_limit = CURR_LIMIT.lock();

        // Align the pointer to the given alignment
        *curr_end = unsafe { curr_end.byte_add(curr_end.align_offset(alignment)) };

        let res = *curr_end;

        // Update heap end by given size
        *curr_end = unsafe { curr_end.byte_add(num_bytes) };

        // Request more memory from JS if necessary to make the pointer valid
        if *curr_end > *curr_limit {
            let new_limit = unsafe { request_enough_mem_for_ptr(*curr_limit) };
            if new_limit == core::ptr::null_mut() {
                // Make sure to drop guards before panicking
                drop((curr_end, curr_limit));

                panic!("Failed to request enough memory from JS");
            }

            *curr_limit = new_limit;
        }

        res
    }

    /// Clears all data on the heap. Only do this if we know no references remain to heap data.
    pub unsafe fn reset() {
        *CURR_END.lock() = heap_base();
    }

    unsafe extern "C" {
        /// Links to a symbol defined by the linker that marks the beginning of the heap
        static __heap_base: u8;
    }

    #[link(wasm_import_module = "env")]
    unsafe extern "C" {
        /// Requests JS to expand WASM memory to fit the pointer given. This returns the new memory
        /// limit if the operations succeeded and a null pointer otherwise
        fn request_enough_mem_for_ptr(ptr: *mut ()) -> *mut ();
    }

    #[allow(unused_unsafe)] // TODO: why does cargo check warn about this?
    const fn heap_base() -> *mut () {
        ( unsafe { &raw const __heap_base }) as *mut ()
    }
}
