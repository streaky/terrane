#[derive(Clone, Copy)]
pub struct CopyView<'view> {
    value: &'view str,
}
impl CopyView<'_> {
    pub fn size(self) -> usize { self.value.len() }
    pub fn bind(self) -> Self { self }
}
pub struct Holder {
    value: String,
}
impl Holder {
    pub fn new() -> Self { Self { value: "hello".to_owned() } }
    pub fn copy_view(&self) -> CopyView<'_> { CopyView { value: &self.value } }
    pub fn replace(&mut self, value: &str) { self.value = value.to_owned(); }
}
pub struct ConditionalView<'view> { value: &'view str }
impl<'view> Copy for ConditionalView<'view> where 'view: 'static {}
impl<'view> Clone for ConditionalView<'view> where 'view: 'static {
    fn clone(&self) -> Self { *self }
}
impl ConditionalView<'_> {
    pub fn size(self) -> usize { self.value.len() }
}
impl Holder {
    pub fn conditional_view(&self) -> ConditionalView<'_> { ConditionalView { value: &self.value } }
}
