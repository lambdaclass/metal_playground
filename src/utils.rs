// stolen from ministark
pub fn void_ptr<T>(v: &T) -> *const core::ffi::c_void {
    v as *const T as *const core::ffi::c_void
}

pub fn deref_void_ptr<T>(ptr: *const core::ffi::c_void) -> *const T {
    ptr as *const T
}