mod metadata;
mod non_null;

mod const_ptr;
mod mut_ptr;

pub use non_null::NonNull;

pub use metadata::{from_raw_parts, from_raw_parts_mut, metadata};

pub use crate::intrinsics::copy_nonoverlapping;
pub use crate::intrinsics::write_bytes;

#[inline(always)]
#[rustc_promotable]
pub const fn null<T>() -> *const T {
    0 as *const T
}

#[inline(always)]
#[rustc_promotable]
pub const fn null_mut<T>() -> *mut T {
    0 as *mut T
}

#[inline]
pub const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
    from_raw_parts(data.cast(), len)
}

#[inline]
pub const fn slice_from_raw_parts_mut<T>(data: *mut T, len: usize) -> *mut [T] {
    from_raw_parts_mut(data.cast(), len)
}
