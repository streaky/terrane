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
// Namespace: descriptor-constructs
fn main() {
    let value: i64 = 1;
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &value; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = value; true }));
}
