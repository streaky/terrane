// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: descriptor-runtime-value
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&TerraneDescriptor { identity : "int8",
        name : "int8", kind : "type", inherently_identity_bearing : false, fields : &[] }
        .name)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let _ = &1; TerraneDescriptor {
        identity : "/core/types::int", name : "int", kind : "type",
        inherently_identity_bearing : false, fields : &[] } } .name.to_owned()),
        terrane_scalar_support::scalar_text(&{ let _ = &1; TerraneDescriptor { identity :
        "/core/types::int", name : "int", kind : "type", inherently_identity_bearing :
        false, fields : &[] } } .identity.to_owned())
    );
}
