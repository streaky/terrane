// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-shared-call-statements
fn say() -> terrane_int_support::Int {
    println!("{}", terrane_scalar_support::scalar_text(&3));
    return terrane_int_support::Int::from(3_i128);
}
fn echo(value: terrane_int_support::Int) -> terrane_int_support::Int {
    println!("{}", terrane_scalar_support::scalar_text(&value));
    return value.clone();
}
pub static TERRANE_STATIC_HOLDER_F: std::sync::LazyLock<
    TerraneSharedCell<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(std::sync::Arc::new(say)));
pub static TERRANE_STATIC_HOLDER_G: std::sync::LazyLock<
    TerraneSharedCell<
        std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    >,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(std::sync::Arc::new(echo)));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_invoke() {
        ({
            let __terrane_value = TERRANE_STATIC_HOLDER_F
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        })();
        ({
            let __terrane_value = TERRANE_STATIC_HOLDER_G
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        })(terrane_int_support::Int::from(4_i128));
    }
}
fn main() {
    ({
        let __terrane_value = TERRANE_STATIC_HOLDER_F
            .lock()
            .expect("static field lock poisoned")
            .clone();
        __terrane_value
    })();
    ({
        let __terrane_value = TERRANE_STATIC_HOLDER_G
            .lock()
            .expect("static field lock poisoned")
            .clone();
        __terrane_value
    })(terrane_int_support::Int::from(4_i128));
    Holder::terrane_static_invoke();
    println!("{}", terrane_scalar_support::scalar_text(&1));
}
