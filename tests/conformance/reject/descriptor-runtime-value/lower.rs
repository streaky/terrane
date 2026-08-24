// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor { identity: &'static str, name: &'static str, kind: &'static str }
// Source: case.trn
// Namespace: descriptor-runtime-value
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&((TerraneDescriptor { identity: "int8", name: "int8", kind: "type" }).name)));
}
