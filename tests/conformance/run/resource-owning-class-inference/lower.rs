// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: resource-owning-class-inference
pub struct StreamOwner {
    pub input: ByteReader,
}
impl StreamOwner {
    pub fn terrane_construct() -> Self {
        Self { input: stdin() }
    }
}
fn finish(owner: StreamOwner) {
    let _ = &owner;
    return ();
}
fn main() {
    let source: StreamOwner;
    let destination: StreamOwner;
    let result: ReadResult;
    source = StreamOwner::terrane_construct();
    destination = source;
    result = destination.input.read(terrane_int_support::Int::from(1_i128));
    println!("{}", terrane_scalar_support::scalar_text(&result.completed));
    finish(destination);
}
