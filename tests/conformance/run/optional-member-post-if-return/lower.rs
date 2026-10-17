// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: optional-member-post-if-return
pub static TERRANE_STATIC_HOLDER_VALUE: std::sync::LazyLock<
    TerraneSharedCell<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(None));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_initialized() -> terrane_int_support::Int {
        if {
            let __terrane_value = TERRANE_STATIC_HOLDER_VALUE
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        }
            .is_none()
        {
            {
                let __terrane_static_value = Some(
                    terrane_int_support::Int::from(7_i128),
                );
                *TERRANE_STATIC_HOLDER_VALUE
                    .lock()
                    .expect("static field lock poisoned") = __terrane_static_value;
            }
        }
        return {
            let __terrane_value = TERRANE_STATIC_HOLDER_VALUE
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        }
            .as_ref()
            .expect("semantic optional narrowing")
            .clone();
    }
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&Holder::terrane_static_initialized())
    );
}
