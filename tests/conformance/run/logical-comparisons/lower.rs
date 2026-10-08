// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: logical-comparisons
fn main() {
    let x: i64;
    let y: i64;
    x = 5;
    y = 9;
    println!("{}", terrane_scalar_support::scalar_text(&(x > 1 &&y > 2)));
}
