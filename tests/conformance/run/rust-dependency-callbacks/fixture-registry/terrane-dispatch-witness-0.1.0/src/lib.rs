pub fn dispatch<F>(label: String, enabled: bool, callback: F) -> String
where
    F: Fn(String, bool) -> String + Send + Sync + 'static,
{
    callback(label, enabled)
}

pub trait Renderable {
    fn render(&self, label: String) -> String;

    fn decorated(&self, label: String) -> String {
        format!("[{}]", self.render(label))
    }
}

pub fn render_value<T: Renderable>(value: &T, label: String) -> String {
    value.render(label)
}

pub fn render_decorated<T: Renderable>(value: &T, label: String) -> String {
    value.decorated(label)
}
