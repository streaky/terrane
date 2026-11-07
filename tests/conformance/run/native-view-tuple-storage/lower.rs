// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let h: Holder;
    let c: terrane_query_witness::CopyView<'_>;
    let t: terrane_collection_support::Tuple<terrane_query_witness::CopyView<'_>>;
    let nested: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_query_witness::CopyView<'_>>,
    >;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: src/main.trn:7:9-7:21 */,
    );
    c = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        1 /* terrane-site: src/main.trn:8:9-8:21 */,
    );
    t = terrane_collection_support::Tuple::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![{ let value : terrane_query_witness::CopyView < '_ > = c.into(); value }],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(c.size()) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 2 /* terrane-site: src/main.trn:10:13-10:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(t
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        3 /* terrane-site: src/main.trn:11:13-11:17 */)), 3 /* terrane-site: src/main.trn:11:13-11:17 */); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 4 /* terrane-site: src/main.trn:11:13-11:23 */) })
    );
    nested = terrane_collection_support::List::<
        terrane_collection_support::Tuple<terrane_query_witness::CopyView<'_>>,
    >::new(
        vec![
            terrane_collection_support::Tuple:: < terrane_query_witness::CopyView < '_ >
            > ::new(vec![{ let value : terrane_query_witness::CopyView < '_ > =
            __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&h).copy_view()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 5 /* terrane-site: src/main.trn:12:29-12:41 */).into(); value }])
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 = { let
        __terrane_index_2 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: src/main.trn:13:13-13:22 */); let __terrane_index_place_3 =
        __terrane_raised(nested.get(__terrane_index_2).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_2)),
        6 /* terrane-site: src/main.trn:13:13-13:22 */);
        __terrane_raised(__terrane_index_place_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: src/main.trn:13:13-13:25 */)), 7 /* terrane-site: src/main.trn:13:13-13:25 */) }; __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 8 /* terrane-site: src/main.trn:13:13-13:31 */) })
    );
}
