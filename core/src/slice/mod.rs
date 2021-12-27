#[lang = "slice"]
impl<T> [T] {
    pub fn len(&self) -> usize {
        crate::ptr::metadata(self)
    }
}
