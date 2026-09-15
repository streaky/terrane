pub trait Renderable {
    fn render(&self, label: String) -> String;
}

#[derive(Clone)]
pub struct Widget;

impl Renderable for Widget {
    fn render(&self, label: String) -> String {
        label
    }
}

pub fn widget() -> Widget {
    Widget
}

pub fn render_value<T: Renderable>(value: &T, label: String) -> String {
    value.render(label)
}

pub fn render_many<T: Renderable>(values: Vec<T>) -> usize {
    values.len()
}
