// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: reentry-recursive-helper
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
fn left(
    first: std::sync::Arc<TerraneSharedCell<Base>>,
    second: std::sync::Arc<TerraneSharedCell<Base>>,
    depth: terrane_int_support::Int,
) -> terrane_int_support::Int {
    if depth.clone() > terrane_int_support::Int::from(0_i128) {
        return right(
            second.clone(),
            first.clone(),
            depth.clone() - terrane_int_support::Int::from(1_i128),
        );
    }
    return {
        let __terrane_owner = first.clone();
        let __terrane_shared_argument_0 = second.clone();
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .peek(__terrane_shared_argument_1);
        __terrane_call_result
    };
}
fn right(
    first: std::sync::Arc<TerraneSharedCell<Base>>,
    second: std::sync::Arc<TerraneSharedCell<Base>>,
    depth: terrane_int_support::Int,
) -> terrane_int_support::Int {
    if depth.clone() > terrane_int_support::Int::from(0_i128) {
        return left(
            first.clone(),
            second.clone(),
            depth.clone() - terrane_int_support::Int::from(1_i128),
        );
    }
    return terrane_int_support::Int::from(0_i128);
}
fn main() {
    let a: std::sync::Arc<TerraneSharedCell<Base>>;
    let held: std::sync::Arc<TerraneSharedCell<Base>>;
    a = std::sync::Arc::new(TerraneSharedCell::new(Base::terrane_construct()));
    held = a.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&right(held.clone(), held.clone(),
        terrane_int_support::Int::from(0_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&right(held.clone(), held.clone(),
        terrane_int_support::Int::from(1_i128)))
    );
}
