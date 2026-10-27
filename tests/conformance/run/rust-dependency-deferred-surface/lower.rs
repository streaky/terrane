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
    buffer = __terrane_raised(
        terrane_static_trn_42797465734d7574_with_capacity(
            terrane_int_support::Int::from(8_i128),
        ),
        0 /* terrane-site: src/main.trn:6:14-6:40 */,
    );
    remaining = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <bytes::BytesMut as bytes::BufMut>::remaining_mut(
                &buffer,
            )),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "bytes",
                        "bytes::BytesMut::remaining_mut",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:7:17-7:38 */,
    );
    candidate = __terrane_raised(
        terrane_static_trn_4e756d626572_from_u128(
            terrane_int_support::Int::from(42_i128),
        ),
        2 /* terrane-site: src/main.trn:8:17-8:38 */,
    );
    data = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| serde_json::error::Category::Data),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "serde_json",
                        "serde_json::error::Category::Data",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:9:12-9:27 */,
    );
    io = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| serde_json::error::Category::Io),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "serde_json",
                        "serde_json::error::Category::Io",
                    ),
                )
            }
        },
        4 /* terrane-site: src/main.trn:10:10-10:23 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(remaining >
        terrane_int_support::Int::from(0_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&candidate).is_some()));
    println!("{}", terrane_scalar_support::scalar_text(&(data != io)));
}
