
pub trait Into<T>: Sized {
    fn into(self) -> T;
}

pub trait From<T>: Sized {
    #[lang = "from"]
    fn from(_: T) -> Self;
}

// From implies Into
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}

// From (and thus Into) is reflexive
impl<T> From<T> for T {
    fn from(t: T) -> T {
        t
    }
}


pub enum Infallible {}

impl Copy for Infallible {}

impl Clone for Infallible {
    fn clone(&self) -> Infallible {
        match *self {}
    }
}

impl PartialEq for Infallible {
    fn eq(&self, _: &Infallible) -> bool {
        match *self {}
    }
}

impl Eq for Infallible {}

impl PartialOrd for Infallible {
    fn partial_cmp(&self, _other: &Self) -> Option<crate::cmp::Ordering> {
        match *self {}
    }
}

impl Ord for Infallible {
    fn cmp(&self, _other: &Self) -> crate::cmp::Ordering {
        match *self {}
    }
}