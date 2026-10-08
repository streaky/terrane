// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
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
fn make_leaf() -> std::sync::Arc<std::sync::Mutex<Leaf>> {
    return std::sync::Arc::new(std::sync::Mutex::new(Leaf::terrane_construct()));
}
#[derive(Clone)]
pub struct Holder {
    pub child: std::sync::Arc<std::sync::Mutex<Leaf>>,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self { child: make_leaf().clone() }
    }
}
fn main() {
    let owner_terrane_f0_s206: Holder;
    owner_terrane_f0_s206 = Holder::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&owner_terrane_f0_s206.child.lock()
        .expect("shared reference lock poisoned").value.clone())
    );
}
