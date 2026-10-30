// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: references-provenance
fn main() {
    let values: std::sync::Arc<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let owner: std::sync::Arc<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let observer: std::sync::Weak<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let owned: terrane_collection_support::List<terrane_int_support::Int>;
    let transferred: terrane_collection_support::List<terrane_int_support::Int>;
    values = std::sync::Arc::new(
        TerraneSharedCell::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        ),
    );
    owner = values.clone();
    observer = std::sync::Arc::downgrade(&values.clone());
    {
        let __terrane_owner_0 = owner.clone();
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(2_i128);
        __terrane_owner_0
            .lock()
            .expect("shared reference lock poisoned")
            .append(__terrane_collection_argument_0);
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = values.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())), terrane_scalar_support::scalar_text(&{ let
        __terrane_owner_1 = (&observer).upgrade().expect("reference expired");
        __terrane_raised(__terrane_owner_1.lock().expect("reference lock poisoned")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:9:25-9:36 */)), 0 /* terrane-site: case.trn:9:25-9:36 */) })
    );
    owned = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(9_i128)]);
    transferred = owned;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(transferred
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:12:10-12:24 */)), 1 /* terrane-site: case.trn:12:10-12:24 */))
    );
}
