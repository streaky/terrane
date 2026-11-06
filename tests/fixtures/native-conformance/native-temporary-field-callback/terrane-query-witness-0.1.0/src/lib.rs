pub struct View<'a>(&'a str);
impl View<'_> {
    pub fn size(&self) -> usize {
        self.0.len()
    }
}
pub fn query(sql: &str) -> View<'_> {
    View(sql)
}
pub struct Holder {
    label: String,
}
impl Holder {
    pub fn new() -> Self {
        Self { label: "hello".to_owned() }
    }
    pub fn view(&self) -> View<'_> {
        View(&self.label)
    }
    pub fn label_len(&self) -> usize {
        self.label.len()
    }
}
pub struct Config {
    pub sql: String,
}
impl Config {
    pub fn new() -> Self {
        Self { sql: "callback".to_owned() }
    }
}
pub fn render<'a, F>(state: &'a Config, draw: F) -> usize
where
    F: FnOnce(&'a Config) -> View<'a>,
{
    draw(state).size()
}
