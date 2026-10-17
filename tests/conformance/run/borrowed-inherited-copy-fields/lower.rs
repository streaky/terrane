// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: borrowed-inherited-copy-fields
#[derive(Clone)]
pub struct BaseBoxStorage {
    pub flag: bool,
    pub count: i32,
    pub ratio: f64,
}
impl BaseBoxStorage {
    pub fn terrane_construct() -> Self {
        Self {
            flag: true,
            count: 3,
            ratio: 1.5,
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
    pub fn terrane_field_flag(&self) -> &bool {
        match self {
            Self::Own(value) => &value.flag,
            Self::DerivedBox(value) => &value.flag,
        }
    }
    pub fn terrane_field_flag_mut(&mut self) -> &mut bool {
        match self {
            Self::Own(value) => &mut value.flag,
            Self::DerivedBox(value) => &mut value.flag,
        }
    }
    pub fn terrane_field_count(&self) -> &i32 {
        match self {
            Self::Own(value) => &value.count,
            Self::DerivedBox(value) => &value.count,
        }
    }
    pub fn terrane_field_count_mut(&mut self) -> &mut i32 {
        match self {
            Self::Own(value) => &mut value.count,
            Self::DerivedBox(value) => &mut value.count,
        }
    }
    pub fn terrane_field_ratio(&self) -> &f64 {
        match self {
            Self::Own(value) => &value.ratio,
            Self::DerivedBox(value) => &value.ratio,
        }
    }
    pub fn terrane_field_ratio_mut(&mut self) -> &mut f64 {
        match self {
            Self::Own(value) => &mut value.ratio,
            Self::DerivedBox(value) => &mut value.ratio,
        }
    }
}
#[derive(Clone)]
pub struct DerivedBox {
    pub flag: bool,
    pub count: i32,
    pub ratio: f64,
    pub extra: terrane_int_support::Int,
}
impl DerivedBox {
    pub fn terrane_construct() -> Self {
        Self {
            flag: true,
            count: 3,
            ratio: 1.5,
            extra: terrane_int_support::Int::from(1_i128),
        }
    }
}
fn make_derived() -> BaseBox {
    let mut value: DerivedBox;
    value = DerivedBox::terrane_construct();
    value.flag = false;
    value.count = 9;
    value.ratio = 2.5;
    return BaseBox::DerivedBox(value);
}
fn read_flag(value: &BaseBox) -> bool {
    return *value.terrane_field_flag();
}
fn read_count(value: &BaseBox) -> i32 {
    return *value.terrane_field_count();
}
fn read_ratio(value: &BaseBox) -> f64 {
    return *value.terrane_field_ratio();
}
fn show_copy(flag: bool, count: i32, ratio: f64) {
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&flag),
        terrane_scalar_support::scalar_text(&count),
        terrane_scalar_support::scalar_text(&ratio)
    );
}
fn check_borrow(value: &BaseBox) {
    let flag: bool;
    let count: i32;
    let ratio: f64;
    println!("{}", terrane_scalar_support::scalar_text(&read_flag(value)));
    println!("{}", terrane_scalar_support::scalar_text(&read_count(value)));
    println!("{}", terrane_scalar_support::scalar_text(&read_ratio(value)));
    flag = *value.terrane_field_flag();
    count = *value.terrane_field_count();
    ratio = *value.terrane_field_ratio();
    show_copy(flag, count, ratio);
    show_copy(
        *value.terrane_field_flag(),
        *value.terrane_field_count(),
        *value.terrane_field_ratio(),
    );
}
fn check_shared(value: std::sync::Arc<std::sync::Mutex<BaseBox>>) {
    let flag: bool;
    let count: i32;
    let ratio: f64;
    flag = *value.lock().expect("shared reference lock poisoned").terrane_field_flag();
    count = *value.lock().expect("shared reference lock poisoned").terrane_field_count();
    ratio = *value.lock().expect("shared reference lock poisoned").terrane_field_ratio();
    show_copy(flag, count, ratio);
    show_copy(
        *value.lock().expect("shared reference lock poisoned").terrane_field_flag(),
        count,
        ratio,
    );
    show_copy(
        flag,
        *value.lock().expect("shared reference lock poisoned").terrane_field_count(),
        ratio,
    );
    show_copy(
        flag,
        count,
        *value.lock().expect("shared reference lock poisoned").terrane_field_ratio(),
    );
}
fn main() {
    let base: BaseBox;
    let derived: BaseBox;
    let shared_base: std::sync::Arc<std::sync::Mutex<BaseBox>>;
    let shared_derived: std::sync::Arc<std::sync::Mutex<BaseBox>>;
    base = BaseBox::terrane_construct();
    check_borrow(&base);
    derived = make_derived();
    check_borrow(&derived);
    shared_base = std::sync::Arc::new(
        std::sync::Mutex::new(BaseBox::terrane_construct()),
    );
    check_shared(shared_base.clone());
    shared_derived = std::sync::Arc::new(std::sync::Mutex::new(make_derived()));
    check_shared(shared_derived.clone());
}
