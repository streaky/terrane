// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: core-tool-imports
fn describe(enabled: bool, payload: Vec<u8>, small: i8) -> String {
    let _ = &small;
    encode_base64(payload, enabled, enabled);
    return String::from("core tools are explicit");
}
fn main() {
    describe(true, Vec::from([97, 98, 99]), 1);
    println!("{}", String::from("core namespace import works"));
}
