// Generated deterministically by Terrane <version>.
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Inner {
    pub value: terrane_int_support::Int,
}
impl Inner {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAppBoxIdentity2F61707000626F78Storage {
    pub scalar: terrane_int_support::Int,
    pub item: Inner,
}
impl TerraneNsAppBoxIdentity2F61707000626F78Storage {
    pub fn terrane_construct() -> Self {
        Self {
            scalar: terrane_int_support::Int::from(0_i128),
            item: Inner::terrane_construct(),
        }
    }
}
#[derive(Clone)]
pub enum TerraneNsAppBoxIdentity2F61707000626F78 {
    Own(TerraneNsAppBoxIdentity2F61707000626F78Storage),
    DerivedBox(DerivedBox),
}
impl TerraneNsAppBoxIdentity2F61707000626F78 {
    pub fn terrane_construct() -> Self {
        Self::Own(TerraneNsAppBoxIdentity2F61707000626F78Storage::terrane_construct())
    }
    pub fn terrane_field_scalar(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.scalar,
            Self::DerivedBox(value) => &value.scalar,
        }
    }
    pub fn terrane_field_scalar_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.scalar,
            Self::DerivedBox(value) => &mut value.scalar,
        }
    }
    pub fn terrane_field_item(&self) -> &Inner {
        match self {
            Self::Own(value) => &value.item,
            Self::DerivedBox(value) => &value.item,
        }
    }
    pub fn terrane_field_item_mut(&mut self) -> &mut Inner {
        match self {
            Self::Own(value) => &mut value.item,
            Self::DerivedBox(value) => &mut value.item,
        }
    }
}
#[derive(Clone)]
pub struct DerivedBox {
    pub scalar: terrane_int_support::Int,
    pub item: Inner,
    pub extra: terrane_int_support::Int,
}
impl DerivedBox {
    pub fn terrane_construct() -> Self {
        Self {
            scalar: terrane_int_support::Int::from(0_i128),
            item: Inner::terrane_construct(),
            extra: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub struct RequiredBox {
    pub item: Option<Inner>,
}
impl RequiredBox {
    pub fn terrane_construct(item: Inner) -> Self {
        let mut __terrane_constructed_value = Self { item: None };
        __terrane_constructed_value.construct(item);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, item: Inner) {
        {
            let __terrane_value = item;
            self.item = Some(__terrane_value);
        }
    }
}
fn scalar_field<'a>(
    holder: &'a mut TerraneNsAppBoxIdentity2F61707000626F78,
) -> &'a mut terrane_int_support::Int {
    return holder.terrane_field_scalar_mut();
}
fn object_field<'a>(
    holder: &'a mut TerraneNsAppBoxIdentity2F61707000626F78,
) -> &'a mut Inner {
    return holder.terrane_field_item_mut();
}
fn inherited_field<'a>(holder: &'a mut DerivedBox) -> &'a mut terrane_int_support::Int {
    return &mut holder.scalar;
}
fn required_field<'a>(holder: &'a mut RequiredBox) -> &'a mut Inner {
    return holder.item.as_mut().expect("required field initialized");
}
fn nested_field<'a>(holder: &'a mut RequiredBox) -> &'a mut terrane_int_support::Int {
    return &mut holder.item.as_mut().expect("required field initialized").value;
}
fn main() {
    let mut original: TerraneNsAppBoxIdentity2F61707000626F78;
    let scalar_alias: &mut terrane_int_support::Int;
    let object_alias: &mut Inner;
    let mut derived: DerivedBox;
    let inherited_alias: &mut terrane_int_support::Int;
    let mut required: RequiredBox;
    let required_alias: &mut Inner;
    let nested_alias: &mut terrane_int_support::Int;
    original = TerraneNsAppBoxIdentity2F61707000626F78::terrane_construct();
    scalar_alias = scalar_field(&mut original);
    *scalar_alias = terrane_int_support::Int::from(21_i128);
    println!(
        "{}", terrane_scalar_support::scalar_text(&original.terrane_field_scalar()
        .clone())
    );
    object_alias = object_field(&mut original);
    {
        let __terrane_value = terrane_int_support::Int::from(34_i128);
        object_alias.value = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&original.terrane_field_item().clone()
        .value)
    );
    derived = DerivedBox::terrane_construct();
    inherited_alias = inherited_field(&mut derived);
    *inherited_alias = terrane_int_support::Int::from(55_i128);
    println!("{}", terrane_scalar_support::scalar_text(&derived.scalar));
    required = RequiredBox::terrane_construct(Inner::terrane_construct());
    required_alias = required_field(&mut required);
    {
        let __terrane_value = terrane_int_support::Int::from(89_i128);
        required_alias.value = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&required.item.as_ref()
        .expect("required field initialized").value)
    );
    nested_alias = nested_field(&mut required);
    *nested_alias = terrane_int_support::Int::from(144_i128);
    println!(
        "{}", terrane_scalar_support::scalar_text(&required.item.as_ref()
        .expect("required field initialized").value)
    );
}
