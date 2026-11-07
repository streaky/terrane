// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let values: terrane_collection_support::UnorderedMap<String, Token>;
    values = terrane_collection_support::UnorderedMap::<
        String,
        Token,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("k"),
            __terrane_raised(terrane_static_trn_546f6b656e_new(), 0 /* terrane-site: src/main.trn:5:63-5:74 */))
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values.get_or_error(&String::from("k")), 1 /* terrane-site: src/main.trn:6:13-6:24 */); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.value()) })) { Ok(result)
        => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::Token::value")) }; Ok:: < _, crate ::TerraneForeignError
        > (terrane_int_support::Int::from_u128(value as u128)) } } (),
        2 /* terrane-site: src/main.trn:6:13-6:31 */) })
    );
}
