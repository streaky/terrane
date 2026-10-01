pub struct ScopedView<'view> {
    marker: std::marker::PhantomData<&'view str>,
}

pub trait RequiredBase {}

pub trait SupertraitView<'view>: RequiredBase {
    fn view(&self, state: &'view String) -> ScopedView<'view>;
}

impl<F> RequiredBase for F {}

impl<'view, F> SupertraitView<'view> for F
where
    F: Fn(&'view String) -> ScopedView<'view>,
{
    fn view(&self, state: &'view String) -> ScopedView<'view> {
        self(state)
    }
}

pub fn render_supertrait(
    state: &String,
    view: impl for<'view> SupertraitView<'view>,
) -> ScopedView<'_> {
    view.view(state)
}
