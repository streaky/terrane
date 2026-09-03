// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: object-contracts-reuse
pub trait DescribableProtocol {
    fn clone_box(&self) -> Box<dyn DescribableProtocol>;
    fn separate_box(&self) -> Box<dyn DescribableProtocol>;
    fn describe(&self, prefix: String) -> String;
}
impl Clone for Box<dyn DescribableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct Describable(Box<dyn DescribableProtocol>);
impl Describable {
    pub fn describe(&self, prefix: String) -> String {
        self.0.describe(prefix)
    }
}
#[derive(Clone)]
pub struct BaseStorage {
    pub value: terrane_int_support::Int,
}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn describe(&self, prefix: String) -> String {
        return format!(
            "{}{}", terrane_scalar_support::scalar_text(&prefix),
            terrane_scalar_support::scalar_text(&String::from("base"))
        );
    }
}
#[derive(Clone)]
pub enum Base {
    Own(BaseStorage),
    Child(Child),
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self::Own(BaseStorage::terrane_construct())
    }
    pub fn describe(&self, prefix: String) -> String {
        match self {
            Self::Own(value) => value.describe(prefix),
            Self::Child(value) => value.describe(prefix),
        }
    }
    pub fn terrane_field_value(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.value,
            Self::Child(value) => &value.value,
        }
    }
    pub fn terrane_field_value_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.value,
            Self::Child(value) => &mut value.value,
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
        return format!(
            "{}{}", terrane_scalar_support::scalar_text(&prefix),
            terrane_scalar_support::scalar_text(&String::from("child"))
        );
    }
    pub fn tagged_value(&self) -> String {
        return self.tag.clone();
    }
}
impl DescribableProtocol for Child {
    fn clone_box(&self) -> Box<dyn DescribableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DescribableProtocol> {
        Box::new(self.clone())
    }
    fn describe(&self, prefix: String) -> String {
        Child::describe(self, prefix)
    }
}
impl From<Child> for Describable {
    fn from(value: Child) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let value: Child = Child::terrane_construct();
    let view: Describable = Describable::from(value.clone());
    let base_view: Base = Base::Child(value.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&view.describe(String::from("a-")))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&base_view
        .describe(String::from("b-")))
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.tagged_value()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(value.value.clone() + value.extra
        .clone()))
    );
}
