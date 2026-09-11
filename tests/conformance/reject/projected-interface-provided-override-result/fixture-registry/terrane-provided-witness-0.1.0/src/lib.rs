pub trait Parser {
    fn label(&self) -> String;

    fn parsed(&self, text: String) -> Result<i64, std::num::ParseIntError> {
        text.parse()
    }
}
