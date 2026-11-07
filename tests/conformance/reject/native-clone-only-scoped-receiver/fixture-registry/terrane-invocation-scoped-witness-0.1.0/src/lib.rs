#[derive(Clone)]
pub struct View<'a>(&'a str);
impl View<'_> {
    pub fn size(self) -> usize { self.0.len() }
}
pub struct Holder(String);
impl Holder {
    pub fn new() -> Self { Self("hello".to_owned()) }
    pub fn view(&self) -> View<'_> { View(&self.0) }
}
