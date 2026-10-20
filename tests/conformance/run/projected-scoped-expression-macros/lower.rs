// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn render<'view>(state: &'view State) -> witness::Element<'view, ()> {
    let mut children: terrane_collection_support::List<_>;
    let mut child: &State;
    children = terrane_collection_support::List::<_>::new(vec![]);
    let __terrane_iterable_0 = &state.children;
    let mut __terrane_iterator_0 = __terrane_iterable_0.iter();
    loop {
        child = match __terrane_iterator_0.next() {
            Some(item) => item,
            None => break,
        };
        {
            let __terrane_collection_argument_0 = render(child);
            children.push_unique(__terrane_collection_argument_0);
        };
    }
    return {
        let value: witness::Element<'_, ()> = {
            let __terrane_native_input_0 = match || -> Result<
                _,
                crate::TerraneForeignError,
            > {
                Ok(
                    __terrane_raised(
                        match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| {
                                witness::column!(
                                    witness::text!("macro {}", 7_i64),
                                    witness::row!(witness::text!("nested {}", 11_i64), { let
                                    __terrane_native_input_0 = match | | -> Result < _, crate
                                    ::TerraneForeignError > { Ok(__terrane_raised(match
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                    (&state).get_label())) { Ok(value) => Ok(value.to_owned()),
                                    Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload, "witness",
                                    "witness::State::get_label")) }, 0 /* terrane-site: src/main.trn:9:134-9:150 */)) } () { Ok(value) => value,
                                    Err(error) => std::panic::panic_any(error) };
                                    __terrane_raised(match
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                    witness::text(__terrane_native_input_0))) { Ok(value) =>
                                    Ok(value), Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload, "witness",
                                    "witness::text")) }, 1 /* terrane-site: src/main.trn:9:118-9:151 */) }), __terrane_raised(match
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                    witness::row:: < _ > (children.into_unique_vec().into_iter()
                                    .map(Into::into).collect:: < Vec < _ > > ()))) { Ok(value)
                                    => Ok(value), Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload, "witness",
                                    "witness::row")) }, 2 /* terrane-site: src/main.trn:9:156-9:178 */)
                                )
                            }),
                        ) {
                            Ok(value) => Ok(value),
                            Err(payload) => {
                                Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "witness",
                                        "witness::column",
                                    ),
                                )
                            }
                        },
                        3 /* terrane-site: src/main.trn:9:31-9:179 */,
                    ),
                )
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error),
            };
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::container::<
                        _,
                    >(__terrane_native_input_0)),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::container",
                            ),
                        )
                    }
                },
                4 /* terrane-site: src/main.trn:9:12-9:180 */,
            )
        }
            .into();
        value
    };
}
fn main() {
    let mut children: terrane_collection_support::List<State>;
    let state: State;
    children = terrane_collection_support::List::<State>::new(vec![]);
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            terrane_static_trn_5374617465_new(String::from("first borrowed leaf")),
            5 /* terrane-site: src/main.trn:12:23-12:56 */,
        );
        children.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            terrane_static_trn_5374617465_new(String::from("second borrowed leaf")),
            6 /* terrane-site: src/main.trn:13:23-13:57 */,
        );
        children.append(__terrane_collection_argument_0);
    };
    state = __terrane_raised(
        terrane_static_trn_5374617465_branch(String::from("borrowed root"), children),
        7 /* terrane-site: src/main.trn:14:13-14:53 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(inspect(&state,
        render), 8 /* terrane-site: src/main.trn:15:13-15:35 */))
    );
}
