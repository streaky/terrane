// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn first<TerraneType54>(
    values: terrane_collection_support::List<
        std::sync::Arc<TerraneSharedCell<TerraneType54>>,
    >,
) -> std::sync::Arc<TerraneSharedCell<TerraneType54>> {
    return __terrane_raised(
            values
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        0 /* terrane-site: src/main.trn:7:12-7:21 */,
                    ),
                ),
            0 /* terrane-site: src/main.trn:7:12-7:21 */,
        )
        .clone();
}
fn main() {
    let values: terrane_collection_support::List<
        std::sync::Arc<TerraneSharedCell<Holder>>,
    >;
    let value: std::sync::Arc<TerraneSharedCell<terrane_query_witness::Holder>>;
    values = terrane_collection_support::List::<
        std::sync::Arc<TerraneSharedCell<Holder>>,
    >::new(
        vec![
            std::sync::Arc::new(TerraneSharedCell::new(__terrane_raised(terrane_static_trn_486f6c646572_new(),
            1 /* terrane-site: src/main.trn:10:58-10:70 */)))
        ],
    );
    value = first(values).clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_1 = value
        .clone(); let __terrane_call_result = __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_owner_1.lock()
        .expect("shared reference lock poisoned").label_len()) })) { Ok(result) => result
        ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 2 /* terrane-site: src/main.trn:12:13-12:29 */);
        __terrane_call_result })
    );
}
