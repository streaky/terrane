// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: optional-field-presence
#[derive(Clone)]
pub struct PlainBox {
    pub value: Option<String>,
}
impl PlainBox {
    pub fn terrane_construct() -> Self {
        Self {
            value: Some(String::from("kept")),
        }
    }
}
#[derive(Clone)]
pub struct BaseBoxStorage {
    pub value: Option<String>,
}
impl BaseBoxStorage {
    pub fn terrane_construct() -> Self {
        Self {
            value: Some(String::from("kept")),
        }
    }
}
#[derive(Clone)]
pub enum BaseBox {
    Own(BaseBoxStorage),
    DerivedBox(DerivedBox),
}
impl BaseBox {
    pub fn terrane_construct() -> Self {
        Self::Own(BaseBoxStorage::terrane_construct())
    }
    pub fn terrane_field_value(&self) -> &Option<String> {
        match self {
            Self::Own(value) => &value.value,
            Self::DerivedBox(value) => &value.value,
        }
    }
    pub fn terrane_field_value_mut(&mut self) -> &mut Option<String> {
        match self {
            Self::Own(value) => &mut value.value,
            Self::DerivedBox(value) => &mut value.value,
        }
    }
}
#[derive(Clone)]
pub struct DerivedBox {
    pub value: Option<String>,
    pub extra: terrane_int_support::Int,
}
impl DerivedBox {
    pub fn terrane_construct() -> Self {
        Self {
            value: Some(String::from("kept")),
            extra: terrane_int_support::Int::from(1_i128),
        }
    }
}
#[derive(Clone)]
pub struct RequiredBox {
    pub value: Option<String>,
}
impl RequiredBox {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = Some(String::from("required"));
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct OuterBox {
    pub inner: BaseBox,
}
impl OuterBox {
    pub fn terrane_construct() -> Self {
        Self {
            inner: BaseBox::terrane_construct(),
        }
    }
}
fn make_box() -> PlainBox {
    println!("{}", String::from("called"));
    return PlainBox::terrane_construct();
}
fn inspect(value: &BaseBox) {
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.terrane_field_value()
        .is_some()), terrane_scalar_support::scalar_text(&value.terrane_field_value()
        .is_none())
    );
}
fn inspect_shared(value: std::sync::Arc<TerraneSharedCell<BaseBox>>) {
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_0 = &value;
        __terrane_owner_0.lock().expect("shared reference lock poisoned")
        .terrane_field_value().is_some() }), terrane_scalar_support::scalar_text(&{ let
        __terrane_owner_1 = &value; __terrane_owner_1.lock()
        .expect("shared reference lock poisoned").terrane_field_value().is_none() })
    );
}
fn main() {
    let mut plain: PlainBox;
    let base: BaseBox;
    let derived: BaseBox;
    let shared_owner: std::sync::Arc<TerraneSharedCell<BaseBox>>;
    let shared: std::sync::Arc<TerraneSharedCell<BaseBox>>;
    let observer: std::sync::Weak<TerraneSharedCell<BaseBox>>;
    let mut required: RequiredBox;
    let outer: std::sync::Arc<TerraneSharedCell<OuterBox>>;
    plain = PlainBox::terrane_construct();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(&plain.value).is_some()),
        terrane_scalar_support::scalar_text(&(&plain.value).is_none())
    );
    {
        let __terrane_value = None;
        plain.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&(&plain.value).is_none()));
    base = BaseBox::terrane_construct();
    inspect(&base);
    derived = BaseBox::DerivedBox(DerivedBox::terrane_construct());
    inspect(&derived);
    shared_owner = std::sync::Arc::new(
        TerraneSharedCell::new(BaseBox::DerivedBox(DerivedBox::terrane_construct())),
    );
    shared = shared_owner.clone();
    inspect_shared(shared.clone());
    observer = std::sync::Arc::downgrade(&shared_owner.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_2 =
        (&observer).upgrade().expect("reference expired"); __terrane_owner_2.lock()
        .expect("reference lock poisoned").terrane_field_value().is_some() }),
        terrane_scalar_support::scalar_text(&{ let __terrane_owner_3 = (&observer)
        .upgrade().expect("reference expired"); __terrane_owner_3.lock()
        .expect("reference lock poisoned").terrane_field_value().is_none() })
    );
    {
        let __terrane_owner_4 = shared.clone();
        let __terrane_value = None;
        *__terrane_owner_4
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_value_mut() = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_5 = &shared;
        __terrane_owner_5.lock().expect("shared reference lock poisoned")
        .terrane_field_value().is_none() })
    );
    required = RequiredBox::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&(&required.value).is_some()));
    {
        let __terrane_value = None;
        required.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&(&required.value).is_none()));
    outer = std::sync::Arc::new(TerraneSharedCell::new(OuterBox::terrane_construct()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_6 = &outer;
        __terrane_owner_6.lock().expect("shared reference lock poisoned").inner
        .terrane_field_value().is_some() })
    );
    {
        let __terrane_owner_7 = outer.clone();
        let __terrane_value = None;
        *__terrane_owner_7
            .lock()
            .expect("shared reference lock poisoned")
            .inner
            .terrane_field_value_mut() = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_8 = &outer;
        __terrane_owner_8.lock().expect("shared reference lock poisoned").inner
        .terrane_field_value().is_none() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_9 =
        &make_box(); (&__terrane_receiver_value_9.value).is_some() })
    );
}
