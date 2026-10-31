// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-referenced-binding-retyping
fn main() {
    let mut value: std::sync::Arc<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let owner: std::sync::Arc<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    value = std::sync::Arc::new(
        TerraneSharedCell::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(12_i128)]),
        ),
    );
    owner = value.clone();
    value = std::sync::Arc::new(
        TerraneSharedCell::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(14_i128)]),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_0 = &owner;
        __terrane_raised(__terrane_owner_0.lock()
        .expect("shared reference lock poisoned")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:8:12-8:20 */)), 0 /* terrane-site: case.trn:8:12-8:20 */) }), terrane_scalar_support::scalar_text(&{ let
        __terrane_owner_1 = &value; __terrane_raised(__terrane_owner_1.lock()
        .expect("reference lock poisoned")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:8:22-8:30 */)), 1 /* terrane-site: case.trn:8:22-8:30 */) })
    );
}
