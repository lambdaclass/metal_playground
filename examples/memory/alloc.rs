use once_cell::sync::Lazy;

#[cfg(target_arch = "aarch64")]
pub static PAGE_SIZE: Lazy<usize> =
    Lazy::new(|| unsafe { libc::sysconf(libc::_SC_PAGESIZE).try_into().unwrap() });

pub struct PageAlignedAllocator;

use std::alloc::{AllocError, Allocator, Global, Layout};
use std::ptr::NonNull;

#[cfg(target_arch = "aarch64")]
unsafe impl Allocator for PageAlignedAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Global.allocate(layout.align_to(*PAGE_SIZE).unwrap())
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout.align_to(*PAGE_SIZE).unwrap())
    }
}
