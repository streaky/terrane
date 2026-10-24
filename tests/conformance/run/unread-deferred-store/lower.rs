// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: unread-deferred-store
fn pick(kind: terrane_int_support::Int) -> terrane_int_support::Int {
    let value: terrane_int_support::Int;
    if kind.clone() == terrane_int_support::Int::from(0_i128) {
        value = terrane_int_support::Int::from(1_i128);
        let _ = &value;
    } else {
        value = terrane_int_support::Int::from(2_i128);
        let _ = &value;
    }
    return kind.clone();
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&pick(terrane_int_support::Int::from(0_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&pick(terrane_int_support::Int::from(1_i128)))
    );
}
