pub enum Result<T, E> {
    /// Contains the success value
    #[lang = "Ok"]
    Ok(T),

    /// Contains the error value
    #[lang = "Err"]
    Err(E),
}

use crate::ops::{self, ControlFlow};
use crate::convert;

impl<T, E> ops::Try for Result<T, E> {
    type Output = T;
    type Residual = Result<convert::Infallible, E>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Ok(output)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

impl<T, E, F: From<E>> ops::FromResidual<Result<convert::Infallible, E>> for Result<T, F> {
    #[inline]
    fn from_residual(residual: Result<convert::Infallible, E>) -> Self {
        match residual {
            Err(e) => Err(From::from(e)),
        }
    }
}
