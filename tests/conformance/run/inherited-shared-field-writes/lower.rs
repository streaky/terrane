// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: inherited-shared-field-writes
#[derive(Clone)]
pub struct Item {
    pub value: terrane_int_support::Int,
}
impl Item {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub struct BaseStorage {
    pub flag: bool,
    pub count: terrane_int_support::Int,
    pub values: terrane_collection_support::List<terrane_int_support::Int>,
    pub required: Option<Item>,
}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            flag: true,
            count: terrane_int_support::Int::from(5_i128),
            values: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
            required: None,
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = Item::terrane_construct();
            self.required = Some(__terrane_value);
        }
    }
    pub fn update(&mut self, amount: terrane_int_support::Int) {
        {
            let __terrane_value = amount.clone();
            self.count = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub enum Base {
    Own(BaseStorage),
    Derived(Derived),
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self::Own(BaseStorage::terrane_construct())
    }
    pub fn update(&mut self, amount: terrane_int_support::Int) {
        match self {
            Self::Own(value) => value.update(amount),
            Self::Derived(value) => value.update(amount),
        }
    }
    pub fn terrane_field_flag(&self) -> &bool {
        match self {
            Self::Own(value) => &value.flag,
            Self::Derived(value) => &value.flag,
        }
    }
    pub fn terrane_field_flag_mut(&mut self) -> &mut bool {
        match self {
            Self::Own(value) => &mut value.flag,
            Self::Derived(value) => &mut value.flag,
        }
    }
    pub fn terrane_field_count(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.count,
            Self::Derived(value) => &value.count,
        }
    }
    pub fn terrane_field_count_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.count,
            Self::Derived(value) => &mut value.count,
        }
    }
    pub fn terrane_field_values(
        &self,
    ) -> &terrane_collection_support::List<terrane_int_support::Int> {
        match self {
            Self::Own(value) => &value.values,
            Self::Derived(value) => &value.values,
        }
    }
    pub fn terrane_field_values_mut(
        &mut self,
    ) -> &mut terrane_collection_support::List<terrane_int_support::Int> {
        match self {
            Self::Own(value) => &mut value.values,
            Self::Derived(value) => &mut value.values,
        }
    }
    pub fn terrane_field_required(&self) -> &Item {
        match self {
            Self::Own(value) => {
                value.required.as_ref().expect("required field initialized")
            }
            Self::Derived(value) => {
                value.required.as_ref().expect("required field initialized")
            }
        }
    }
    pub fn terrane_field_required_mut(&mut self) -> &mut Item {
        match self {
            Self::Own(value) => {
                value.required.as_mut().expect("required field initialized")
            }
            Self::Derived(value) => {
                value.required.as_mut().expect("required field initialized")
            }
        }
    }
    pub fn terrane_field_required_slot_mut(&mut self) -> &mut Option<Item> {
        match self {
            Self::Own(value) => &mut value.required,
            Self::Derived(value) => &mut value.required,
        }
    }
}
#[derive(Clone)]
pub struct Derived {
    pub flag: bool,
    pub count: terrane_int_support::Int,
    pub values: terrane_collection_support::List<terrane_int_support::Int>,
    pub required: Option<Item>,
    pub extra: terrane_int_support::Int,
}
impl Derived {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            flag: true,
            count: terrane_int_support::Int::from(5_i128),
            values: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
            required: None,
            extra: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = Item::terrane_construct();
            self.required = Some(__terrane_value);
        }
    }
    pub fn update(&mut self, amount: terrane_int_support::Int) {
        {
            let __terrane_value = amount.clone();
            self.count = __terrane_value;
        }
    }
}
fn assign(value: std::sync::Arc<TerraneSharedCell<Derived>>) {
    {
        let __terrane_owner = value.clone();
        let __terrane_value = terrane_int_support::Int::from(9_i128);
        __terrane_owner.lock().expect("shared reference lock poisoned").count = __terrane_value;
    }
}
fn main() {
    let plain: std::sync::Arc<TerraneSharedCell<Derived>>;
    let held: std::sync::Arc<TerraneSharedCell<Derived>>;
    let weak: std::sync::Weak<TerraneSharedCell<Derived>>;
    let other: std::sync::Arc<TerraneSharedCell<Base>>;
    let other_held: std::sync::Arc<TerraneSharedCell<Base>>;
    let other_observer: std::sync::Weak<TerraneSharedCell<Base>>;
    let next: Item;
    let mut replacement: Item;
    plain = std::sync::Arc::new(TerraneSharedCell::new(Derived::terrane_construct()));
    held = plain.clone();
    {
        let __terrane_owner = held.clone();
        let __terrane_value = false;
        __terrane_owner.lock().expect("shared reference lock poisoned").flag = __terrane_value;
    }
    {
        let __terrane_owner = held.clone();
        let __terrane_value = {
            let __terrane_snapshot = held
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        __terrane_owner.lock().expect("shared reference lock poisoned").count = __terrane_value;
    }
    {
        let __terrane_owner = held.clone();
        let __terrane_collection_argument_0 = {
            let __terrane_snapshot = held
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        };
        __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .values
            .append(__terrane_collection_argument_0);
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = {
        let __terrane_result = plain.lock().expect("reference lock poisoned").flag;
        __terrane_result }; __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").count.clone();
        __terrane_result }; __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_snapshot = { let __terrane_result = plain.lock()
        .expect("reference lock poisoned").values.clone(); __terrane_result };
        __terrane_snapshot } .length()))
    );
    assign(held.clone());
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").count.clone();
        __terrane_result }; __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
    weak = std::sync::Arc::downgrade(&plain.clone());
    {
        let __terrane_owner = weak.upgrade().expect("reference expired");
        let __terrane_value = terrane_int_support::Int::from(12_i128);
        __terrane_owner.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").count.clone();
        __terrane_result }; __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
    {
        let __terrane_owner = weak.upgrade().expect("reference expired");
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            0 /* terrane-site: case.trn:39:5-39:32 */,
        );
        let __terrane_value = {
            let __terrane_snapshot = {
                let __terrane_owner = weak.upgrade().expect("reference expired");
                __terrane_owner.lock().expect("reference lock poisoned").count.clone()
            };
            __terrane_snapshot
        };
        let _ = __terrane_raised(
            __terrane_owner
                .lock()
                .expect("reference lock poisoned")
                .values
                .set(__terrane_index, __terrane_value),
            0 /* terrane-site: case.trn:39:5-39:32 */,
        );
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = &plain;
        __terrane_raised(__terrane_owner.lock().expect("reference lock poisoned").values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:40:12-40:27 */)), 1 /* terrane-site: case.trn:40:12-40:27 */) }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_owner = &held;
        __terrane_raised(__terrane_owner.lock().expect("shared reference lock poisoned")
        .values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:40:34-40:48 */)), 2 /* terrane-site: case.trn:40:34-40:48 */) })
    );
    other = std::sync::Arc::new(TerraneSharedCell::new(Base::terrane_construct()));
    other_held = other.clone();
    {
        let __terrane_owner = other_held.clone();
        let __terrane_value = terrane_int_support::Int::from(14_i128);
        *__terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_count_mut() = __terrane_value;
    }
    other_observer = std::sync::Arc::downgrade(&other.clone());
    {
        let __terrane_owner = other_observer.upgrade().expect("reference expired");
        let __terrane_value = false;
        *__terrane_owner
            .lock()
            .expect("reference lock poisoned")
            .terrane_field_flag_mut() = __terrane_value;
    }
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = {
        let __terrane_result = other.lock().expect("reference lock poisoned")
        .terrane_field_count().clone(); __terrane_result }; __terrane_snapshot }),
        String::from(" "), terrane_scalar_support::scalar_text(&{ let __terrane_snapshot
        = other_held.lock().expect("shared reference lock poisoned")
        .terrane_field_count().clone(); __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = * other.lock().expect("reference lock poisoned")
        .terrane_field_flag(); __terrane_result }; __terrane_snapshot })
    );
    {
        let __terrane_owner = other_held.clone();
        let __terrane_collection_argument_0 = {
            let __terrane_snapshot = {
                let __terrane_owner = other_observer
                    .upgrade()
                    .expect("reference expired");
                __terrane_owner
                    .lock()
                    .expect("reference lock poisoned")
                    .terrane_field_count()
                    .clone()
            };
            __terrane_snapshot
        };
        __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_values_mut()
            .append(__terrane_collection_argument_0);
    };
    next = Item::terrane_construct();
    {
        let __terrane_owner = other_held.clone();
        let __terrane_value = next;
        *__terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_required_slot_mut() = Some(__terrane_value);
    }
    {
        let __terrane_owner = other_observer.upgrade().expect("reference expired");
        let __terrane_value = terrane_int_support::Int::from(25_i128);
        __terrane_owner
            .lock()
            .expect("reference lock poisoned")
            .terrane_field_required_mut()
            .value = __terrane_value;
    }
    {
        let __terrane_owner = other_held.clone();
        let __terrane_shared_argument_0 = {
            let __terrane_snapshot = {
                let __terrane_owner = other_observer
                    .upgrade()
                    .expect("reference expired");
                __terrane_owner
                    .lock()
                    .expect("reference lock poisoned")
                    .terrane_field_count()
                    .clone()
            };
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .update(__terrane_shared_argument_1);
        __terrane_call_result
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
        other_held.lock().expect("shared reference lock poisoned").terrane_field_count()
        .clone(); __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = other.lock().expect("reference lock poisoned")
        .terrane_field_required().clone(); __terrane_result }; __terrane_snapshot }
        .value), String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_snapshot = { let __terrane_result = other.lock()
        .expect("reference lock poisoned").terrane_field_values().clone();
        __terrane_result }; __terrane_snapshot } .length()))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = *
        other_held.lock().expect("shared reference lock poisoned").terrane_field_flag();
        __terrane_snapshot }), String::from(" "), terrane_scalar_support::scalar_text(&{
        let __terrane_snapshot = { let __terrane_owner = other_observer.upgrade()
        .expect("reference expired"); * __terrane_owner.lock()
        .expect("reference lock poisoned").terrane_field_flag() }; __terrane_snapshot })
    );
    {
        let __terrane_owner = held.clone();
        let __terrane_value = terrane_int_support::Int::from(17_i128);
        __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .required
            .as_mut()
            .expect("required field initialized")
            .value = __terrane_value;
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").required
        .as_ref().expect("required field initialized").clone(); __terrane_result };
        __terrane_snapshot } .value), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").required.as_ref()
        .expect("required field initialized").clone(); __terrane_snapshot } .value)
    );
    replacement = Item::terrane_construct();
    {
        let __terrane_value = terrane_int_support::Int::from(23_i128);
        replacement.value = __terrane_value;
    }
    {
        let __terrane_owner = weak.upgrade().expect("reference expired");
        let __terrane_value = replacement;
        __terrane_owner.lock().expect("reference lock poisoned").required = Some(
            __terrane_value,
        );
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").required
        .as_ref().expect("required field initialized").clone(); __terrane_result };
        __terrane_snapshot } .value), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").required.as_ref()
        .expect("required field initialized").clone(); __terrane_snapshot } .value)
    );
    {
        let __terrane_owner = plain.clone();
        let __terrane_value = terrane_int_support::Int::from(19_i128);
        __terrane_owner.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
}
