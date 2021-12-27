
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonZeroUsize(usize);

impl NonZeroUsize {

    #[inline]
    pub fn new(n: usize) -> Option<Self> {
        if n != 0 {
            // SAFETY: we just checked that there's no `0`
            Some(unsafe { Self(n) })
        } else {
            None
        }
    }
}

impl Clone for NonZeroUsize {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for NonZeroUsize {}
