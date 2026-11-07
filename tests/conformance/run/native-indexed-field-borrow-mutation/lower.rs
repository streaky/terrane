// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct R {
    pub r: Option<Record>,
}
impl R {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { r: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = {
                let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                    7_i128,
                );
                __terrane_raised(
                    record(__terrane_conversion_input_0),
                    0 /* terrane-site: src/main.trn:7:18-7:27 */,
                )
            };
            self.r = Some(__terrane_value);
        }
    }
}
fn main() {
    let mut values: terrane_collection_support::List<R>;
    values = terrane_collection_support::List::<R>::new(vec![R::terrane_construct()]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_0 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: src/main.trn:10:13-10:22 */); let __terrane_index_place_1 =
        __terrane_raised(values.get(__terrane_index_0).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_0)),
        1 /* terrane-site: src/main.trn:10:13-10:22 */); let __terrane_native_input_0
        = &__terrane_index_place_1.r.as_ref().expect("required field initialized");
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.value()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "receiver_api",
        "receiver_api::Record::value")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 2 /* terrane-site: src/main.trn:10:13-10:31 */) })
    );
    {
        let __terrane_receiver_place_2 = {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                3 /* terrane-site: src/main.trn:11:5-11:14 */,
            );
            __terrane_raised(
                values
                    .get_mut(__terrane_index)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index,
                    )),
                3 /* terrane-site: src/main.trn:11:5-11:14 */,
            )
        };
        let __terrane_native_input_0 = &mut &mut __terrane_receiver_place_2
            .r
            .as_mut()
            .expect("required field initialized");
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(9_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = __terrane_native_input_0
                                    .set(
                                        terrane_int_support::coerce::<
                                            i64,
                                        >(&__terrane_conversion_input_0)
                                            .map_err(|error| crate::TerraneForeignError(
                                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                            ))?,
                                    );
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "receiver_api",
                                    "receiver_api::Record::set",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            4 /* terrane-site: src/main.trn:11:5-11:23 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_3 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: src/main.trn:12:14-12:23 */); let __terrane_index_place_4 =
        __terrane_raised(values.get(__terrane_index_3).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_3)),
        5 /* terrane-site: src/main.trn:12:14-12:23 */); let __terrane_native_input_0
        = &__terrane_index_place_4.r.as_ref().expect("required field initialized");
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.value()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "receiver_api",
        "receiver_api::Record::value")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 6 /* terrane-site: src/main.trn:12:13-12:33 */) })
    );
}
