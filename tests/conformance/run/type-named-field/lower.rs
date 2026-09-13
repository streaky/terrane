// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneFieldMetadata {
    name: &'static str,
    external_name: &'static str,
    defaulted: bool,
    optional: bool,
    secret: bool,
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
    inherently_identity_bearing: bool,
    fields: &'static [TerraneFieldMetadata],
}
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
