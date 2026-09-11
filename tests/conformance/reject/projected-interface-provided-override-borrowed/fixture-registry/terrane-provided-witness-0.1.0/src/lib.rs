pub trait Labelled {
    fn label(&self) -> String;

    fn shout(&self, suffix: &str) -> String {
        format!("{}{}", self.label(), suffix)
    }
}
