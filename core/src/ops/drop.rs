#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}
