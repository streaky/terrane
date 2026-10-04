use iced::{Element, advanced::widget::Tree};
#[derive(Clone)]
pub struct State {
    pub label: String,
}
impl State {
    pub fn new(label: String) -> Self {
        Self { label }
    }
    pub fn get_label(&self) -> &str {
        &self.label
    }
}
pub fn inspect<F>(state: &State, render: F) -> usize
where
    F: for<'a> Fn(&'a State) -> Element<'a, ()>,
{
    Tree::new(render(state).as_widget()).children.len()
}
