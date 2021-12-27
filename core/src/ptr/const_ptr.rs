#[lang = "const_ptr"]
impl<T: ?Sized> *const T {
    #[inline]
    pub fn is_null(self) -> bool {
        // Compare via a cast to a thin pointer, so fat pointers are only
        // considering their "data" part for null-ness.
        (self as *const u8).guaranteed_eq(crate::ptr::null())
    }

    #[inline]
    pub fn guaranteed_eq(self, other: *const T) -> bool
    where
        T: Sized,
    {
        crate::intrinsics::ptr_guaranteed_eq(self, other)
    }

    #[inline]
    pub const fn cast<U>(self) -> *const U {
        self as _
    }
}

#[lang = "const_slice_ptr"]
impl<T> *const [T] {
    #[inline]
    pub fn len(self) -> usize {
        crate::ptr::metadata(self)
    }

    #[inline]
    pub const fn as_ptr(self) -> *const T {
        self as *const T
    }
}

// Equality for pointers
impl<T: ?Sized> PartialEq for *const T {
    #[inline]
    fn eq(&self, other: &*const T) -> bool {
        *self == *other
    }
}

impl<T: ?Sized> Eq for *const T {}
