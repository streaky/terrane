// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct Holder {
    pub value: terrane_int_support::Int,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
}
fn main() {
    let x: Holder;
    let flag: bool;
    x = Holder::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&x.value));
    flag = true;
    println!("{}", terrane_scalar_support::scalar_text(&flag));
    println!("{}", String::from("after"));
}
