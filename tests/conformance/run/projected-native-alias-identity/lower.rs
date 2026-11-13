// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct TerraneNsAppMutableAliasIdentity2F617070006D757461626C652D616C696173 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppMutableAliasIdentity2F617070006D757461626C652D616C696173 {
    pub fn terrane_construct(value: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: terrane_int_support::Int) {
        {
            let __terrane_value: terrane_int_support::Int = value.clone();
            self.value = __terrane_value;
        }
    }
}
fn mutate_collision_alias(value: &mut api::MutableAlias, replacement: u8) {
    {
        value.set_value(replacement);
    }
}
fn mutate_default_alias(value: &mut api::Alias<u8>, replacement: u8) {
    {
        let __terrane_conversion_input_0 = replacement;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = (*value).set_value(__terrane_conversion_input_0);
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "api",
                                    "api::Alias::set_value",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            0 /* terrane-site: src/main.trn:17:5-17:33 */,
        )
    };
}
fn main() {
    let defaulted: api::Alias<u8>;
    let facade: owner::Holder<u8>;
    let second_facade: owner::Holder<u8>;
    let declared_owner: owner::Holder<u8>;
    let underlying: owner::Holder<u8>;
    let round_trip: api::Alias<u8>;
    let selected: api::Alias<String>;
    let default_text: api::StringAlias<String>;
    let provider_input: u8;
    let facade_input: u8;
    let provider_value: PublicValue;
    let facade_value: PublicValue;
    let borrowed: owner::Holder<u8>;
    let borrowed_string: api::Alias<String>;
    let holder: StringHolder;
    let mut counter: owner::Holder<u8>;
    let source_value: TerraneNsAppMutableAliasIdentity2F617070006D757461626C652D616C696173;
    let mut collision_counter: owner::Holder<u8>;
    defaulted = __terrane_raised(
        make_default(),
        1 /* terrane-site: src/main.trn:21:23-21:36 */,
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
        (value) } } (), 2 /* terrane-site: src/main.trn:25:13-25:34 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        declared_owner; __terrane_raised(take_byte(__terrane_conversion_input_0),
        3 /* terrane-site: src/main.trn:26:13-26:38 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        defaulted; __terrane_raised(owner_byte(__terrane_conversion_input_0),
        4 /* terrane-site: src/main.trn:27:13-27:34 */) })
    );
    underlying = __terrane_raised(
        make_owner(),
        5 /* terrane-site: src/main.trn:29:39-29:50 */,
    );
    round_trip = underlying;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(round_trip.into_value()) })) { Ok(result) => result
        ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "api",
        "api::Alias::into_value")) }; Ok:: < _, crate ::TerraneForeignError > (value) } }
        (), 6 /* terrane-site: src/main.trn:31:13-31:35 */))
    );
    selected = __terrane_raised(
        make_string(),
        7 /* terrane-site: src/main.trn:34:32-34:44 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(selected.clone().into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "api", "api::Alias::into_value")) }; Ok:: < _, crate ::TerraneForeignError >
        (value) } } (), 8 /* terrane-site: src/main.trn:35:13-35:33 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        selected; __terrane_raised(take_text(__terrane_conversion_input_0),
        9 /* terrane-site: src/main.trn:36:13-36:32 */) })
    );
    default_text = __terrane_raised(
        make_string_default(),
        10 /* terrane-site: src/main.trn:37:32-37:52 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(default_text.into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "api", "api::StringAlias::into_value")) }; Ok:: < _, crate ::TerraneForeignError
        > (value) } } (), 11 /* terrane-site: src/main.trn:38:13-38:37 */))
    );
    provider_input = 23;
    facade_input = 29;
    provider_value = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(
            provider_input as i128,
        );
        __terrane_raised(
            terrane_static_trn_5075626c696356616c7565_new(__terrane_conversion_input_0),
            12 /* terrane-site: src/main.trn:43:36-43:68 */,
        )
    };
    facade_value = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(
            facade_input as i128,
        );
        __terrane_raised(
            terrane_static_trn_5075626c696356616c7565_new(__terrane_conversion_input_0),
            13 /* terrane-site: src/main.trn:44:32-44:64 */,
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
        (), 14 /* terrane-site: src/main.trn:45:13-45:39 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(facade_value.into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "owner", "owner::PublicValue::into_value")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 15 /* terrane-site: src/main.trn:46:13-46:37 */))
    );
    borrowed = __terrane_raised(
        make_owner(),
        16 /* terrane-site: src/main.trn:47:37-47:48 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&consume_borrowed(&borrowed)));
    borrowed_string = __terrane_raised(
        make_string(),
        17 /* terrane-site: src/main.trn:49:39-49:51 */,
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
            18 /* terrane-site: src/main.trn:52:38-52:50 */,
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&consume_holder_field(&holder)));
    counter = __terrane_raised(
        make_owner(),
        19 /* terrane-site: src/main.trn:54:15-54:26 */,
    );
    mutate_default_alias(&mut counter, 73);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(counter.into_value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "owner",
        "owner::Holder::into_value")) }; Ok:: < _, crate ::TerraneForeignError > (value)
        } } (), 20 /* terrane-site: src/main.trn:56:13-56:32 */))
    );
    source_value = TerraneNsAppMutableAliasIdentity2F617070006D757461626C652D616C696173::terrane_construct(
        terrane_int_support::Int::from(37_i128),
    );
    collision_counter = __terrane_raised(
        make_owner(),
        21 /* terrane-site: src/main.trn:58:25-58:36 */,
    );
    mutate_collision_alias(&mut collision_counter, 91);
    println!("{}", terrane_scalar_support::scalar_text(&source_value.value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(collision_counter.into_value()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "owner", "owner::Holder::into_value")) }; Ok:: < _, crate ::TerraneForeignError >
        (value) } } (), 22 /* terrane-site: src/main.trn:61:13-61:42 */))
    );
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
        23 /* terrane-site: src/main.trn:64:12-64:29 */,
    );
}
fn consume_borrowed_string(value: &api::Alias<String>) -> String {
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
                                "api",
                                "api::Alias::into_value",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        24 /* terrane-site: src/main.trn:66:12-66:29 */,
    );
}
fn consume_aliased_ref(value: &api::Alias<String>) -> String {
    let alias: &api::Alias<String>;
    alias = value;
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(alias.clone().into_value()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "api",
                                "api::Alias::into_value",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        25 /* terrane-site: src/main.trn:69:12-69:29 */,
    );
}
pub struct StringHolder {
    pub item: Option<api::Alias<String>>,
}
impl StringHolder {
    pub fn terrane_construct(item: api::Alias<String>) -> Self {
        let mut __terrane_constructed_value = Self { item: None };
        __terrane_constructed_value.construct(item);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, item: api::Alias<String>) {
        {
            let __terrane_value: api::Alias<String> = item;
            self.item = Some(__terrane_value);
        }
    }
}
fn consume_holder_field(holder: &StringHolder) -> String {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            {
                                let __terrane_snapshot = holder
                                    .item
                                    .as_ref()
                                    .expect("required field initialized")
                                    .clone();
                                __terrane_snapshot
                            }
                                .into_value(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "api",
                                "api::Alias::into_value",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        26 /* terrane-site: src/main.trn:75:12-75:35 */,
    );
}
