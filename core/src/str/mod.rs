
#[lang = "str"]
#[cfg(not(test))]
impl str {
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { crate::mem::transmute(self) }
    }
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }
    pub fn as_ptr(&self) -> *const u8 {
        self as *const str as *const u8
    }
}