// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn get(values: terrane_collection_support::Map<String, Token>) -> Token {
    return __terrane_raised(
        values.get_or_error(&String::from("item")),
        0 /* terrane-site: src/main.trn:7:10-7:24 */,
    );
}
fn main() {
    let values: terrane_collection_support::Map<String, Token>;
    let value: Token;
    values = terrane_collection_support::Map::<
        String,
        Token,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            __terrane_raised(terrane_static_trn_546f6b656e_new(), 1 /* terrane-site: src/main.trn:10:44-10:55 */))
        ],
    );
    value = get(values);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(value.clone().value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Token::value")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 2 /* terrane-site: src/main.trn:12:11-12:23 */))
    );
}
