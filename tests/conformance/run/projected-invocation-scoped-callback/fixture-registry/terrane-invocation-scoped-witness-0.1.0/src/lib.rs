use std::sync::atomic::{AtomicI64, Ordering};

static TRACE: AtomicI64 = AtomicI64::new(0);

pub struct Label {
    value: i64,
}

#[derive(Clone)]
pub struct BorrowedLabel<'view> {
    value: &'view str,
}

pub struct BorrowedButton<'view> {
    value: &'view str,
}

pub struct BorrowedGroup<'view> {
    value: i64,
    marker: std::marker::PhantomData<&'view str>,
}

pub struct BorrowedValue<'view, Message> {
    value: &'view str,
    marker: std::marker::PhantomData<Message>,
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
pub type TerminalView<'view, Message = String, Theme = bool> =
    ScopedView<'view, Message, Theme>;

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
impl<'view, Message, Theme> From<BorrowedLabel<'view>>
    for ScopedView<'view, Message, Theme>
{
    fn from(label: BorrowedLabel<'view>) -> Self {
        Self {
            value: label.value.len() as i64,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'view, Message, Theme> From<BorrowedButton<'view>>
    for ScopedView<'view, Message, Theme>
{
    fn from(button: BorrowedButton<'view>) -> Self {
        Self {
            value: button.value.len() as i64 + 10,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'view, Message, Theme> From<BorrowedGroup<'view>>
    for ScopedView<'view, Message, Theme>
{
    fn from(group: BorrowedGroup<'view>) -> Self {
        Self {
            value: group.value,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'view, Message, Theme> From<BorrowedValue<'view, Message>>
    for ScopedView<'view, Message, Theme>
{
    fn from(value: BorrowedValue<'view, Message>) -> Self {
        Self {
            value: value.value.len() as i64,
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
pub fn borrowed_label(value: &String) -> BorrowedLabel<'_> {
    BorrowedLabel { value }
}

pub fn borrowed_button(value: &String) -> BorrowedButton<'_> {
    BorrowedButton { value }
}

pub fn borrowed_value(value: &String) -> BorrowedValue<'_, String> {
    BorrowedValue {
        value,
        marker: std::marker::PhantomData,
    }
}

pub fn borrowed_group<'view>(
    label_value: BorrowedLabel<'view>,
    button_value: BorrowedButton<'view>,
) -> BorrowedGroup<'view> {
    BorrowedGroup {
        value: label_value.value.len() as i64 * 100 + button_value.value.len() as i64,
        marker: std::marker::PhantomData,
    }
}

pub fn borrowed_label_group<'view>(label_value: BorrowedLabel<'view>) -> BorrowedGroup<'view> {
    BorrowedGroup {
        value: label_value.value.len() as i64,
        marker: std::marker::PhantomData,
    }
}

pub fn borrowed_button_group<'view>(button_value: BorrowedButton<'view>) -> BorrowedGroup<'view> {
    BorrowedGroup {
        value: button_value.value.len() as i64 + 10,
        marker: std::marker::PhantomData,
    }
}

pub fn borrowed_row<'view>(children: Vec<BorrowedLabel<'view>>) -> BorrowedGroup<'view> {
    BorrowedGroup {
        value: children
            .into_iter()
            .fold(0, |combined, child| combined * 100 + child.value.len() as i64),
        marker: std::marker::PhantomData,
    }
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
    fn view(&self, state: &'view State) -> TerminalView<'view, Message, Theme>;
}

impl<'view, F, State: 'view, Message, Theme, WidgetValue>
    ViewFn<'view, State, Message, Theme> for F
where
    F: Fn(&'view State) -> WidgetValue,
    WidgetValue: Into<TerminalView<'view, Message, Theme>>,
{
    fn view(&self, state: &'view State) -> TerminalView<'view, Message, Theme> {
        self(state).into()
    }
}

pub trait ExactViewFn<'view, Message, Theme> {
    fn view(&self, state: &'view String) -> ScopedView<'view, Message, Theme>;
}

impl<'view, F, Message, Theme, WidgetValue> ExactViewFn<'view, Message, Theme> for F
where
    F: Fn(&'view String) -> WidgetValue,
    WidgetValue: Into<ScopedView<'view, Message, Theme>>,
{
    fn view(&self, state: &'view String) -> ScopedView<'view, Message, Theme> {
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

pub fn render_default<State, Message, Theme>(
    state: &State,
    view: impl for<'view> ViewFn<'view, State, Message, Theme>,
) -> i64 {
    view.view(state).value
}

pub fn render_exact<Message, Theme>(
    state: &String,
    message: Message,
    theme: Theme,
    view: impl for<'view> ExactViewFn<'view, Message, Theme>,
) -> i64 {
    let _ = (message, theme);
    view.view(state).value
}

pub trait Application: Send {
    fn title(&self) -> String;
}

pub fn launch<Render>(application: Box<dyn Application>, render: Render) -> i64
where
    Render: for<'view> Fn(&'view String) -> BorrowedLabel<'view> + Send + Sync + 'static,
{
    let title = application.title();
    render(&title).value.len() as i64
}

pub fn run_application<State, Boot, Update, View, Title>(
    boot: Boot,
    update: Update,
    view: View,
    title: Title,
) -> i64
where
    Boot: Fn() -> State + Send + 'static,
    Update: Fn(&mut State, i64) + Send + Sync + 'static,
    View: for<'view> Fn(&'view State) -> BorrowedLabel<'view> + Send + Sync + 'static,
    Title: Fn(&State) -> String + Send + Sync + 'static,
{
    let mut state = boot();
    update(&mut state, 1);
    view(&state).value.len() as i64 + title(&state).len() as i64
}
