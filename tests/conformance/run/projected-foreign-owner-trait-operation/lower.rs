// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: Value;
    value = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(37_i128);
        __terrane_raised(
            terrane_static_trn_56616c7565_new(__terrane_conversion_input_0),
            0 /* terrane-site: src/main.trn:6:19-6:33 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(< provider::Value as contract::Doubled >
        ::doubled(&value)) })) { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "provider", "provider::Value::doubled")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 1 /* terrane-site: src/main.trn:7:13-7:27 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        &provider::Value::BASE; __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(< provider::Value as contract::Doubled >
        ::doubled(__terrane_native_input_0)) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload, "provider",
        "provider::Value::doubled")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 2 /* terrane-site: src/main.trn:8:13-8:35 */) })
    );
}
