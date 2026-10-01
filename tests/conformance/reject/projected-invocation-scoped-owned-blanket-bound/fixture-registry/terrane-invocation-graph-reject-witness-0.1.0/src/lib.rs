pub struct BadWidget {
    value: i64,
}

pub struct ScopedView<'view> {
    value: i64,
    marker: std::marker::PhantomData<&'view ()>,
}

pub fn bad_widget(value: i64) -> BadWidget {
    BadWidget { value }
}

pub trait ViewFn<'view, State> {
    fn view(&self, state: &'view State) -> ScopedView<'view>;
}

impl<'view, F, State: 'view, Widget> ViewFn<'view, State> for F
where
    F: Fn(&'view State) -> Widget,
    Widget: Into<ScopedView<'view>>,
{
    fn view(&self, state: &'view State) -> ScopedView<'view> {
        self(state).into()
    }
}

pub fn render_scoped<State>(
    state: &State,
    view: impl for<'view> ViewFn<'view, State>,
) -> i64 {
    view.view(state).value
}
