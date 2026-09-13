// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: optional-member-post-if-return
pub static TERRANE_STATIC_HOLDER_VALUE: std::sync::LazyLock<
    std::sync::Mutex<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_initialized() -> terrane_int_support::Int {
        if TERRANE_STATIC_HOLDER_VALUE
            .lock()
            .expect("static field lock poisoned")
            .clone()
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
        return TERRANE_STATIC_HOLDER_VALUE
            .lock()
            .expect("static field lock poisoned")
            .clone()
            .expect("semantic optional narrowing");
    }
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&Holder::terrane_static_initialized())
    );
}
