// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: descriptor-reflection
fn main() {
    let descriptor: TerraneDescriptor = TerraneDescriptor {
        identity: "int",
        name: "int",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&descriptor.name.to_owned()),
        terrane_scalar_support::scalar_text(&descriptor.kind.to_owned()),
        terrane_scalar_support::scalar_text(&descriptor.identity.to_owned())
    );
}
