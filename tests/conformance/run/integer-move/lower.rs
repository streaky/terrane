// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: integer-move
fn compact() {
    let owned: i64;
    let transferred: terrane_int_support::Int;
    owned = 1;
    transferred = terrane_int_support::Int::from(owned as i128);
    println!("{}", terrane_scalar_support::scalar_text(&transferred));
}
fn reassigned() {
    let mut owned: terrane_int_support::Int;
    let transferred: terrane_int_support::Int;
    owned = terrane_int_support::Int::from(1_i128);
    transferred = owned;
    owned = terrane_int_support::Int::from(2_i128);
    println!("{}", terrane_scalar_support::scalar_text(&transferred));
    println!("{}", terrane_scalar_support::scalar_text(&owned));
}
fn large() {
    let mut owned: terrane_int_support::Int;
    let transferred: terrane_int_support::Int;
    owned = terrane_int_support::Int::from_decimal(
        "340282366920938463463374607431768211456",
    );
    transferred = owned;
    owned = terrane_int_support::Int::from_decimal(
        "340282366920938463463374607431768211457",
    );
    println!("{}", terrane_scalar_support::scalar_text(&transferred));
    println!("{}", terrane_scalar_support::scalar_text(&owned));
}
fn narrowed() {
    let owned: Option<terrane_int_support::Int>;
    let transferred: terrane_int_support::Int;
    owned = Some(terrane_int_support::Int::from(3_i128));
    if (&owned).is_some() {
        transferred = owned.expect("semantic optional narrowing");
        println!("{}", terrane_scalar_support::scalar_text(&transferred));
    }
}
fn from_parameter(owned: terrane_int_support::Int) -> terrane_int_support::Int {
    return owned;
}
fn main() {
    compact();
    reassigned();
    large();
    narrowed();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&from_parameter(terrane_int_support::Int::from(5_i128)))
    );
}
