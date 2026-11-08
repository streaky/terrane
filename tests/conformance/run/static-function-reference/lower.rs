// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-function-reference
fn five() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(5_i128);
}
pub static TERRANE_STATIC_HOLDER_F: std::sync::LazyLock<
    TerraneSharedCell<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(std::sync::Arc::new(five)));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    let callback: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    {
        let __terrane_static_value = std::sync::Arc::new(five);
        *TERRANE_STATIC_HOLDER_F.lock().expect("static field lock poisoned") = __terrane_static_value;
    }
    callback = {
        let __terrane_value = TERRANE_STATIC_HOLDER_F
            .lock()
            .expect("static field lock poisoned")
            .clone();
        __terrane_value
    }
        .clone();
    println!("{}", terrane_scalar_support::scalar_text(&callback()));
}
