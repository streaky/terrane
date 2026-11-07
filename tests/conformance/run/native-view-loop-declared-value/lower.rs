// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let h: Holder;
    let l: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let mut c: terrane_query_witness::CopyView<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: src/main.trn:7:9-7:21 */,
    );
    l = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&h).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 1 /* terrane-site: src/main.trn:8:33-8:45 */).into(); value }
        ],
    );
    let __terrane_iterable_0 = l;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        c = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(c.size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 2 /* terrane-site: src/main.trn:10:17-10:24 */))
        );
    }
}
