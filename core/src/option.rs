pub enum Option<T> {
    #[lang = "None"]
    None,

    #[lang = "Some"]
    Some(T),
}