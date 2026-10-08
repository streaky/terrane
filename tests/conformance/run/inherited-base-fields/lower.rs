// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: inherited-base-fields
#[derive(Clone)]
pub struct BaseStorage {
    pub value: terrane_int_support::Int,
}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(2_i128),
        }
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
    pub extra: terrane_int_support::Int,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(2_i128),
            extra: terrane_int_support::Int::from(3_i128),
        }
    }
}
fn main() {
    let concrete_terrane_f0_s126: Child;
    let mut view_terrane_f0_s157: Base;
    concrete_terrane_f0_s126 = Child::terrane_construct();
    view_terrane_f0_s157 = Base::Child(concrete_terrane_f0_s126);
    println!(
        "{}", terrane_scalar_support::scalar_text(&view_terrane_f0_s157
        .terrane_field_value().clone())
    );
    *view_terrane_f0_s157.terrane_field_value_mut() = terrane_int_support::Int::from(
        9_i128,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&view_terrane_f0_s157
        .terrane_field_value().clone())
    );
}
