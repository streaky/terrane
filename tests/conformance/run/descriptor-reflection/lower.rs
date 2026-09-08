// Generated deterministically by Terrane <version>.
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
