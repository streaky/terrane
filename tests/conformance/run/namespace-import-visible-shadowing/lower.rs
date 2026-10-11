// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: body-shadow/value.trn
// Namespace: body-shadow
fn parent_body_terrane_body_shadow() -> String {
    return String::from("imported-parent");
}
fn global_body() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(4_i128);
}
fn int8() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(8_i128);
}
fn utf8() -> String {
    return String::from("imported-prelude");
}
fn use_body_prelude() -> String {
    return utf8();
}
// Source: parent/child/main.trn
// Namespace: parent/child
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&original_total()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&parent_root_terrane_root_shadow())
    );
    println!("{}", terrane_scalar_support::scalar_text(&global_root()));
    println!("{}", terrane_scalar_support::scalar_text(&uint8()));
    println!("{}", terrane_scalar_support::scalar_text(&use_root_prelude()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&parent_body_terrane_body_shadow())
    );
    println!("{}", terrane_scalar_support::scalar_text(&global_body()));
    println!("{}", terrane_scalar_support::scalar_text(&int8()));
    println!("{}", terrane_scalar_support::scalar_text(&use_body_prelude()));
    println!("{}", terrane_scalar_support::scalar_text(&true));
}
// Source: parent/main.trn
// Namespace: parent
fn parent_root_terrane_parent() -> terrane_int_support::Int {
    return __TERRANE_GLOBAL_GLOBAL_ROOT
        .lock()
        .expect("program-global lock poisoned")
        .clone()
        .unwrap_or_else(|| __terrane_uninitialized_binding(
            "global-root",
            "main.trn",
            7,
            12,
        ))
        .clone();
}
fn parent_body_terrane_parent() -> terrane_int_support::Int {
    return __TERRANE_GLOBAL_GLOBAL_BODY
        .lock()
        .expect("program-global lock poisoned")
        .clone()
        .unwrap_or_else(|| __terrane_uninitialized_binding(
            "global-body",
            "main.trn",
            10,
            12,
        ))
        .clone();
}
fn original_total() -> terrane_int_support::Int {
    return parent_root_terrane_parent() + parent_body_terrane_parent();
}
// Source: root-shadow/value.trn
// Namespace: root-shadow
fn parent_root_terrane_root_shadow() -> String {
    return String::from("imported-parent");
}
fn global_root() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(3_i128);
}
fn uint8() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(8_i128);
}
fn print() -> String {
    return String::from("imported-prelude");
}
fn use_root_prelude() -> String {
    return print();
}
