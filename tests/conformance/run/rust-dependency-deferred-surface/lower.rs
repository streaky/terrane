// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
fn main() {
    let buffer: BytesMut = __terrane_raised(
        terrane_static_trn_42797465734d7574_with_capacity(
            terrane_int_support::Int::from(8_i128),
        ),
        0 /* terrane-site: src/main.trn:8:14-8:40 */,
    );
    let remaining: terrane_int_support::Int = __terrane_raised(
        remaining_mut(&buffer),
        1 /* terrane-site: src/main.trn:9:17-9:38 */,
    );
    let candidate: Option<Number> = __terrane_raised(
        terrane_static_trn_4e756d626572_from_u128(
            terrane_int_support::Int::from(42_i128),
        ),
        2 /* terrane-site: src/main.trn:10:17-10:38 */,
    );
    let data: Category = __terrane_raised(
        __trn_44617461(),
        3 /* terrane-site: src/main.trn:11:12-11:17 */,
    );
    let io: Category = __terrane_raised(
        __trn_496f(),
        4 /* terrane-site: src/main.trn:12:10-12:13 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(remaining.clone() >
        terrane_int_support::Int::from(0_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&candidate.is_some()));
    println!("{}", terrane_scalar_support::scalar_text(&(data != io)));
}
// Source: <terrane>/projected/deps/bytes.trn
// Namespace: deps/bytes
pub use bytes::Bytes;
pub use bytes::BytesMut;
pub fn terrane_static_trn_42797465734d7574_new() -> Result<
    BytesMut,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| bytes::BytesMut::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "bytes", "bytes::BytesMut"))
        }
    }
}
pub fn terrane_static_trn_42797465734d7574_with_capacity(
    capacity: terrane_int_support::Int,
) -> Result<BytesMut, crate::TerraneForeignError> {
    let capacity = terrane_int_support::coerce::<usize>(&capacity)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| bytes::BytesMut::with_capacity(capacity)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "bytes", "bytes::BytesMut"))
        }
    }
}
pub fn terrane_static_trn_42797465734d7574_zeroed(
    len: terrane_int_support::Int,
) -> Result<BytesMut, crate::TerraneForeignError> {
    let len = terrane_int_support::coerce::<usize>(&len)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| bytes::BytesMut::zeroed(len)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "bytes", "bytes::BytesMut"))
        }
    }
}
// Source: <terrane>/projected/deps/bytes/bufmut.trn
// Namespace: deps/bytes/bufmut
pub fn remaining_mut(
    receiver: &BytesMut,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| <bytes::BytesMut as bytes::BufMut>::remaining_mut(
            receiver,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "bytes",
                    "<bytes::BytesMut as bytes::BufMut>::remaining_mut",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/serde-json.trn
// Namespace: deps/serde-json
pub use serde_json::Number;
pub fn terrane_static_trn_4e756d626572_from_f64(
    f: f64,
) -> Result<Option<Number>, crate::TerraneForeignError> {
    let f = f;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| serde_json::Number::from_f64(f)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::Number",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_4e756d626572_from_i128(
    i: terrane_int_support::Int,
) -> Result<Option<Number>, crate::TerraneForeignError> {
    let i = terrane_int_support::coerce::<i128>(&i)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| serde_json::Number::from_i128(i)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::Number",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_4e756d626572_from_u128(
    i: terrane_int_support::Int,
) -> Result<Option<Number>, crate::TerraneForeignError> {
    let i = terrane_int_support::coerce::<u128>(&i)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| serde_json::Number::from_u128(i)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::Number",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/serde-json/error.trn
// Namespace: deps/serde-json/error
pub use serde_json::error::Category;
// Source: <terrane>/projected/deps/serde-json/error/category.trn
// Namespace: deps/serde-json/error/category
/// Projected enum variant constructor for `serde_json::error::Category::Data`.
pub fn __trn_44617461() -> Result<Category, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| serde_json::error::Category::Data),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::error::Category::Data",
                ),
            )
        }
    }
}
/// Projected enum variant constructor for `serde_json::error::Category::Io`.
pub fn __trn_496f() -> Result<Category, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| serde_json::error::Category::Io),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::error::Category::Io",
                ),
            )
        }
    }
}
