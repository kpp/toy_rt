#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}

impl<T: ?Sized> NonNull<T> {
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        NonNull { pointer: ptr as _ }
    }

    #[inline]
    pub fn new(ptr: *mut T) -> Option<Self> {
        if !ptr.is_null() {
            // SAFETY: The pointer is already checked and is not null
            Some(unsafe { NonNull { pointer: ptr as _ } })
        } else {
            None
        }
    }
    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.pointer as *mut T
    }
}

impl<T> NonNull<[T]> {
    #[inline]
    pub const fn slice_from_raw_parts(data: NonNull<T>, len: usize) -> Self {
        // SAFETY: `data` is a `NonNull` pointer which is necessarily non-null
        unsafe { Self::new_unchecked(crate::ptr::slice_from_raw_parts_mut(data.as_ptr(), len)) }
    }

    #[inline]
    pub fn len(self) -> usize {
        self.as_ptr().len()
    }

    #[inline]
    pub const fn as_non_null_ptr(self) -> NonNull<T> {
        // SAFETY: We know `self` is non-null.
        unsafe { NonNull::new_unchecked(self.as_ptr().as_mut_ptr()) }
    }

    #[inline]
    pub const fn as_mut_ptr(self) -> *mut T {
        self.as_non_null_ptr().as_ptr()
    }
}

impl<T: ?Sized> Clone for NonNull<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for NonNull<T> {}
