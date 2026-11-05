pub struct View<'a>(&'a str);
impl View<'_> {
    pub fn size(&self) -> usize { self.0.len() }
}
pub fn query(sql: &str) -> View<'_> { View(sql) }
pub struct Holder { pub inner: Inner }
pub struct Inner { label: String }
impl Holder {
    pub fn new() -> Self { Self { inner: Inner { label: "hello".to_owned() } } }
}
impl Inner {
    pub fn view(&self) -> View<'_> { View(&self.label) }
}
