// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates:
// Source: src/main.trn
// Namespace: descriptor-constructs-no-prelude
fn accepts(value: i8) -> i8 {
    return value;
}
fn main() {
    let value: i8 = accepts(1);
    let result: bool = {
        let _ = &value;
        true
    };
    let _ = &result;
}
