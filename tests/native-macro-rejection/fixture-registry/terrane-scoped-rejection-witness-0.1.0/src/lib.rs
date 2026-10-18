use std::borrow::Cow;
use std::marker::PhantomData;

pub struct State {
    label: String,
}
impl State {
    pub fn new(label: String) -> Self {
        Self { label }
    }
    pub fn get_label(&self) -> &str {
        &self.label
    }
}

pub struct Element<'a, Message = ()> {
    label: Cow<'a, str>,
    message: PhantomData<Message>,
}
pub struct Text<'a>(Cow<'a, str>);
impl<'a, M> From<Text<'a>> for Element<'a, M> {
    fn from(text: Text<'a>) -> Self {
        Self {
            label: text.0,
            message: PhantomData,
        }
    }
}

pub fn text<'a>(value: impl Into<Cow<'a, str>>) -> Text<'a> {
    Text(value.into())
}
#[macro_export]
macro_rules! text {
    ($format:literal, $value:expr) => {
        $crate::text(format!($format, $value))
    };
}

pub fn inspect<F>(state: &State, render: F) -> usize
where
    F: for<'a> Fn(&'a State) -> Element<'a, ()>,
{
    render(state).label.len()
}
