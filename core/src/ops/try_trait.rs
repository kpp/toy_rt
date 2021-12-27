use crate::ops::ControlFlow;

#[lang = "Try"]
pub trait Try: FromResidual {
    /// The type of the value produced by `?` when *not* short-circuiting.
    type Output;

    type Residual;

    #[lang = "from_output"]
    fn from_output(output: Self::Output) -> Self;

    #[lang = "branch"]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
}

pub trait FromResidual<R = <Self as Try>::Residual> {
    #[lang = "from_residual"]
    fn from_residual(residual: R) -> Self;
}
