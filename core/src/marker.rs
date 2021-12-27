
#[lang = "copy"]
pub trait Copy: Clone {
    // Empty.
}
#[lang = "sized"]
pub trait Sized {
    // Empty.
}

/// Compiler-internal trait used to determine whether a type contains
/// any `UnsafeCell` internally, but not through an indirection.
/// This affects, for example, whether a `static` of that type is
/// placed in read-only static memory or writable static memory.
#[lang = "freeze"]
pub(crate) unsafe auto trait Freeze {}

//impl<T: ?Sized> !Freeze for UnsafeCell<T> {}
//unsafe impl<T: ?Sized> Freeze for PhantomData<T> {}
unsafe impl<T: ?Sized> Freeze for *const T {}
unsafe impl<T: ?Sized> Freeze for *mut T {}
unsafe impl<T: ?Sized> Freeze for &T {}
unsafe impl<T: ?Sized> Freeze for &mut T {}

mod copy_impls {
    impl Copy for usize {}
    impl Copy for isize {}
    impl Copy for u8 {}
    impl Copy for i8 {}
    impl Copy for u16 {}
    impl Copy for i16 {}
    impl Copy for u32 {}
    impl Copy for i32 {}
    impl Copy for u64 {}
    impl Copy for i64 {}
    impl Copy for u128 {}
    impl Copy for i128 {}
    impl Copy for f32 {}
    impl Copy for f64 {}
    impl Copy for bool {}
    impl Copy for char {}

    impl<T: ?Sized> Copy for *const T {}
    impl<T: ?Sized> Copy for *mut T {}

    /// Shared references can be copied, but mutable references *cannot*!
    impl<T: ?Sized> Copy for &T {}
}
