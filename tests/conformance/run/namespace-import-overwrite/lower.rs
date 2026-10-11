// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: app/main.trn
// Namespace: app
fn local_choice() -> terrane_int_support::Int {
    println!(
        "{}", terrane_scalar_support::scalar_text(&local_value_terrane_local_one())
    );
    return local_value_terrane_local_two();
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&root_value_terrane_root_one()));
    println!("{}", terrane_scalar_support::scalar_text(&root_value_terrane_root_two()));
    println!("{}", terrane_scalar_support::scalar_text(&local_choice()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_base64_terrane_codec_shadow())
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&encode_base64_terrane_core_codecs(Vec::from([97]),
        false, true))
    );
}
// Source: codec-shadow/value.trn
// Namespace: codec-shadow
fn encode_base64_terrane_codec_shadow() -> String {
    return String::from("shadow");
}
// Source: local-one/value.trn
// Namespace: local-one
fn local_value_terrane_local_one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(3_i128);
}
// Source: local-two/value.trn
// Namespace: local-two
fn local_value_terrane_local_two() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(4_i128);
}
// Source: root-one/value.trn
// Namespace: root-one
fn root_value_terrane_root_one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
// Source: root-two/value.trn
// Namespace: root-two
fn root_value_terrane_root_two() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
}
