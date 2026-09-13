// Generated deterministically by Terrane <version>.
// Runtime support: mutable_callable.rs, consuming_callable.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: callable-mode-fields-only
fn noop() {
    return ();
}
pub struct Operations {
    pub change: TerraneMutableCallable<(), ()>,
    pub finish: TerraneConsumingCallable<(), ()>,
}
impl Operations {
    pub fn terrane_construct() -> Self {
        Self {
            change: TerraneMutableCallable::new(move |(): ()| noop()),
            finish: TerraneConsumingCallable::new(move |(): ()| noop()),
        }
    }
}
fn main() {
    let value: Operations = Operations::terrane_construct();
    value.change.call(());
    println!("{}", terrane_scalar_support::scalar_text(&String::from("changed")));
    value.finish.call(());
    println!("{}", terrane_scalar_support::scalar_text(&String::from("finished")));
}
