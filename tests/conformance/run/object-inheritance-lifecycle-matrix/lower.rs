// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: object-inheritance-lifecycle-matrix
pub trait NamedProtocol {
    fn clone_box(&self) -> Box<dyn NamedProtocol>;
    fn separate_box(&self) -> Box<dyn NamedProtocol>;
    fn report(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn NamedProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Named(Box<dyn NamedProtocol>);
impl Clone for Named {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Named {
    pub fn report(&self) -> terrane_int_support::Int {
        self.0.report()
    }
    fn terrane_separate(&self) -> Self {
        Self(self.0.separate_box())
    }
}
#[derive(Clone)]
pub struct BaseStorage {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn report(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        self.value = value.clone();
    }
    pub fn destruct(&mut self) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("base-destruct"))
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
    pub fn terrane_separate(&self) -> Self {
        match self {
            Self::Own(value) => Self::Own(value.terrane_separate()),
            Self::Child(value) => Self::Child(value.terrane_separate()),
        }
    }
    pub fn report(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.report(),
            Self::Child(value) => value.report(),
        }
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        match self {
            Self::Own(value_) => value_.set(value),
            Self::Child(value_) => value_.set(value),
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
impl NamedProtocol for Base {
    fn clone_box(&self) -> Box<dyn NamedProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn NamedProtocol> {
        Box::new(self.terrane_separate())
    }
    fn report(&self) -> terrane_int_support::Int {
        Base::report(&*self)
    }
}
impl From<Base> for Named {
    fn from(value: Base) -> Self {
        Self(Box::new(value))
    }
}
impl Drop for BaseStorage {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
#[derive(Clone)]
pub struct Child {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
    pub extra: terrane_int_support::Int,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            extra: terrane_int_support::Int::from(2_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn report(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        self.value = value.clone();
    }
    pub fn destruct(&mut self) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("child-destruct"))
        );
    }
    pub fn terrane_destruct_0(&mut self) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("base-destruct"))
        );
    }
}
impl NamedProtocol for Child {
    fn clone_box(&self) -> Box<dyn NamedProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn NamedProtocol> {
        Box::new(self.terrane_separate())
    }
    fn report(&self) -> terrane_int_support::Int {
        Child::report(&*self)
    }
}
impl From<Child> for Named {
    fn from(value: Child) -> Self {
        Self(Box::new(value))
    }
}
impl Drop for Child {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
            self.terrane_destruct_0();
        }
    }
}
fn main() {
    let mut value: Child = Child::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.report()));
    value.set(terrane_int_support::Int::from(4_i128));
    println!("{}", terrane_scalar_support::scalar_text(&value.report()));
    let view: Named = <Named>::from(value.terrane_separate());
    let copied: Named = view.terrane_separate();
    println!("{}", terrane_scalar_support::scalar_text(&copied.report()));
}
