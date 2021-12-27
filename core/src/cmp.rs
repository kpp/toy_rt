#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    #[must_use]
    fn eq(&self, other: &Rhs) -> bool;

    #[inline]
    #[must_use]
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

pub trait Eq: PartialEq<Self> {
    // This should never be implemented by hand.
    #[inline]
    fn assert_receiver_is_total_eq(&self) {}
}

pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}

use self::Ordering::*;

#[lang = "partial_ord"]
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    #[must_use]
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    #[inline]
    #[must_use]
    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    #[inline]
    #[must_use]
    fn le(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    #[inline]
    #[must_use]
    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    #[inline]
    #[must_use]
    fn ge(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}

pub trait Ord: Eq + PartialOrd<Self> {
    #[must_use]
    fn cmp(&self, other: &Self) -> Ordering;
}

mod impls {
    use crate::cmp::Ordering::{self, Equal, Greater, Less};

    macro_rules! partial_eq_impl {
        ($($t:ty)*) => ($(
            impl PartialEq for $t {
                #[inline]
                fn eq(&self, other: &$t) -> bool { (*self) == (*other) }
                #[inline]
                fn ne(&self, other: &$t) -> bool { (*self) != (*other) }
            }
        )*)
    }

    partial_eq_impl! {
        bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64
    }

    impl PartialEq for () {
        #[inline]
        fn eq(&self, _other: &()) -> bool {
            true
        }
        #[inline]
        fn ne(&self, _other: &()) -> bool {
            false
        }
    }

    macro_rules! eq_impl {
        ($($t:ty)*) => ($(
            impl Eq for $t {}
        )*)
    }

    eq_impl! { () bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

    macro_rules! partial_ord_impl {
        ($($t:ty)*) => ($(
            impl PartialOrd for $t {
                #[inline]
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    match (*self <= *other, *self >= *other) {
                        (false, false) => None,
                        (false, true) => Some(Greater),
                        (true, false) => Some(Less),
                        (true, true) => Some(Equal),
                    }
                }
                #[inline]
                fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
                #[inline]
                fn le(&self, other: &$t) -> bool { (*self) <= (*other) }
                #[inline]
                fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
                #[inline]
                fn gt(&self, other: &$t) -> bool { (*self) > (*other) }
            }
        )*)
    }

    partial_ord_impl! { f32 f64 }

    impl PartialOrd for () {
        #[inline]
        fn partial_cmp(&self, _: &()) -> Option<Ordering> {
            Some(Equal)
        }
    }

    macro_rules! ord_impl {
        ($($t:ty)*) => ($(
            impl PartialOrd for $t {
                #[inline]
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
                #[inline]
                fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
                #[inline]
                fn le(&self, other: &$t) -> bool { (*self) <= (*other) }
                #[inline]
                fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
                #[inline]
                fn gt(&self, other: &$t) -> bool { (*self) > (*other) }
            }

            impl Ord for $t {
                #[inline]
                fn cmp(&self, other: &$t) -> Ordering {
                    if *self < *other { Less }
                    else if *self == *other { Equal }
                    else { Greater }
                }
            }
        )*)
    }

    impl Ord for () {
        #[inline]
        fn cmp(&self, _other: &()) -> Ordering {
            Equal
        }
    }

    ord_impl! { bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
}