// Generated deterministically by Terrane <version>.
// Source: app/a-first.trn
// Namespace: app
fn first_choice() -> String {
    return pick_terrane_two();
}
// Source: app/c-main.trn
// Namespace: app
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&first_import_choice()));
    println!("{}", terrane_scalar_support::scalar_text(&first_choice()));
    println!("{}", terrane_scalar_support::scalar_text(&pick_terrane_two()));
}
// Source: one/value.trn
// Namespace: one
fn pick_terrane_one() -> String {
    return String::from("one");
}
fn first_import_choice() -> String {
    return pick_terrane_one();
}
// Source: two/value.trn
// Namespace: two
fn pick_terrane_two() -> String {
    return String::from("two");
}
