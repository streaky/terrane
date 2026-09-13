// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: non-owning-reference-release
fn main() {
    let value: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(12_i128)]);
    let observer: &terrane_collection_support::List<terrane_int_support::Int> = &value;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(observer.clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:8:12-8:23 */)), 0 /* terrane-site: case.trn:8:12-8:23 */))
    );
    let _ = &value;
    let value: String = String::from("replacement");
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
