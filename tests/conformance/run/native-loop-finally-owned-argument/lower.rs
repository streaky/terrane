// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub struct W {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl W {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn consume(value: View) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(value.into_size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::into_size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 0 /* terrane-site: src/main.trn:11:13-11:29 */))
    );
}
fn main() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    let mut i: terrane_int_support::Int;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        1 /* terrane-site: src/main.trn:13:9-13:21 */,
    );
    let _ = v
        .insert(
            __terrane_raised(
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
                2 /* terrane-site: src/main.trn:14:9-14:16 */,
            ),
        );
    i = terrane_int_support::Int::from(0_i128);
    while i < terrane_int_support::Int::from(2_i128) {
        let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!("{}", terrane_scalar_support::scalar_text(&1));
                TerraneCompletion::Normal
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_0: TerraneCompletion<()> = (|| {
            consume(v.take().expect("flow-proven availability"));
            let _ = v
                .insert(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane_query_witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 3 /* terrane-site: src/main.trn:21:17-21:24 */
                    ),
                );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_0 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_0 = replacement,
        }
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break => break,
            TerraneCompletion::Continue => {
                __terrane_generated_defect("non-continuing try produced continue")
            }
        }
        i = i.clone() + terrane_int_support::Int::from(1_i128);
    }
}
