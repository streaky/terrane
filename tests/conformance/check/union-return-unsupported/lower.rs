// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: union-return-unsupported
fn choose() -> i8 {
    let value: i8;
    value = 1;
    return value;
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&choose()));
}
