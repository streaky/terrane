pub struct Label {
    value: i64,
}

pub struct ScopedView<'view, Message, Theme> {
    value: i64,
    marker: std::marker::PhantomData<(&'view (), Message, Theme)>,
}

impl<'view, Message, Theme> From<Label> for ScopedView<'view, Message, Theme> {
    fn from(label: Label) -> Self {
        Self {
            value: label.value,
            marker: std::marker::PhantomData,
        }
    }
}

pub fn label(value: i64) -> Label {
    Label { value }
}

pub trait ViewFn<'view, State, Message, Theme> {
    fn view(&self, state: &'view State) -> ScopedView<'view, Message, Theme>;
}

impl<'view, F, State: 'view, Message, Theme, Widget> ViewFn<'view, State, Message, Theme>
    for F
where
    F: Fn(&'view State) -> Widget,
    Widget: Into<ScopedView<'view, Message, Theme>>,
{
    fn view(&self, state: &'view State) -> ScopedView<'view, Message, Theme> {
        self(state).into()
    }
}

pub fn render_scoped<State, Message, Theme>(
    state: &State,
    message: Message,
    theme: Theme,
    view: impl for<'view> ViewFn<'view, State, Message, Theme>,
) -> i64 {
    let _ = (message, theme);
    view.view(state).value
}
