// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: object-contracts-reuse
pub trait DescribableProtocol {
    fn clone_box(&self) -> Box<dyn DescribableProtocol>;
    fn describe(&self, prefix: String) -> String;
}
impl Clone for Box<dyn DescribableProtocol> { fn clone(&self) -> Self { self.clone_box() } }
#[derive(Clone)]
pub struct Describable(Box<dyn DescribableProtocol>);
impl Describable {
    pub fn describe(&self, prefix: String) -> String {
        self.0.describe(prefix)
    }
}
#[derive(Clone)]
pub struct BaseOwn {
    pub value: terrane_int_support::Int,
}
impl BaseOwn {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn describe(&self, prefix: String) -> String {
        let _ = &self;
        return format!("{}{}", terrane_scalar_support::scalar_text(&(prefix)), terrane_scalar_support::scalar_text(&(String::from("base"))));
    }
}
#[derive(Clone)]
pub enum Base {
    Own(BaseOwn),
    Child(Child),
}
impl Base {
    pub fn terrane_construct() -> Self { Self::Own(BaseOwn::terrane_construct()) }
    pub fn describe(&self, prefix: String) -> String {
        match self {
            Self::Own(value) => value.describe(prefix),
            Self::Child(value) => value.describe(prefix),
        }
    }
}
#[derive(Clone)]
pub struct Child {
    pub value: terrane_int_support::Int,
    pub tag: String,
    pub extra: terrane_int_support::Int,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            tag: String::from("!"),
            extra: terrane_int_support::Int::from(2_i128),
        }
    }
    pub fn describe(&self, prefix: String) -> String {
        let _ = &self;
        return format!("{}{}", terrane_scalar_support::scalar_text(&(prefix)), terrane_scalar_support::scalar_text(&(String::from("child"))));
    }
    pub fn tagged_value(&self) -> String {
        let _ = &self;
        return (self.tag).clone();
    }
}
impl DescribableProtocol for Child {
    fn clone_box(&self) -> Box<dyn DescribableProtocol> { Box::new(self.clone()) }
    fn describe(&self, prefix: String) -> String {
        Child::describe(self, prefix)
    }
}
impl From<Child> for Describable { fn from(value: Child) -> Self { Self(Box::new(value)) } }
fn main() {
    let value: Child = Child::terrane_construct();
    let view: Describable = Describable::from((value).clone());
    let base_view: Base = Base::Child((value).clone());
    println!("{}", terrane_scalar_support::scalar_text(&(view.describe(String::from("a-")))));
    println!("{}", terrane_scalar_support::scalar_text(&(base_view.describe(String::from("b-")))));
    println!("{}", terrane_scalar_support::scalar_text(&(value.tagged_value())));
    println!("{}", terrane_scalar_support::scalar_text(&(((value.value).clone() + (value.extra).clone()))));
}
