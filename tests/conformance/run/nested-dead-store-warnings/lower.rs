// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: nested-dead-store-warnings
fn main() {
    let mut nested: i8;
    let mut top: i8;
    if 1 == 1 {
        nested = 1;
        let _ = &nested;
        nested = 2;
        println!("{}", terrane_scalar_support::scalar_text(&nested));
    }
    top = 3;
    let _ = &top;
    top = 4;
    println!("{}", terrane_scalar_support::scalar_text(&top));
}
