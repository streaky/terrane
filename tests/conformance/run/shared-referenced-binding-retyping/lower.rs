// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: shared-referenced-binding-retyping
fn main() {
    let value: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    > = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(12_i128)]),
        ),
    );
    let owner: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    > = value.clone();
    let _ = &value;
    let value: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(14_i128)]);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
        __terrane_value = owner.lock().expect("shared reference lock poisoned").clone();
        __terrane_value }
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:9:12-9:20 */)), 0 /* terrane-site: case.trn:9:12-9:20 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(value
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:9:22-9:30 */)), 1 /* terrane-site: case.trn:9:22-9:30 */))
    );
}
