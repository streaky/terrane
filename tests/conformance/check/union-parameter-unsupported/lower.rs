// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: union-parameter-unsupported
fn choose(value: i8) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn main() {
    let value: i8;
    value = 1;
    choose(value);
}
