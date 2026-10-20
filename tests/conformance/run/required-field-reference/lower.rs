// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: required-field-reference
#[derive(Clone)]
pub struct Holder {
    pub value: terrane_int_support::Int,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(4_i128),
        }
    }
    pub fn peek(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct OuterStorage {
    pub inner: Option<Holder>,
}
impl OuterStorage {
    pub fn terrane_construct(inner: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct(inner);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder) {
        {
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
    }
    pub fn peek(&self) -> terrane_int_support::Int {
        let alias: &Holder;
        alias = self.inner.as_ref().expect("required field initialized");
        return alias.clone().peek();
    }
}
#[derive(Clone)]
pub enum Outer {
    Own(OuterStorage),
    DerivedOuter(DerivedOuter),
}
impl Outer {
    pub fn terrane_construct(inner: Holder) -> Self {
        Self::Own(OuterStorage::terrane_construct(inner))
    }
    pub fn peek(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.peek(),
            Self::DerivedOuter(value) => value.peek(),
        }
    }
    pub fn terrane_field_inner(&self) -> &Holder {
        match self {
            Self::Own(value) => value.inner.as_ref().expect("required field initialized"),
            Self::DerivedOuter(value) => {
                value.inner.as_ref().expect("required field initialized")
            }
        }
    }
    pub fn terrane_field_inner_mut(&mut self) -> &mut Holder {
        match self {
            Self::Own(value) => value.inner.as_mut().expect("required field initialized"),
            Self::DerivedOuter(value) => {
                value.inner.as_mut().expect("required field initialized")
            }
        }
    }
    pub fn terrane_field_inner_slot_mut(&mut self) -> &mut Option<Holder> {
        match self {
            Self::Own(value) => &mut value.inner,
            Self::DerivedOuter(value) => &mut value.inner,
        }
    }
}
#[derive(Clone)]
pub struct DerivedOuter {
    pub inner: Option<Holder>,
}
impl DerivedOuter {
    pub fn terrane_construct(inner: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct(inner);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder) {
        {
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
    }
    pub fn peek(&self) -> terrane_int_support::Int {
        let alias: &Holder;
        alias = self.inner.as_ref().expect("required field initialized");
        return alias.clone().peek();
    }
}
fn main() {
    let o: Outer;
    let alias: &Holder;
    let d: DerivedOuter;
    let derived_alias: &Holder;
    o = Outer::terrane_construct(Holder::terrane_construct());
    alias = o.terrane_field_inner();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&alias.clone().peek()),
        terrane_scalar_support::scalar_text(&o.peek())
    );
    d = DerivedOuter::terrane_construct(Holder::terrane_construct());
    derived_alias = d.inner.as_ref().expect("required field initialized");
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&derived_alias.clone().peek()),
        terrane_scalar_support::scalar_text(&d.peek())
    );
}
