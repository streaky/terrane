pub struct BorrowedLabel<'view> {
    marker: std::marker::PhantomData<&'view str>,
}

pub struct ScopedView<'view> {
    marker: std::marker::PhantomData<&'view str>,
}

pub fn borrowed_label(value: &str) -> BorrowedLabel<'_> {
    let _ = value;
    BorrowedLabel {
        marker: std::marker::PhantomData,
    }
}

impl<'view> From<BorrowedLabel<'view>> for ScopedView<'view> {
    fn from(_: BorrowedLabel<'view>) -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
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

pub fn render_scoped<State>(state: &State, view: impl for<'view> ViewFn<'view, State>) {
    let _ = view.view(state);
}
