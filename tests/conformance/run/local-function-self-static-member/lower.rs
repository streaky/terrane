// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: local-function-self-static-member
pub static TERRANE_STATIC_COUNTER_TOTAL: std::sync::LazyLock<
    std::sync::Mutex<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct Counter {}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn increment(&self) -> terrane_int_support::Int {
        pub fn read() -> terrane_int_support::Int {
            return TERRANE_STATIC_COUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone();
        }
        {
            let __terrane_static_value = TERRANE_STATIC_COUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone() + terrane_int_support::Int::from(1_i128);
            *TERRANE_STATIC_COUNTER_TOTAL.lock().expect("static field lock poisoned") = __terrane_static_value;
        }
        return read();
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return TERRANE_STATIC_COUNTER_TOTAL
            .lock()
            .expect("static field lock poisoned")
            .clone();
    }
}
pub static TERRANE_STATIC_STATICCOUNTER_TOTAL: std::sync::LazyLock<
    std::sync::Mutex<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    terrane_int_support::Int::from(7_i128),
));
#[derive(Clone)]
pub struct StaticCounter {}
impl StaticCounter {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn nested(&self) -> terrane_int_support::Int {
        return TERRANE_STATIC_STATICCOUNTER_TOTAL
            .lock()
            .expect("static field lock poisoned")
            .clone();
    }
    pub fn terrane_static_read() -> terrane_int_support::Int {
        pub fn nested() -> terrane_int_support::Int {
            return TERRANE_STATIC_STATICCOUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone();
        }
        return nested();
    }
}
pub static TERRANE_STATIC_INHERITEDCOUNTER_TOTAL: std::sync::LazyLock<
    std::sync::Mutex<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    terrane_int_support::Int::from(2_i128),
));
#[derive(Clone)]
pub struct InheritedCounterStorage {}
impl InheritedCounterStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn nested(&self) -> terrane_int_support::Int {
        return TERRANE_STATIC_INHERITEDCOUNTER_TOTAL
            .lock()
            .expect("static field lock poisoned")
            .clone();
    }
}
#[derive(Clone)]
pub enum InheritedCounter {
    Own(InheritedCounterStorage),
    DerivedCounter(DerivedCounter),
}
impl InheritedCounter {
    pub fn terrane_construct() -> Self {
        Self::Own(InheritedCounterStorage::terrane_construct())
    }
    pub fn terrane_static_read() -> terrane_int_support::Int {
        pub fn nested() -> terrane_int_support::Int {
            return TERRANE_STATIC_INHERITEDCOUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone();
        }
        return nested();
    }
    pub fn nested(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.nested(),
            Self::DerivedCounter(value) => value.nested(),
        }
    }
}
pub static TERRANE_STATIC_DERIVEDCOUNTER_TOTAL: std::sync::LazyLock<
    std::sync::Mutex<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    terrane_int_support::Int::from(2_i128),
));
#[derive(Clone)]
pub struct DerivedCounter {}
impl DerivedCounter {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn nested(&self) -> terrane_int_support::Int {
        return TERRANE_STATIC_DERIVEDCOUNTER_TOTAL
            .lock()
            .expect("static field lock poisoned")
            .clone();
    }
    pub fn terrane_static_read() -> terrane_int_support::Int {
        pub fn nested() -> terrane_int_support::Int {
            return TERRANE_STATIC_DERIVEDCOUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone();
        }
        return nested();
    }
}
fn main() {
    let item: Counter;
    item = Counter::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&item.increment()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&StaticCounter::terrane_static_read())
    );
    {
        let __terrane_static_value = terrane_int_support::Int::from(9_i128);
        *TERRANE_STATIC_DERIVEDCOUNTER_TOTAL
            .lock()
            .expect("static field lock poisoned") = __terrane_static_value;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&InheritedCounter::terrane_static_read())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&DerivedCounter::terrane_static_read())
    );
}
