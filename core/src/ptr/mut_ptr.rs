#[lang = "mut_ptr"]
impl<T: ?Sized> *mut T {
    #[inline]
    pub fn is_null(self) -> bool {
        // Compare via a cast to a thin pointer, so fat pointers are only
        // considering their "data" part for null-ness.
        (self as *mut u8).guaranteed_eq(crate::ptr::null_mut())
    }

    #[inline]
    pub fn guaranteed_eq(self, other: *mut T) -> bool
    where
        T: Sized,
    {
        crate::intrinsics::ptr_guaranteed_eq(self as *const _, other as *const _)
    }

    #[inline]
    pub const fn cast<U>(self) -> *mut U {
        self as _
    }

    #[inline(always)]
    pub unsafe fn write_bytes(self, val: u8, count: usize)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `write_bytes`.
        crate::ptr::write_bytes(self, val, count)
    }
}

#[lang = "mut_slice_ptr"]
impl<T> *mut [T] {
    #[inline(always)]
    pub fn len(self) -> usize {
        crate::ptr::metadata(self)
    }

    #[inline(always)]
    pub const fn as_mut_ptr(self) -> *mut T {
        self as *mut T
    }
}

// Equality for pointers
impl<T: ?Sized> PartialEq for *mut T {
    #[inline(always)]
    fn eq(&self, other: &*mut T) -> bool {
        *self == *other
    }
}

impl<T: ?Sized> Eq for *mut T {}
