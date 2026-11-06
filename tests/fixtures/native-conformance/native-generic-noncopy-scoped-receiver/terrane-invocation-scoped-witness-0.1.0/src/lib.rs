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
#[derive(Clone, Copy)]
pub struct GenericView<'view, T> {
    value: &'view str,
    marker: std::marker::PhantomData<T>,
}
impl<T> GenericView<'_, T> {
    pub fn size(self) -> usize { self.value.len() }
}
impl Holder {
    pub fn generic_copy(&self) -> GenericView<'_, i64> {
        GenericView { value: &self.value, marker: std::marker::PhantomData }
    }
    pub fn generic_clone(&self) -> GenericView<'_, String> {
        GenericView { value: &self.value, marker: std::marker::PhantomData }
    }
}
