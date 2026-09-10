// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: builtin-name-class
#[derive(Clone)]
pub struct List {
    pub value: terrane_int_support::Int,
}
impl List {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
fn main() {
    let item: List = List::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&item.value));
}
