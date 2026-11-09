// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let values: terrane_collection_support::Tuple<Token>;
    values = terrane_collection_support::Tuple::<
        Token,
    >::new(
        vec![
            __terrane_raised(terrane_static_trn_546f6b656e_new(), 0 /* terrane-site: src/main.trn:5:22-5:33 */),
            __terrane_raised(terrane_static_trn_546f6b656e_new(), 1 /* terrane-site: src/main.trn:5:37-5:48 */)
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: src/main.trn:6:13-6:22 */)), 2 /* terrane-site: src/main.trn:6:13-6:22 */); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.value()) })) { Ok(result)
        => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::Token::value")) }; Ok:: < _, crate ::TerraneForeignError
        > (terrane_int_support::Int::from_u128(value as u128)) } } (),
        3 /* terrane-site: src/main.trn:6:13-6:29 */) })
    );
}
