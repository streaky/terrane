// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let values: terrane_collection_support::List<Token>;
    let item: Token;
    values = terrane_collection_support::List::<
        Token,
    >::new(
        vec![
            __terrane_raised(terrane_static_trn_546f6b656e_new(), 0 /* terrane-site: src/main.trn:5:35-5:46 */)
        ],
    );
    item = __terrane_raised(
        values
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    1 /* terrane-site: src/main.trn:6:18-6:27 */,
                ),
            ),
        1 /* terrane-site: src/main.trn:6:18-6:27 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(item.value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Token::value")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 2 /* terrane-site: src/main.trn:7:13-7:24 */))
    );
}
