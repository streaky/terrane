// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: unsafe-class-dispatch
#[derive(Clone)]
pub struct BaseStorage {}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(8_i128);
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
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => unsafe { value._terrane_unsafe_726177() }
            Self::Child(value) => unsafe { value._terrane_unsafe_726177() }
        }
    }
}
#[derive(Clone)]
pub struct Child {}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(8_i128);
    }
}
#[allow(unsafe_code)]
fn main() {
    let value: Base;
    value = Base::Child(Child::terrane_construct());
    println!(
        "{}", terrane_scalar_support::scalar_text(&unsafe { value
        ._terrane_unsafe_726177() })
    );
}
