pub struct FirstWidget<'view> {
    value: i64,
    marker: std::marker::PhantomData<&'view str>,
}

pub struct SecondWidget<'view> {
    value: i64,
    marker: std::marker::PhantomData<&'view str>,
}

pub struct ScopedView<'view> {
    value: i64,
    marker: std::marker::PhantomData<&'view str>,
}

pub fn first_widget(value: &str) -> FirstWidget<'_> {
    FirstWidget {
        value: value.len() as i64,
        marker: std::marker::PhantomData,
    }
}

pub fn second_widget(value: &str) -> SecondWidget<'_> {
    SecondWidget {
        value: value.len() as i64,
        marker: std::marker::PhantomData,
    }
}

impl<'view> From<FirstWidget<'view>> for ScopedView<'view> {
    fn from(widget: FirstWidget<'view>) -> Self {
        Self {
            value: widget.value,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'view> From<SecondWidget<'view>> for ScopedView<'view> {
    fn from(widget: SecondWidget<'view>) -> Self {
        Self {
            value: widget.value,
            marker: std::marker::PhantomData,
        }
    }
}

pub fn first_row<'view>(children: Vec<FirstWidget<'view>>) -> ScopedView<'view> {
    ScopedView {
        value: children.into_iter().map(|child| child.value).sum(),
        marker: std::marker::PhantomData,
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

pub fn render_scoped<State>(
    state: &State,
    view: impl for<'view> ViewFn<'view, State>,
) -> i64 {
    view.view(state).value
}

pub trait RestrictedViewFn<'view> {
    fn view(&self, state: &'view String) -> ScopedView<'view>;
}

impl<'view, F> RestrictedViewFn<'view> for F
where
    F: Fn(&'view String) -> FirstWidget<'view>,
{
    fn view(&self, state: &'view String) -> ScopedView<'view> {
        self(state).into()
    }
}

pub fn render_restricted(
    state: &String,
    view: impl for<'view> RestrictedViewFn<'view>,
) -> i64 {
    view.view(state).value
}
