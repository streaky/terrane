pub struct ScopedView<'view> {
    marker: std::marker::PhantomData<&'view str>,
}

pub unsafe trait UnsafeView<'view> {
    fn view(&self, state: &'view String) -> ScopedView<'view>;
}

unsafe impl<'view, F> UnsafeView<'view> for F
where
    F: Fn(&'view String) -> ScopedView<'view>,
{
    fn view(&self, state: &'view String) -> ScopedView<'view> {
        self(state)
    }
}

pub fn render_unsafe(
    state: &String,
    view: impl for<'view> UnsafeView<'view>,
) -> ScopedView<'_> {
    view.view(state)
}
