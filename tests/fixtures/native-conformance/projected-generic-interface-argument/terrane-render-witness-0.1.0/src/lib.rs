pub trait Renderable {
    fn render(&self, label: String) -> String;
}

pub fn render_value<T: Renderable>(value: &T, label: String) -> String {
    value.render(label)
}

pub fn render_many<T: Renderable>(values: Vec<T>) -> usize {
    values.len()
}
