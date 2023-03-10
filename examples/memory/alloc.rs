// Code extracted from ministark

use ark_std::alloc::Global;
use once_cell::sync::Lazy;
use std::alloc::AllocError;
use std::alloc::Allocator;
use std::alloc::Layout;
use std::ptr::NonNull;

#[cfg(target_arch = "aarch64")]
pub static PAGE_SIZE: Lazy<usize> =
    Lazy::new(|| unsafe { libc::sysconf(libc::_SC_PAGESIZE).try_into().unwrap() });

pub struct PageAlignedAllocator;

// TODO: come up with better allocation abstraction for different architectures
#[cfg(target_arch = "aarch64")]
unsafe impl Allocator for PageAlignedAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Global.allocate(layout.align_to(*PAGE_SIZE).unwrap().pad_to_align())
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout.align_to(*PAGE_SIZE).unwrap().pad_to_align())
    }
}

#[cfg(not(target_arch = "aarch64"))]
unsafe impl Allocator for PageAlignedAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout)
    }
}
