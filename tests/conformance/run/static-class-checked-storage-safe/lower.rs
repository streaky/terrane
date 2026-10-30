// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-class-checked-storage-safe
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone()
                + terrane_int_support::Int::from(1_i128);
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
}
pub static TERRANE_STATIC_HOLDER_FIRST: std::sync::LazyLock<
    TerraneStaticObject<Counter>,
> = std::sync::LazyLock::new(|| TerraneStaticObject::new(Counter::terrane_construct()));
pub static TERRANE_STATIC_HOLDER_SECOND: std::sync::LazyLock<
    TerraneStaticObject<Counter>,
> = std::sync::LazyLock::new(|| TerraneStaticObject::new(Counter::terrane_construct()));
pub static TERRANE_STATIC_HOLDER_SCALAR: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(7_i128),
));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_update() -> terrane_int_support::Int {
        let first_value: terrane_int_support::Int;
        let second_value: terrane_int_support::Int;
        first_value = {
            let __terrane_owner_1 = TERRANE_STATIC_HOLDER_FIRST.handle();
            let __terrane_call_result = __terrane_owner_1
                .lock()
                .expect("static field lock poisoned")
                .increment();
            __terrane_call_result
        };
        second_value = {
            let __terrane_owner_3 = TERRANE_STATIC_HOLDER_SECOND.handle();
            let __terrane_call_result = __terrane_owner_3
                .lock()
                .expect("static field lock poisoned")
                .increment();
            __terrane_call_result
        };
        {
            let __terrane_owner_4 = TERRANE_STATIC_HOLDER_FIRST.handle();
            let __terrane_value = first_value.clone() + second_value.clone()
                + terrane_int_support::Int::from(1_i128);
            __terrane_owner_4.lock().expect("static field lock poisoned").value = __terrane_value;
        }
        return {
            let __terrane_owner_6 = TERRANE_STATIC_HOLDER_FIRST.handle();
            let __terrane_call_result = __terrane_owner_6
                .lock()
                .expect("static field lock poisoned")
                .read();
            __terrane_call_result
        }
            + {
                let __terrane_owner_8 = TERRANE_STATIC_HOLDER_SECOND.handle();
                let __terrane_call_result = __terrane_owner_8
                    .lock()
                    .expect("static field lock poisoned")
                    .read();
                __terrane_call_result
            };
    }
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&Holder::terrane_static_update())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = TERRANE_STATIC_HOLDER_FIRST.handle().lock()
        .expect("static field lock poisoned").value.clone(); __terrane_result };
        __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = TERRANE_STATIC_HOLDER_SECOND.handle().lock()
        .expect("static field lock poisoned").value.clone(); __terrane_result };
        __terrane_snapshot })
    );
    println!(
        "{}{}", { let __terrane_result = terrane_scalar_support::scalar_text(&*
        TERRANE_STATIC_HOLDER_SCALAR.lock().expect("static field lock poisoned"));
        __terrane_result }, { let __terrane_result =
        terrane_scalar_support::scalar_text(&* TERRANE_STATIC_HOLDER_SCALAR.lock()
        .expect("static field lock poisoned")); __terrane_result }
    );
}
