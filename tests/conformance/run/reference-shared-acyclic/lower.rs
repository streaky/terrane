// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: reference-shared-acyclic
#[derive(Clone)]
pub struct Leaf {
    pub value: terrane_int_support::Int,
}
impl Leaf {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
fn make_leaf() -> std::sync::Arc<TerraneSharedCell<Leaf>> {
    return std::sync::Arc::new(TerraneSharedCell::new(Leaf::terrane_construct()));
}
#[derive(Clone)]
pub struct Holder {
    pub child: std::sync::Arc<TerraneSharedCell<Leaf>>,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self { child: make_leaf().clone() }
    }
}
fn main() {
    let owner: Holder;
    owner = Holder::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = owner.child
        .lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot })
    );
}
