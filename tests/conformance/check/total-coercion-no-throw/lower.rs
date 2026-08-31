// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: total-coercion-no-throw
fn widen(value: i8) -> i16 {
    return value as i16;
}
fn main() {
    let value: i8 = 12;
    let widened: i16 = widen(value);
    println!("{}", terrane_scalar_support::scalar_text(&widened));
}
