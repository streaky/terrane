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
// Namespace: descriptor-reflection
fn main() {
    let descriptor_terrane_f0_s50: TerraneDescriptor;
    descriptor_terrane_f0_s50 = TerraneDescriptor {
        identity: "int",
        name: "int",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&descriptor_terrane_f0_s50.name
        .to_owned()), terrane_scalar_support::scalar_text(&descriptor_terrane_f0_s50.kind
        .to_owned()), terrane_scalar_support::scalar_text(&descriptor_terrane_f0_s50
        .identity.to_owned())
    );
}
