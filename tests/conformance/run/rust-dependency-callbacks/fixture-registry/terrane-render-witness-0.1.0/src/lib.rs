/// A sendable rendering contract used to prove projected interface metadata.
pub trait Renderable: Send {
    /// Render one owned label.
    fn render(&self, label: String) -> String;

    /// Decorate a rendered label through the default implementation.
    fn decorated(&self, label: String) -> String {
        format!("[{}]", self.render(label))
    }

    /// Exercise a provided borrowed parameter without requiring implementors to bridge it.
    fn borrowed(&self, label: &str) -> String {
        self.render(label.to_owned())
    }

    /// Exercise provided Result metadata without requiring a foreign implementation conversion.
    fn parsed(&self, text: String) -> Result<i64, std::num::ParseIntError> {
        text.parse()
    }

    /// A common provided method shape that is omitted from the Terrane interface.
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub fn render_value<T: Renderable>(value: &T, label: String) -> String {
    value.render(label)
}

pub fn render_decorated<T: Renderable>(value: &T, label: String) -> String {
    value.decorated(label)
}

pub fn render_impl(value: &impl Renderable, label: String) -> String {
    value.render(label)
}

pub fn render_many<T: Renderable>(values: Vec<T>) -> usize {
    values.len()
}

pub trait Named {
    fn name_len(&self, name: &str) -> i64;
}

pub trait Parser {
    fn parse(&self, text: String) -> Result<i64, std::num::ParseIntError>;
}

pub trait StaticFactory {
    fn make() -> i64;
}

pub trait AssociatedValue {
    type Value;
    fn value(&self) -> Self::Value;
}

pub trait GenericMethod {
    fn map<T>(&self, value: T) -> T;
}

pub trait HigherRanked {
    fn visit(&self, visitor: for<'a> fn(&'a str));
}

pub trait UnprojectableMember {
    fn raw(&self) -> *const u8;
}
