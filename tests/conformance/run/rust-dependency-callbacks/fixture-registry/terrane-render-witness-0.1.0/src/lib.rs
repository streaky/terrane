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
