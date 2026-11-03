pub struct ScopedView<'view> {
    marker: std::marker::PhantomData<&'view str>,
}

pub trait SendView<'view>: Send + Unpin {
    fn view(&self, state: &'view String) -> ScopedView<'view>;
}

impl<'view, F> SendView<'view> for F
where
    F: Send + Unpin + Fn(&'view String) -> ScopedView<'view>,
{
    fn view(&self, state: &'view String) -> ScopedView<'view> {
        self(state)
    }
}

pub fn render_send(
    state: &String,
    view: impl for<'view> SendView<'view>,
) -> ScopedView<'_> {
    view.view(state)
}
