// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: composed-descriptor-identities
fn increment(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(1_i128);
}
fn main() {
    let callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    > = std::sync::Arc::new(increment);
    let optional: Option<String> = Some(String::from("value"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = &callback; TerraneDescriptor
        { identity : "function from int to int", name : "function from int to int", kind
        : "type", inherently_identity_bearing : false, fields : &[] } } .identity
        .to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = &optional; TerraneDescriptor
        { identity : "string|none", name : "string|none", kind : "type",
        inherently_identity_bearing : false, fields : &[] } } .identity.to_owned())
    );
}
