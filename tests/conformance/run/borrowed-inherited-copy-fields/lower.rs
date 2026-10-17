// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
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
    {
        let __terrane_value = false;
        value.flag = __terrane_value;
    }
    {
        let __terrane_value = 9;
        value.count = __terrane_value;
    }
    {
        let __terrane_value = 2.5;
        value.ratio = __terrane_value;
    }
    return BaseBox::DerivedBox(value);
}
fn read_flag(value: &BaseBox) -> bool {
    return {
        let __terrane_snapshot = *value.terrane_field_flag();
        __terrane_snapshot
    };
}
fn read_count(value: &BaseBox) -> i32 {
    return {
        let __terrane_snapshot = *value.terrane_field_count();
        __terrane_snapshot
    };
}
fn read_ratio(value: &BaseBox) -> f64 {
    return {
        let __terrane_snapshot = *value.terrane_field_ratio();
        __terrane_snapshot
    };
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
    flag = {
        let __terrane_snapshot = *value.terrane_field_flag();
        __terrane_snapshot
    };
    count = {
        let __terrane_snapshot = *value.terrane_field_count();
        __terrane_snapshot
    };
    ratio = {
        let __terrane_snapshot = *value.terrane_field_ratio();
        __terrane_snapshot
    };
    show_copy(flag, count, ratio);
    show_copy(
        {
            let __terrane_snapshot = *value.terrane_field_flag();
            __terrane_snapshot
        },
        {
            let __terrane_snapshot = *value.terrane_field_count();
            __terrane_snapshot
        },
        {
            let __terrane_snapshot = *value.terrane_field_ratio();
            __terrane_snapshot
        },
    );
}
fn check_shared(value: std::sync::Arc<TerraneSharedCell<BaseBox>>) {
    let flag: bool;
    let count: i32;
    let ratio: f64;
    flag = {
        let __terrane_snapshot = *value
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_flag();
        __terrane_snapshot
    };
    count = {
        let __terrane_snapshot = *value
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_count();
        __terrane_snapshot
    };
    ratio = {
        let __terrane_snapshot = *value
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_ratio();
        __terrane_snapshot
    };
    show_copy(flag, count, ratio);
    show_copy(
        {
            let __terrane_snapshot = *value
                .lock()
                .expect("shared reference lock poisoned")
                .terrane_field_flag();
            __terrane_snapshot
        },
        count,
        ratio,
    );
    show_copy(
        flag,
        {
            let __terrane_snapshot = *value
                .lock()
                .expect("shared reference lock poisoned")
                .terrane_field_count();
            __terrane_snapshot
        },
        ratio,
    );
    show_copy(
        flag,
        count,
        {
            let __terrane_snapshot = *value
                .lock()
                .expect("shared reference lock poisoned")
                .terrane_field_ratio();
            __terrane_snapshot
        },
    );
}
fn main() {
    let base: BaseBox;
    let derived: BaseBox;
    let shared_base: std::sync::Arc<TerraneSharedCell<BaseBox>>;
    let shared_derived: std::sync::Arc<TerraneSharedCell<BaseBox>>;
    base = BaseBox::terrane_construct();
    check_borrow(&base);
    derived = make_derived();
    check_borrow(&derived);
    shared_base = std::sync::Arc::new(
        TerraneSharedCell::new(BaseBox::terrane_construct()),
    );
    check_shared(shared_base.clone());
    shared_derived = std::sync::Arc::new(TerraneSharedCell::new(make_derived()));
    check_shared(shared_derived.clone());
}
