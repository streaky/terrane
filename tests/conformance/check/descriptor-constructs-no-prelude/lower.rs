// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates:
// Source: src/main.trn
// Namespace: descriptor-constructs-no-prelude
fn accepts(value: i8) -> i8 {
    return value;
}
fn main() {
    let value: i8;
    let result: bool;
    value = accepts(1);
    result = {
        let _ = &value;
        true
    };
    let _ = &result;
}
