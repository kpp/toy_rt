// implements the unary operator "op &T"
// based on "op T" where T is expected to be `Copy`able
macro_rules! forward_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for &$t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    }
}

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}


#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;

    #[must_use]
    fn add(self, rhs: Rhs) -> Self::Output;
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add for $t {
            type Output = $t;

            #[inline]
            fn add(self, other: $t) -> $t { self + other }
        }

        forward_ref_binop! { impl Add, add for $t, $t }
    )*)
}

add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

#[lang = "sub"]
pub trait Sub<Rhs = Self> {
    type Output;

    #[must_use]
    fn sub(self, rhs: Rhs) -> Self::Output;
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Sub for $t {
            type Output = $t;

            #[inline]
            fn sub(self, other: $t) -> $t { self - other }
        }

        forward_ref_binop! { impl Sub, sub for $t, $t }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }


#[lang = "mul"]
pub trait Mul<Rhs = Self> {
    type Output;

    #[must_use]
    fn mul(self, rhs: Rhs) -> Self::Output;
}

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: $t) -> $t { self * other }
        }

        forward_ref_binop! { impl Mul, mul for $t, $t }
    )*)
}

mul_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

#[lang = "neg"]
pub trait Neg {
    type Output;

    #[must_use]
    fn neg(self) -> Self::Output;
}

macro_rules! neg_impl {
    ($($t:ty)*) => ($(
        impl Neg for $t {
            type Output = $t;

            #[inline]
            fn neg(self) -> $t { -self }
        }

        forward_ref_unop! { impl Neg, neg for $t }
    )*)
}

neg_impl! { isize i8 i16 i32 i64 i128 f32 f64 }


// FIXME move to ops/bit.rs

#[lang = "bitor"]
pub trait BitOr<Rhs = Self> {
    type Output;

    #[must_use]
    fn bitor(self, rhs: Rhs) -> Self::Output;
}

macro_rules! bitor_impl {
    ($($t:ty)*) => ($(
        impl BitOr for $t {
            type Output = $t;

            #[inline]
            fn bitor(self, rhs: $t) -> $t { self | rhs }
        }

        forward_ref_binop! { impl BitOr, bitor for $t, $t }
    )*)
}

bitor_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

#[lang = "not"]
pub trait Not {
    type Output;

    #[must_use]
    fn not(self) -> Self::Output;
}

macro_rules! not_impl {
    ($($t:ty)*) => ($(
        impl Not for $t {
            type Output = $t;

            #[inline]
            fn not(self) -> $t { !self }
        }

        forward_ref_unop! { impl Not, not for $t }
    )*)
}

not_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
