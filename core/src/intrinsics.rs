

extern "rust-intrinsic" {
    pub fn transmute<T, U>(e: T) -> U;
    pub fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);
    pub fn ptr_guaranteed_eq<T>(ptr: *const T, other: *const T) -> bool;
    pub fn write_bytes<T>(dst: *mut T, val: u8, count: usize);
}
