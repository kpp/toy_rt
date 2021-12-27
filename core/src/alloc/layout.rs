
use crate::num::NonZeroUsize;

#[lang = "alloc_layout"]
pub struct Layout {
    size_: usize,
    pub align_: NonZeroUsize,
}

impl Layout {
    #[inline]
    pub const fn size(&self) -> usize {
        self.size_
    }
}

impl Clone for Layout {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Layout {}


pub struct LayoutError;
