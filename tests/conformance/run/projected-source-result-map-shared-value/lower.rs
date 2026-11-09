// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn get<TerraneType54>(
    values: terrane_collection_support::Map<
        String,
        std::sync::Arc<TerraneSharedCell<TerraneType54>>,
    >,
) -> std::sync::Arc<TerraneSharedCell<TerraneType54>> {
    return __terrane_raised(
            values.get_or_error(&String::from("item")),
            0 /* terrane-site: src/main.trn:9:12-9:26 */,
        )
        .clone();
}
fn main() {
    let values: terrane_collection_support::Map<
        String,
        std::sync::Arc<TerraneSharedCell<Holder>>,
    >;
    let value: std::sync::Arc<TerraneSharedCell<terrane_query_witness::Holder>>;
    values = terrane_collection_support::Map::<
        String,
        std::sync::Arc<TerraneSharedCell<Holder>>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            std::sync::Arc::new(TerraneSharedCell::new(__terrane_raised(terrane_static_trn_486f6c646572_new(),
            1 /* terrane-site: src/main.trn:12:70-12:82 */))))
        ],
    );
    value = get(values).clone();
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
        u128)) } } (), 2 /* terrane-site: src/main.trn:14:13-14:29 */);
        __terrane_call_result })
    );
}
