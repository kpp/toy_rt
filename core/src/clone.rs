#[lang = "clone"]
pub trait Clone: Sized {
    fn clone(&self) -> Self;
}

mod clone_impls {
    use super::Clone;

    impl Clone for usize {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for isize {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for u8 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for i8 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for u16 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for i16 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for u32 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for i32 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for u64 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for i64 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for u128 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for i128 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for f32 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for f64 {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for bool {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Clone for char {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: ?Sized> Clone for *const T {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: ?Sized> Clone for *mut T {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }

    /// Shared references can be cloned, but mutable references *cannot*!
    impl<T: ?Sized> Clone for &T {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }

    /// Shared references can be cloned, but mutable references *cannot*!
    impl<T: ?Sized> !Clone for &mut T {}
}
