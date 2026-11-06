pub struct Holder {
    label: String,
}
impl Holder {
    pub fn new() -> Self {
        Self { label: "hello".to_owned() }
    }
    pub fn label_len(&self) -> usize {
        self.label.len()
    }
}
pub struct Other {
    label: String,
}
impl Other {
    pub fn new() -> Self {
        Self {
            label: "different".to_owned(),
        }
    }
    pub fn other_len(&self) -> usize {
        self.label.len()
    }
}
