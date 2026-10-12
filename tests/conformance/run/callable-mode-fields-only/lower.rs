// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
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
    let value: Operations;
    value = Operations::terrane_construct();
    value.change.call(());
    println!("{}", String::from("changed"));
    value.finish.call(());
    println!("{}", String::from("finished"));
}
