extern crate std;
use crate::alloc_interface::{IAlloc, IAllocation};
use core::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

pub struct NativeAllocation<T: ?Sized> {
    /// Invariant: it is always safe to call
    /// [`as_mut_unchecked`](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.as_mut_unchecked)
    /// on this pointer
    ptr: *mut T,
}

impl<T: ?Sized> Deref for NativeAllocation<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: by invariant
        unsafe { self.ptr.as_mut_unchecked() }
    }
}

impl<T: ?Sized> DerefMut for NativeAllocation<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: by invariant
        unsafe { self.ptr.as_mut_unchecked() }
    }
}

unsafe impl<T: ?Sized> IAllocation for NativeAllocation<T> {
    type MapResult<U: ?Sized> = NativeAllocation<U>;

    unsafe fn unsized_map<U: ?Sized>(
        self,
        f: impl FnOnce(&mut Self::Target) -> &mut U,
    ) -> Self::MapResult<U> {
        unsafe {
            #[cfg(debug_assertions)]
            {
                NUM_ACTIVE_ALLOCATIONS.fetch_add(1, SeqCst);
            }

            // Invariant: guaranteed by f
            NativeAllocation {
                // Safety: by invariant
                ptr: f(self.ptr.as_mut_unchecked()),
            }
        }
    }
}

#[cfg(debug_assertions)]
impl<T: ?Sized> Drop for NativeAllocation<T> {
    fn drop(&mut self) {
        unsafe { self.ptr.drop_in_place() };

        // This wraps around. This may be ok
        if NUM_ACTIVE_ALLOCATIONS.fetch_sub(1, SeqCst) == 0 {
            panic!("Something really bad happened; num active allocations became negative");
        }
    }
}

#[cfg(debug_assertions)]
static NUM_ACTIVE_ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

pub struct NativeAllocator;

unsafe impl IAlloc for NativeAllocator {
    type Allocation<T: ?Sized> = NativeAllocation<T>;

    fn alloc_raw(num_bytes: usize, alignment: usize) -> Self::Allocation<()> {
        #[cfg(debug_assertions)]
        {
            NUM_ACTIVE_ALLOCATIONS.fetch_add(1, SeqCst);
        }

        NativeAllocation {
            ptr: native_alloc_internals::append_raw(num_bytes, alignment),
        }
    }

    fn reset() {
        #[cfg(debug_assertions)]
        if NUM_ACTIVE_ALLOCATIONS.load(SeqCst) == 0 {
            panic!("Attempted to reset memory when not all allocations are dropped");
        }

        unsafe {
            native_alloc_internals::reset();
        }
    }
}

mod native_alloc_internals {
    use std::alloc::Layout;
    use std::boxed::Box;
    use std::ptr::slice_from_raw_parts_mut;
    use std::sync::Mutex;
    use std::vec::Vec;

    static MEM: Mutex<Vec<Box<[u8]>>> = Mutex::new(Vec::new());

    pub fn append_raw(num_bytes: usize, align: usize) -> *mut () {
        debug_assert_eq!(align.count_ones(), 1, "Align must be a power of 2");

        if num_bytes == 0 {
            // Can't return null pointer due to some Allocation operations panicing on null (even if
            // zero sized) so we have to do this instead
            return align as *mut ();
        }

        unsafe {
            // Safety: above check ensures num_bytes is nonzero
            let res_raw = std::alloc::alloc(Layout::from_size_align(num_bytes, align).unwrap());
            // Safety: Box::from_raw is ok because the layout is guaranteed to be ok by above
            MEM.lock()
                .unwrap()
                .push(Box::from_raw(slice_from_raw_parts_mut(res_raw, num_bytes)));
            res_raw as *mut ()
        }
    }

    pub unsafe fn reset() {
        MEM.lock().unwrap().clear();
    }
}
