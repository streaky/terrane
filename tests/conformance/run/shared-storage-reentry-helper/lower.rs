// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: reentry-helper
#[derive(Clone)]
pub struct Base {
    pub count: terrane_int_support::Int,
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(5_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<TerraneSharedCell<Base>>,
    ) -> terrane_int_support::Int {
        return {
            let __terrane_snapshot = other
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        };
    }
}
fn go(
    x: std::sync::Arc<TerraneSharedCell<Base>>,
    y: std::sync::Arc<TerraneSharedCell<Base>>,
) -> terrane_int_support::Int {
    return {
        let __terrane_owner_1 = x.clone();
        let __terrane_shared_argument_0 = y.clone();
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner_1
            .lock()
            .expect("shared reference lock poisoned")
            .peek(__terrane_shared_argument_1);
        __terrane_call_result
    };
}
fn main() {
    let a: std::sync::Arc<TerraneSharedCell<Base>>;
    let ha: std::sync::Arc<TerraneSharedCell<Base>>;
    a = std::sync::Arc::new(TerraneSharedCell::new(Base::terrane_construct()));
    ha = a.clone();
    println!("{}", terrane_scalar_support::scalar_text(&go(ha.clone(), ha.clone())));
}
