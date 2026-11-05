// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let h: Holder;
    let v: terrane_query_witness::View<'_>;
    let outer: i64;
    let inner: i64;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: src/main.trn:7:9-7:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        1 /* terrane-site: src/main.trn:8:9-8:16 */,
    );
    outer = 0;
    while outer < 2 {
        inner = 0;
        while inner < 2 {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 2 /* terrane-site: src/main.trn:13:21-13:33 */))
            );
            break;
        }
        break;
    }
}
