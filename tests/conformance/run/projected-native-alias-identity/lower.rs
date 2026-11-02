// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let defaulted: owner::Holder<u8>;
    let facade: owner::Holder<u8>;
    let second_facade: owner::Holder<u8>;
    let declared_owner: owner::Holder<u8>;
    let underlying: owner::Holder<u8>;
    let round_trip: owner::Holder<u8>;
    let selected: owner::Holder<String>;
    let default_text: owner::Holder<String>;
    let provider_input: u8;
    let facade_input: u8;
    let provider_value: PublicValue;
    let facade_value: PublicValue;
    defaulted = __terrane_raised(
        make_default(),
        0 /* terrane-site: src/main.trn:8:23-8:36 */,
    );
    facade = defaulted.clone();
    second_facade = facade;
    declared_owner = second_facade;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(defaulted.clone().into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "api", "api::Alias::into_value")) }; Ok:: < _, crate ::TerraneForeignError >
        (value) } } (), 1 /* terrane-site: src/main.trn:12:13-12:34 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        declared_owner; __terrane_raised(take_byte(__terrane_conversion_input_0),
        2 /* terrane-site: src/main.trn:13:13-13:38 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        defaulted; __terrane_raised(owner_byte(__terrane_conversion_input_0),
        3 /* terrane-site: src/main.trn:14:13-14:34 */) })
    );
    underlying = __terrane_raised(
        make_owner(),
        4 /* terrane-site: src/main.trn:16:39-16:50 */,
    );
    round_trip = underlying;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(round_trip.into_value()) })) { Ok(result) => result
        ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "api",
        "api::Alias::into_value")) }; Ok:: < _, crate ::TerraneForeignError > (value) } }
        (), 5 /* terrane-site: src/main.trn:18:13-18:35 */))
    );
    selected = __terrane_raised(
        make_string(),
        6 /* terrane-site: src/main.trn:21:32-21:44 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(selected.clone().into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "api", "api::Alias::into_value")) }; Ok:: < _, crate ::TerraneForeignError >
        (value) } } (), 7 /* terrane-site: src/main.trn:22:13-22:33 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        selected; __terrane_raised(take_text(__terrane_conversion_input_0),
        8 /* terrane-site: src/main.trn:23:13-23:32 */) })
    );
    default_text = __terrane_raised(
        make_string_default(),
        9 /* terrane-site: src/main.trn:24:32-24:52 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(default_text.into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "api", "api::StringAlias::into_value")) }; Ok:: < _, crate ::TerraneForeignError
        > (value) } } (), 10 /* terrane-site: src/main.trn:25:13-25:37 */))
    );
    provider_input = 23;
    facade_input = 29;
    provider_value = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(
            provider_input as i128,
        );
        __terrane_raised(
            terrane_static_trn_5075626c696356616c7565_new(__terrane_conversion_input_0),
            11 /* terrane-site: src/main.trn:30:36-30:68 */,
        )
    };
    facade_value = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(
            facade_input as i128,
        );
        __terrane_raised(
            terrane_static_trn_5075626c696356616c7565_new(__terrane_conversion_input_0),
            12 /* terrane-site: src/main.trn:31:32-31:64 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(provider_value.into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "owner", "owner::PublicValue::into_value")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 13 /* terrane-site: src/main.trn:32:13-32:39 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(facade_value.into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "owner", "owner::PublicValue::into_value")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 14 /* terrane-site: src/main.trn:33:13-33:37 */))
    );
}
