// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn total(
    values: &terrane_collection_support::List<CopyView>,
) -> terrane_int_support::Int {
    let mut n: terrane_int_support::Int;
    let mut c: &CopyView;
    n = terrane_int_support::Int::from(0_i128);
    let __terrane_iterable_0 = values;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        c = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        n = n.clone()
            + __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok(c.clone().size()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_query_witness",
                                        "terrane_query_witness::CopyView::size",
                                    ),
                                );
                            }
                        };
                        Ok::<
                            _,
                            crate::TerraneForeignError,
                        >(terrane_int_support::Int::from_u128(value as u128))
                    }
                }(),
                0 /* terrane-site: src/main.trn:9:18-9:25 */,
            );
    }
    return n.clone();
}
fn main() {
    let h: Holder;
    let values: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        1 /* terrane-site: src/main.trn:13:9-13:21 */,
    );
    values = terrane_collection_support::List::<
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
            ::TerraneForeignError > (value) } } (), 2 /* terrane-site: src/main.trn:14:38-14:50 */); value }
        ],
    );
    println!("{}", terrane_scalar_support::scalar_text(&total(&values)));
}
