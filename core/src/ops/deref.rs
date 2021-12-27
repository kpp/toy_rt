


/// Indicates that a struct can be used as a method receiver, without the
/// `arbitrary_self_types` feature. This is implemented by stdlib pointer types like `Box<T>`,
/// `Rc<T>`, `&T`, and `Pin<P>`.
#[lang = "receiver"]
pub trait Receiver {
    // Empty.
}

impl<T: ?Sized> Receiver for &T {}

impl<T: ?Sized> Receiver for &mut T {}
