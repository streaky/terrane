// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn exact_label<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    return __terrane_raised(
        borrowed_label(&*state),
        0 /* terrane-site: src/main.trn:8:12-8:33 */,
    );
}
fn exact<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    return exact_label(state);
}
fn exact_nested<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let first: witness::BorrowedLabel<'_>;
    let second: witness::BorrowedButton<'_>;
    first = __terrane_raised(
        borrowed_label(&*state),
        1 /* terrane-site: src/main.trn:13:13-13:34 */,
    );
    second = __terrane_raised(
        borrowed_button(&*state),
        2 /* terrane-site: src/main.trn:14:14-14:36 */,
    );
    return __terrane_raised(
        borrowed_group(first, second),
        3 /* terrane-site: src/main.trn:15:12-15:41 */,
    );
}
fn exact_typed<'view>(
    state: &'view String,
) -> witness::BorrowedValue<'view, std::string::String> {
    return __terrane_raised(
        borrowed_value(&*state),
        4 /* terrane-site: src/main.trn:18:12-18:33 */,
    );
}
fn exact_recursive<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    let value: witness::BorrowedLabel<'_>;
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(100_i128)
    {
        return exact_recursive(state);
    }
    value = __terrane_raised(
        borrowed_label(&*state),
        5 /* terrane-site: src/main.trn:23:13-23:34 */,
    );
    return value;
}
fn exact_wrapped_binding<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let value: witness::BorrowedLabel<'_>;
    value = __terrane_raised(
        borrowed_label(&*state),
        6 /* terrane-site: src/main.trn:27:13-27:34 */,
    );
    return __terrane_raised(
        borrowed_label_group(value),
        7 /* terrane-site: src/main.trn:28:12-28:39 */,
    );
}
fn exact_ping<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(100_i128)
    {
        return exact_pong(state);
    }
    return __terrane_raised(
        borrowed_label(&*state),
        8 /* terrane-site: src/main.trn:33:12-33:33 */,
    );
}
fn exact_pong<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(100_i128)
    {
        return exact_ping(state);
    }
    return __terrane_raised(
        borrowed_label(&*state),
        9 /* terrane-site: src/main.trn:38:12-38:33 */,
    );
}
fn exact_inner_label<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    return __terrane_raised(
        borrowed_label(&*state),
        10 /* terrane-site: src/main.trn:41:12-41:33 */,
    );
}
fn exact_outer_group<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    return __terrane_raised(
        borrowed_label_group(exact_inner_label(state)),
        11 /* terrane-site: src/main.trn:44:12-44:60 */,
    );
}
fn exact_forwarded_binding<'view>(
    state: &'view String,
) -> witness::BorrowedLabel<'view> {
    let value: witness::BorrowedLabel<'_>;
    value = exact_inner_label(state);
    return value;
}
fn exact_reassign_helper_then_producer<'view>(
    state: &'view String,
) -> witness::BorrowedLabel<'view> {
    let mut value: witness::BorrowedLabel<'_>;
    value = exact_inner_label(state);
    let _ = &value;
    value = __terrane_raised(
        borrowed_label(&*state),
        12 /* terrane-site: src/main.trn:52:13-52:34 */,
    );
    return value;
}
fn exact_reassign_reverse<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    let mut value: witness::BorrowedLabel<'_>;
    value = __terrane_raised(
        borrowed_label(&*state),
        13 /* terrane-site: src/main.trn:56:13-56:34 */,
    );
    let _ = &value;
    value = exact_inner_label(state);
    return value;
}
fn exact_reassign_conditional<'view>(
    state: &'view String,
) -> witness::BorrowedLabel<'view> {
    let mut value: witness::BorrowedLabel<'_>;
    value = __terrane_raised(
        borrowed_label(&*state),
        14 /* terrane-site: src/main.trn:61:13-61:34 */,
    );
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(0_i128)
    {
        value = exact_inner_label(state);
    }
    return value;
}
fn exact_reassign_alias<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    let value: witness::BorrowedLabel<'_>;
    let mut alias: witness::BorrowedLabel<'_>;
    value = __terrane_raised(
        borrowed_label(&*state),
        15 /* terrane-site: src/main.trn:67:13-67:34 */,
    );
    alias = value;
    let _ = &alias;
    alias = exact_inner_label(state);
    return alias;
}
fn exact_bound_wrapped<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let value: witness::BorrowedLabel<'_>;
    value = exact_inner_label(state);
    return __terrane_raised(
        borrowed_label_group(value),
        16 /* terrane-site: src/main.trn:74:12-74:39 */,
    );
}
fn exact_mixed_branch<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    let value: witness::BorrowedLabel<'_>;
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(1_i128)
    {
        value = exact_inner_label(state);
        return value;
    }
    return __terrane_raised(
        borrowed_label(&*state),
        17 /* terrane-site: src/main.trn:80:12-80:33 */,
    );
}
fn exact_helper_aggregate<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let mut children: terrane_collection_support::List<_>;
    children = terrane_collection_support::List::<_>::new(vec![]);
    {
        let __terrane_collection_argument_0 = exact_inner_label(state);
        children.push_unique(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            borrowed_label(&*state),
            18 /* terrane-site: src/main.trn:85:23-85:44 */,
        );
        children.push_unique(__terrane_collection_argument_0);
    };
    return __terrane_raised(
        borrowed_row(children),
        19 /* terrane-site: src/main.trn:86:12-86:34 */,
    );
}
fn exact_branch<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let intermediate: witness::BorrowedLabel<'_>;
    let button_intermediate: witness::BorrowedButton<'_>;
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(1_i128)
    {
        intermediate = __terrane_raised(
            borrowed_label(&*state),
            20 /* terrane-site: src/main.trn:90:24-90:45 */,
        );
        return __terrane_raised(
            borrowed_label_group(intermediate),
            21 /* terrane-site: src/main.trn:91:16-91:50 */,
        );
    }
    button_intermediate = __terrane_raised(
        borrowed_button(&*state),
        22 /* terrane-site: src/main.trn:92:27-92:49 */,
    );
    return __terrane_raised(
        borrowed_button_group(button_intermediate),
        23 /* terrane-site: src/main.trn:93:12-93:54 */,
    );
}
fn exact_aggregate<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let mut children: terrane_collection_support::List<_>;
    children = terrane_collection_support::List::<_>::new(vec![]);
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            borrowed_label(&*state),
            24 /* terrane-site: src/main.trn:97:23-97:44 */,
        );
        children.push_unique(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            borrowed_label(&*state),
            25 /* terrane-site: src/main.trn:98:23-98:44 */,
        );
        children.push_unique(__terrane_collection_argument_0);
    };
    return __terrane_raised(
        borrowed_row(children),
        26 /* terrane-site: src/main.trn:99:12-99:34 */,
    );
}
fn direct(state: &i64) -> Label {
    return __terrane_raised(
        label(
            terrane_int_support::Int::from(
                __terrane_raised(
                    terrane_int_support::fixed_addition(state.clone(), 1),
                    27 /* terrane-site: src/main.trn:102:19-102:28 */,
                ) as i128,
            ),
        ),
        28 /* terrane-site: src/main.trn:102:12-102:28 */,
    );
}
fn helper(value: i64) -> Label {
    return __terrane_raised(
        label(terrane_int_support::Int::from(value as i128)),
        29 /* terrane-site: src/main.trn:105:12-105:24 */,
    );
}
fn through_helper(state: &i64) -> Label {
    return helper(
        __terrane_raised(
            terrane_int_support::fixed_addition(state.clone(), 1),
            30 /* terrane-site: src/main.trn:108:20-108:29 */,
        ),
    );
}
fn nested(state: &i64) -> Widget {
    return {
        let __terrane_native_input_0 = __terrane_raised(
            button(
                terrane_int_support::Int::from(
                    __terrane_raised(
                        terrane_int_support::fixed_addition(state.clone(), 1),
                        31 /* terrane-site: src/main.trn:111:34-111:43 */,
                    ) as i128,
                ),
            ),
            32 /* terrane-site: src/main.trn:111:26-111:43 */,
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::into_widget(
                    __terrane_native_input_0,
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::into_widget",
                        ),
                    )
                }
            },
            33 /* terrane-site: src/main.trn:111:12-111:44 */,
        )
    };
}
fn branch(state: &i64) -> Widget {
    if state.clone() > 0 {
        return {
            let __terrane_native_input_0 = __terrane_raised(
                label(
                    terrane_int_support::Int::from(
                        __terrane_raised(
                            terrane_int_support::fixed_addition(state.clone(), 0),
                            34 /* terrane-site: src/main.trn:115:37-115:46 */,
                        ) as i128,
                    ),
                ),
                35 /* terrane-site: src/main.trn:115:30-115:46 */,
            );
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_native_input_0,
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::into_widget",
                            ),
                        )
                    }
                },
                36 /* terrane-site: src/main.trn:115:16-115:47 */,
            )
        };
    }
    return {
        let __terrane_native_input_0 = __terrane_raised(
            button(
                terrane_int_support::Int::from(
                    __terrane_raised(
                        terrane_int_support::fixed_addition(state.clone(), 7),
                        37 /* terrane-site: src/main.trn:116:34-116:43 */,
                    ) as i128,
                ),
            ),
            38 /* terrane-site: src/main.trn:116:26-116:43 */,
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::into_widget(
                    __terrane_native_input_0,
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::into_widget",
                        ),
                    )
                }
            },
            39 /* terrane-site: src/main.trn:116:12-116:44 */,
        )
    };
}
fn aggregate(state: &i64) -> Widget {
    let mut children: terrane_collection_support::List<Widget>;
    let _ = &state;
    children = terrane_collection_support::List::<Widget>::new(vec![]);
    {
        let __terrane_collection_argument_0 = {
            let __terrane_native_input_0 = __terrane_raised(
                traced_label(terrane_int_support::Int::from(1_i128)),
                40 /* terrane-site: src/main.trn:120:37-120:52 */,
            );
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_native_input_0,
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::into_widget",
                            ),
                        )
                    }
                },
                41 /* terrane-site: src/main.trn:120:23-120:53 */,
            )
        };
        children.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = {
            let __terrane_native_input_0 = __terrane_raised(
                traced_label(terrane_int_support::Int::from(2_i128)),
                42 /* terrane-site: src/main.trn:121:37-121:52 */,
            );
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_native_input_0,
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::into_widget",
                            ),
                        )
                    }
                },
                43 /* terrane-site: src/main.trn:121:23-121:53 */,
            )
        };
        children.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = {
            let __terrane_native_input_0 = __terrane_raised(
                traced_label(terrane_int_support::Int::from(3_i128)),
                44 /* terrane-site: src/main.trn:122:37-122:52 */,
            );
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_native_input_0,
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::into_widget",
                            ),
                        )
                    }
                },
                45 /* terrane-site: src/main.trn:122:23-122:53 */,
            )
        };
        children.append(__terrane_collection_argument_0);
    };
    return {
        let __terrane_native_input_0 = __terrane_raised(
            row(children),
            46 /* terrane-site: src/main.trn:123:26-123:39 */,
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::into_widget(
                    __terrane_native_input_0,
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::into_widget",
                        ),
                    )
                }
            },
            47 /* terrane-site: src/main.trn:123:12-123:40 */,
        )
    };
}
#[derive(Clone)]
pub struct DemoApplication {}
impl DemoApplication {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn title(&self) -> String {
        return String::from("ui");
    }
}
impl ApplicationProtocol for DemoApplication {
    fn clone_box(&self) -> Box<dyn ApplicationProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ApplicationProtocol> {
        Box::new(self.clone())
    }
    fn title(&self) -> String {
        DemoApplication::title(&*self)
    }
}
impl From<DemoApplication> for Application {
    fn from(value: DemoApplication) -> Self {
        Self(Box::new(value))
    }
}
impl witness::Application for DemoApplication {
    fn title(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DemoApplication>::title(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
fn borrowed_title<'view>(title: &'view String) -> witness::BorrowedLabel<'view> {
    return __terrane_raised(
        borrowed_label(&*title),
        48 /* terrane-site: src/main.trn:130:12-130:33 */,
    );
}
fn boot() -> MutableState {
    return __terrane_raised(
        terrane_static_trn_4d757461626c655374617465_new(String::from("ui")),
        49 /* terrane-site: src/main.trn:133:12-133:35 */,
    );
}
fn update(state: &mut MutableState, message: terrane_int_support::Int) {
    if message == terrane_int_support::Int::from(1_i128) {
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = (*state).mark();
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::MutableState::mark",
                        ),
                    )
                }
            },
            50 /* terrane-site: src/main.trn:137:9-137:20 */,
        );
    }
}
fn application_view<'view>(state: &'view MutableState) -> witness::BorrowedLabel<'view> {
    return __terrane_raised(
        mutable_state_label(&*state),
        51 /* terrane-site: src/main.trn:140:12-140:38 */,
    );
}
fn application_title(state: &MutableState) -> String {
    let _ = &state;
    return String::from("ui");
}
fn main() {
    let state: i64;
    let exact_state: String;
    let offset: i64;
    let captured: std::sync::Arc<dyn Fn(&i64) -> Label + Send + Sync>;
    let zero: i64;
    let application: DemoApplication;
    state = 40;
    exact_state = String::from("40");
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(direct); move | callback_argument_0 : &'_ i64 | { match | |
        -> Result < _, crate ::TerraneForeignError > { let callback_value = callback(&*
        callback_argument_0); Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_scoped")) },
        52 /* terrane-site: src/main.trn:148:13-148:58 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact")) }, 53 /* terrane-site: src/main.trn:149:13-149:62 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_typed))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact")) }, 54 /* terrane-site: src/main.trn:150:13-150:68 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_default:: < i64, String, bool > (&state, match | | -> Result < _,
        crate ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(direct);
        move | callback_argument_0 : &'_ i64 | { match | | -> Result < _, crate
        ::TerraneForeignError > { let callback_value = callback(&* callback_argument_0);
        Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_default")) },
        55 /* terrane-site: src/main.trn:151:13-151:42 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_nested))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact")) }, 56 /* terrane-site: src/main.trn:152:13-152:69 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_branch))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact")) }, 57 /* terrane-site: src/main.trn:153:13-153:69 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_aggregate)))
        { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact")) }, 58 /* terrane-site: src/main.trn:154:13-154:72 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_recursive)))
        { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact")) }, 59 /* terrane-site: src/main.trn:155:13-155:72 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_wrapped_binding))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        60 /* terrane-site: src/main.trn:156:13-156:78 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_ping))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact")) }, 61 /* terrane-site: src/main.trn:157:13-157:67 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_outer_group))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        62 /* terrane-site: src/main.trn:158:13-158:74 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_forwarded_binding))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        63 /* terrane-site: src/main.trn:159:13-159:80 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_bound_wrapped))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        64 /* terrane-site: src/main.trn:160:13-160:76 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_mixed_branch))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        65 /* terrane-site: src/main.trn:161:13-161:75 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_helper_aggregate))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        66 /* terrane-site: src/main.trn:162:13-162:79 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_reassign_helper_then_producer))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        67 /* terrane-site: src/main.trn:163:13-163:92 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_reassign_reverse))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        68 /* terrane-site: src/main.trn:164:13-164:79 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_reassign_conditional))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        69 /* terrane-site: src/main.trn:165:13-165:83 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_reassign_alias))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_exact")) },
        70 /* terrane-site: src/main.trn:166:13-166:77 */))
    );
    offset = 2;
    captured = {
        let offset = offset.clone();
        std::sync::Arc::new(move |state: &i64| -> Label {
            return __terrane_raised(
                label(
                    terrane_int_support::Int::from(
                        __terrane_raised(
                            terrane_int_support::fixed_addition(state.clone(), offset),
                            71 /* terrane-site: src/main.trn:169:23-169:37 */,
                        ) as i128,
                    ),
                ),
                72 /* terrane-site: src/main.trn:169:16-169:37 */,
            );
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = captured.clone(); move | callback_argument_0 : &'_ i64 | { match | | -> Result
        < _, crate ::TerraneForeignError > { let callback_value = callback(&*
        callback_argument_0); Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_scoped")) },
        73 /* terrane-site: src/main.trn:170:13-170:60 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(through_helper); move | callback_argument_0 : &'_ i64 | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(&* callback_argument_0); Ok(callback_value) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_scoped")) },
        74 /* terrane-site: src/main.trn:171:13-171:66 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(nested); move | callback_argument_0 : &'_ i64 | { match | |
        -> Result < _, crate ::TerraneForeignError > { let callback_value = callback(&*
        callback_argument_0); Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_scoped")) },
        75 /* terrane-site: src/main.trn:172:13-172:58 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(branch); move | callback_argument_0 : &'_ i64 | { match | |
        -> Result < _, crate ::TerraneForeignError > { let callback_value = callback(&*
        callback_argument_0); Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_scoped")) },
        76 /* terrane-site: src/main.trn:173:13-173:58 */))
    );
    zero = 0;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&zero, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(branch); move | callback_argument_0 : &'_ i64 | { match | |
        -> Result < _, crate ::TerraneForeignError > { let callback_value = callback(&*
        callback_argument_0); Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_scoped")) },
        77 /* terrane-site: src/main.trn:175:13-175:57 */))
    );
    __terrane_raised(reset_trace(), 78 /* terrane-site: src/main.trn:176:5-176:17 */);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(aggregate); move | callback_argument_0 : &'_ i64 | { match
        | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(&* callback_argument_0); Ok(callback_value) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_scoped")) },
        79 /* terrane-site: src/main.trn:177:13-177:61 */))
    );
    application = DemoApplication::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(launch(application,
        borrowed_title), 80 /* terrane-site: src/main.trn:179:13-179:48 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::run_application:: < witness::MutableState, _, _, _, _ > (match | | ->
        Result < _, crate ::TerraneForeignError > { Ok({ let callback =
        std::sync::Arc::new(boot); move | | { match | | -> Result < _, crate
        ::TerraneForeignError > { let callback_value = callback(); Ok(callback_value) }
        () { Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } } }) } ()
        { Ok(value) => value, Err(error) => std::panic::panic_any(error) }, match | | ->
        Result < _, crate ::TerraneForeignError > { Ok({ let callback =
        std::sync::Arc::new(update); move | callback_argument_0 : &mut
        witness::MutableState, callback_argument_1 : i64 | { match | | -> Result < _,
        crate ::TerraneForeignError > { let callback_value = callback(&mut *
        callback_argument_0,
        terrane_int_support::Int::from(i128::from(callback_argument_1)));
        Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }, application_view, match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback =
        std::sync::Arc::new(application_title); move | callback_argument_0 :
        &witness::MutableState | { match | | -> Result < _, crate ::TerraneForeignError >
        { let callback_value = callback(&* callback_argument_0); Ok(callback_value) } ()
        { Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } } }) } () {
        Ok(value) => value, Err(error) => std::panic::panic_any(error) }))) { Ok(value)
        => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::run_application")) }, 81 /* terrane-site: src/main.trn:180:13-180:79 */))
    );
}
