// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: inherited-self-return
#[derive(Clone)]
pub struct BaseStorage {}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn copy(&self) -> Base {
        return Base::Own(self.clone());
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
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
    pub fn copy(&self) -> Base {
        match self {
            Self::Own(value) => value.copy(),
            Self::Child(value) => value.copy(),
        }
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.marker(),
            Self::Child(value) => value.marker(),
        }
    }
}
#[derive(Clone)]
pub struct Child {}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn copy(&self) -> Base {
        return Base::Child(self.clone());
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
fn main() {
    let value: Child = Child::terrane_construct();
    let result: Base = value.copy();
    println!("{}", terrane_scalar_support::scalar_text(&result.marker()));
}
