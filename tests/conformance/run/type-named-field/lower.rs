// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: type-named-field
#[derive(Clone)]
pub struct Sample {
    pub __trn_74797065: String,
}
impl Sample {
    pub fn terrane_construct() -> Self {
        Self {
            __trn_74797065: String::from("declared-field"),
        }
    }
}
fn main() {
    let value: Sample = Sample::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.__trn_74797065));
}
