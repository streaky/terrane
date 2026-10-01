pub struct Widget<'view>(std::marker::PhantomData<&'view str>);
pub struct ScopedView<'view>(std::marker::PhantomData<&'view str>);

pub fn widget(_value: &str) -> Widget<'_> {
    Widget(std::marker::PhantomData)
}

impl<'view> From<Widget<'view>> for ScopedView<'view> {
    fn from(_widget: Widget<'view>) -> Self {
        Self(std::marker::PhantomData)
    }
}

pub trait MultiViewFn<'view, State> {
    fn view(&self, state: &'view State) -> ScopedView<'view>;
    fn duplicate(&self, state: &'view State) -> ScopedView<'view>;
}

impl<'view, F, State: 'view, Value> MultiViewFn<'view, State> for F
where
    F: Fn(&'view State) -> Value,
    Value: Into<ScopedView<'view>>,
{
    fn view(&self, state: &'view State) -> ScopedView<'view> {
        self(state).into()
    }

    fn duplicate(&self, state: &'view State) -> ScopedView<'view> {
        self(state).into()
    }
}

pub fn render_multi<State>(
    state: &State,
    view: impl for<'view> MultiViewFn<'view, State>,
) -> i64 {
    let _ = view.view(state);
    let _ = view.duplicate(state);
    2
}
