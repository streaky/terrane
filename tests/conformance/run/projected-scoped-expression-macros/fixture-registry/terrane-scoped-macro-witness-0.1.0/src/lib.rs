use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

static CLONES: AtomicUsize = AtomicUsize::new(0);
pub struct State {
    pub label: String,
    pub children: Vec<State>,
}
impl Clone for State {
    fn clone(&self) -> Self {
        CLONES.fetch_add(1, Ordering::SeqCst);
        Self {
            label: self.label.clone(),
            children: self.children.clone(),
        }
    }
}
impl State {
    pub fn new(label: String) -> Self {
        Self {
            label,
            children: Vec::new(),
        }
    }
    pub fn branch(label: String, children: Vec<State>) -> Self {
        Self { label, children }
    }
    pub fn get_label(&self) -> &str {
        &self.label
    }
}

// The message binder and intermediate Into conversions are part of the regression.
pub struct Element<'a, Message = ()> {
    children: Vec<Self>,
    label: Cow<'a, str>,
    message: PhantomData<Message>,
}
pub struct Text<'a>(Cow<'a, str>);
pub struct Group<'a, Message = ()>(Element<'a, Message>);
impl<'a, M> Group<'a, M> {
    pub fn new(label: &'static str, children: Vec<Element<'a, M>>) -> Self {
        Self(Element {
            children,
            label: Cow::Borrowed(label),
            message: PhantomData,
        })
    }
}
impl<'a, M> From<Text<'a>> for Element<'a, M> {
    fn from(text: Text<'a>) -> Self {
        Self {
            children: Vec::new(),
            label: text.0,
            message: PhantomData,
        }
    }
}
impl<'a, M> From<Group<'a, M>> for Element<'a, M> {
    fn from(group: Group<'a, M>) -> Self {
        group.0
    }
}

#[macro_export]
macro_rules! text {
    ($format:literal, $value:expr) => {
        $crate::text(format!($format, $value))
    };
}
#[macro_export]
macro_rules! row {
    ($($value:expr),* $(,)?) => {
        $crate::Group::new("row", vec![$(::core::convert::Into::into($value)),*])
    };
}
#[macro_export]
macro_rules! column {
    ($($value:expr),* $(,)?) => {
        $crate::Group::new("column", vec![$(::core::convert::Into::into($value)),*])
    };
}

pub fn text<'a>(label: impl Into<Cow<'a, str>>) -> Text<'a> {
    Text(label.into())
}
pub fn row<'a, M>(children: impl IntoIterator<Item = Element<'a, M>>) -> Group<'a, M> {
    Group::new("children", children.into_iter().collect())
}
pub fn container<'a, M>(child: impl Into<Element<'a, M>>) -> Group<'a, M> {
    Group::new("container", vec![child.into()])
}

pub fn inspect<F>(state: &State, render: F) -> String
where
    F: for<'a> Fn(&'a State) -> Element<'a, ()>,
{
    fn shape<M>(node: &Element<'_, M>, depth: usize, totals: &mut (usize, usize, usize, usize)) {
        totals.0 += 1;
        totals.1 += usize::from(!node.children.is_empty());
        totals.2 = totals.2.max(depth);
        totals.3 += node.label.len();
        for child in &node.children {
            shape(child, depth + 1, totals);
        }
    }
    let before = CLONES.load(Ordering::SeqCst);
    let node = render(state);
    let mut totals = (0, 0, 0, 0);
    shape(&node, 1, &mut totals);
    format!(
        "{}:{}:{}:{}:{}",
        totals.0,
        totals.1,
        totals.2,
        totals.3,
        CLONES.load(Ordering::SeqCst) - before
    )
}
