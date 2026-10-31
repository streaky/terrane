// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: non-owning-reference-release
fn main() {
    let value: terrane_collection_support::List<terrane_int_support::Int>;
    let observer: &terrane_collection_support::List<terrane_int_support::Int>;
    let value_2: String;
    value = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(12_i128)]);
    observer = &value;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(observer
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:7:12-7:23 */)), 0 /* terrane-site: case.trn:7:12-7:23 */))
    );
    let __terrane_replacement = String::from("replacement");
    drop(value);
    value_2 = __terrane_replacement;
    println!("{}", terrane_scalar_support::scalar_text(&value_2));
}
