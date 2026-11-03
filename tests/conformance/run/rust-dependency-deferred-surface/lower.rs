// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let buffer: BytesMut;
    let remaining: terrane_int_support::Int;
    let candidate: Option<Number>;
    let data: Category;
    let io: Category;
    buffer = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(8_i128);
        __terrane_raised(
            terrane_static_trn_42797465734d7574_with_capacity(
                __terrane_conversion_input_0,
            ),
            0 /* terrane-site: src/main.trn:6:14-6:40 */,
        )
    };
    remaining = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(<bytes::BytesMut as bytes::BufMut>::remaining_mut(&buffer)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "bytes",
                                "bytes::BytesMut::remaining_mut",
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
        1 /* terrane-site: src/main.trn:7:17-7:38 */,
    );
    candidate = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(42_i128);
        __terrane_raised(
            terrane_static_trn_4e756d626572_from_u128(__terrane_conversion_input_0),
            2 /* terrane-site: src/main.trn:8:17-8:38 */,
        )
    };
    data = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(serde_json::error::Category::Data) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "serde_json",
                                "serde_json::error::Category::Data",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        3 /* terrane-site: src/main.trn:9:12-9:27 */,
    );
    io = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(serde_json::error::Category::Io) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "serde_json",
                                "serde_json::error::Category::Io",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        4 /* terrane-site: src/main.trn:10:10-10:23 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(remaining >
        terrane_int_support::Int::from(0_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&candidate).is_some()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
        &data; let __terrane_comparison_right = &io; < Category as std::cmp::PartialEq >
        ::ne(__terrane_comparison_left, __terrane_comparison_right) })
    );
}
