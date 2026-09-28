pub struct ScopedView<'view> {
    pub marker: std::marker::PhantomData<&'view ()>,
}

pub fn scoped_view<'view>(value: &'view str) -> ScopedView<'view> {
    let _ = value;
    ScopedView {
        marker: std::marker::PhantomData,
    }
}
