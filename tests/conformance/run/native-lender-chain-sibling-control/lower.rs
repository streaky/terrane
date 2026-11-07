// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Cfg {
    pub sql: String,
    pub other: String,
}
impl Cfg {
    pub fn terrane_construct() -> Self {
        Self {
            sql: String::from("insert"),
            other: String::from("spare"),
        }
    }
}
fn main() {
    let mut cfg: Cfg;
    let q: terrane_query_witness::View<'_>;
    let next: terrane_query_witness::View<'_>;
    cfg = Cfg::terrane_construct();
    q = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(terrane_query_witness::query(&cfg.sql)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::query",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        0 /* terrane-site: src/main.trn:12:9-12:23 */,
    );
    next = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(q.chain()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::View::chain",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        1 /* terrane-site: src/main.trn:13:12-13:20 */,
    );
    {
        let __terrane_value = String::from("ok");
        cfg.other = __terrane_value;
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&next).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 2 /* terrane-site: src/main.trn:15:13-15:23 */)), String::from(" "),
        terrane_scalar_support::scalar_text(&cfg.other)
    );
    {
        let __terrane_value = String::from("x");
        cfg.sql = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&cfg.sql));
}
