use std::sync::atomic::{AtomicI64, Ordering};

static TRACE: AtomicI64 = AtomicI64::new(0);

pub struct Label {
    value: i64,
}

pub struct Button {
    value: i64,
}

pub struct Row {
    value: i64,
}

#[derive(Clone)]
pub struct Widget {
    value: i64,
}

pub struct ScopedView<'view, Message, Theme> {
    value: i64,
    marker: std::marker::PhantomData<(&'view (), Message, Theme)>,
}

impl From<Label> for Widget {
    fn from(label: Label) -> Self {
        Self { value: label.value }
    }
}

impl From<Button> for Widget {
    fn from(button: Button) -> Self {
        Self {
            value: button.value,
        }
    }
}

impl From<Row> for Widget {
    fn from(row: Row) -> Self {
        Self { value: row.value }
    }
}

impl<'view, Message, Theme> From<Label> for ScopedView<'view, Message, Theme> {
    fn from(label: Label) -> Self {
        Self {
            value: label.value,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'view, Message, Theme> From<Widget> for ScopedView<'view, Message, Theme> {
    fn from(widget: Widget) -> Self {
        Self {
            value: widget.value,
            marker: std::marker::PhantomData,
        }
    }
}

pub fn label(value: i64) -> Label {
    Label { value }
}

pub fn button(value: i64) -> Button {
    Button { value }
}

pub fn traced_label(value: i64) -> Label {
    let sequence = TRACE.fetch_add(1, Ordering::SeqCst) + 1;
    Label {
        value: value * 10 + sequence,
    }
}

pub fn reset_trace() {
    TRACE.store(0, Ordering::SeqCst);
}

pub fn into_widget<Item>(item: Item) -> Widget
where
    Item: Into<Widget>,
{
    item.into()
}

pub fn row(children: Vec<Widget>) -> Row {
    Row {
        value: children
            .into_iter()
            .fold(0, |combined, child| combined * 100 + child.value),
    }
}

pub trait ViewFn<'view, State, Message, Theme> {
    fn view(&self, state: &'view State) -> ScopedView<'view, Message, Theme>;
}

impl<'view, F, State: 'view, Message, Theme, WidgetValue>
    ViewFn<'view, State, Message, Theme> for F
where
    F: Fn(&'view State) -> WidgetValue,
    WidgetValue: Into<ScopedView<'view, Message, Theme>>,
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
