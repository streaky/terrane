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
    fields: &'static [TerraneFieldMetadata],
}
// Source: case.trn
// Namespace: descriptor-runtime-value
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&TerraneDescriptor { identity : "int8",
        name : "int8", kind : "type", fields : &[] } .name)
    );
}
