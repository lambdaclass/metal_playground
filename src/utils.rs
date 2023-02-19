// original code from ministark
pub fn void_ptr<T>(v: &T) -> *const core::ffi::c_void {
    v as *const T as *const core::ffi::c_void
}

pub fn deref_void_ptr<T>(ptr: *const core::ffi::c_void) -> *const T {
    ptr as *const T
}

pub fn is_power_of_two(n: usize) -> bool {
    n & (n - 1) == 0
}
