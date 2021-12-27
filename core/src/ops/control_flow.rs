pub enum ControlFlow<B, C = ()> {
    #[lang = "Continue"]
    Continue(C),
    #[lang = "Break"]
    Break(B),
}
