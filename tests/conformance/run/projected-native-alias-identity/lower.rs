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
    let borrowed: owner::Holder<u8>;
    let borrowed_string: owner::Holder<String>;
    let holder: StringHolder;
    defaulted = __terrane_raised(
        make_default(),
        0 /* terrane-site: src/main.trn:8:23-8:36 */,
    );
    facade = defaulted.clone();
    second_facade = facade;
    declared_owner = second_facade;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(defaulted.clone()
        .into_value(), 1 /* terrane-site: src/main.trn:12:13-12:34 */))
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
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(round_trip
        .into_value(), 5 /* terrane-site: src/main.trn:18:13-18:35 */))
    );
    selected = __terrane_raised(
        make_string(),
        6 /* terrane-site: src/main.trn:21:32-21:44 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(selected.clone()
        .into_value(), 7 /* terrane-site: src/main.trn:22:13-22:33 */))
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
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(default_text
        .into_value(), 10 /* terrane-site: src/main.trn:25:13-25:37 */))
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
    borrowed = __terrane_raised(
        make_owner(),
        15 /* terrane-site: src/main.trn:34:37-34:48 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&consume_borrowed(&borrowed)));
    borrowed_string = __terrane_raised(
        make_string(),
        16 /* terrane-site: src/main.trn:36:39-36:51 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&consume_borrowed_string(&borrowed_string))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&consume_aliased_ref(&borrowed_string))
    );
    holder = StringHolder::terrane_construct(
        __terrane_raised(
            make_string(),
            17 /* terrane-site: src/main.trn:39:38-39:50 */,
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&consume_holder_field(&holder)));
}
fn consume_borrowed(value: &owner::Holder<u8>) -> u8 {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(value.clone().into_value()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "owner",
                                "owner::Holder::into_value",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        18 /* terrane-site: src/main.trn:43:12-43:29 */,
    );
}
fn consume_borrowed_string(value: &owner::Holder<String>) -> String {
    return __terrane_raised(
        value.clone().into_value(),
        19 /* terrane-site: src/main.trn:45:12-45:29 */,
    );
}
fn consume_aliased_ref(value: &owner::Holder<String>) -> String {
    let alias: &owner::Holder<String>;
    alias = value;
    return __terrane_raised(
        alias.clone().into_value(),
        20 /* terrane-site: src/main.trn:48:12-48:29 */,
    );
}
pub struct StringHolder {
    pub item: Option<owner::Holder<String>>,
}
impl StringHolder {
    pub fn terrane_construct(item: owner::Holder<String>) -> Self {
        let mut __terrane_constructed_value = Self { item: None };
        __terrane_constructed_value.construct(item);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, item: owner::Holder<String>) {
        {
            let __terrane_value = item;
            self.item = Some(__terrane_value);
        }
    }
}
fn consume_holder_field(holder: &StringHolder) -> String {
    return __terrane_raised(
        {
            let __terrane_snapshot = holder
                .item
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        }
            .into_value(),
        21 /* terrane-site: src/main.trn:54:12-54:35 */,
    );
}
