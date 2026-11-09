// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-callable-field-snapshots
fn five() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(5_i128);
}
fn counter() -> TerraneMutableCallable<(), terrane_int_support::Int> {
    let n: terrane_int_support::Int;
    n = terrane_int_support::Int::from(0_i128);
    return {
        let mut n = n.clone();
        TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
            n = n.clone() + terrane_int_support::Int::from(1_i128);
            return n.clone();
        })
    };
}
pub static TERRANE_STATIC_CALLBACKS_OBSERVE: std::sync::LazyLock<
    TerraneSharedCell<std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(std::sync::Arc::new(five)));
pub static TERRANE_STATIC_CALLBACKS_ADVANCE: std::sync::LazyLock<
    TerraneSharedCell<TerraneMutableCallable<(), terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(counter()));
#[derive(Clone)]
pub struct Callbacks {}
impl Callbacks {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_control() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(5_i128);
    }
}
fn main() {
    let local: TerraneMutableCallable<(), terrane_int_support::Int>;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value =
        TERRANE_STATIC_CALLBACKS_OBSERVE.lock().expect("static field lock poisoned")
        .clone(); __terrane_value } ())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value =
        TERRANE_STATIC_CALLBACKS_ADVANCE.lock().expect("static field lock poisoned")
        .clone(); __terrane_value } .call(()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value =
        TERRANE_STATIC_CALLBACKS_ADVANCE.lock().expect("static field lock poisoned")
        .clone(); __terrane_value } .call(()))
    );
    local = {
        let __terrane_value = TERRANE_STATIC_CALLBACKS_ADVANCE
            .lock()
            .expect("static field lock poisoned")
            .clone();
        __terrane_value
    }
        .clone();
    println!("{}", terrane_scalar_support::scalar_text(&local.call(())));
    println!("{}", terrane_scalar_support::scalar_text(&local.call(())));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value =
        TERRANE_STATIC_CALLBACKS_ADVANCE.lock().expect("static field lock poisoned")
        .clone(); __terrane_value } .call(()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&Callbacks::terrane_static_control())
    );
}
