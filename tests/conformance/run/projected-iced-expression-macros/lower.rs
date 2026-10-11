// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn render<'view>(state: &'view State) -> iced::Element<'view, ()> {
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
        children.push_unique(render(child));
    }
    return {
        let value: iced::Element<'_, ()> = __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| iced::widget::container::<
                        _,
                        _,
                        _,
                    >(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                __terrane_raised(
                                    match std::panic::catch_unwind(
                                        std::panic::AssertUnwindSafe(|| {
                                            iced::widget::column!(
                                                iced::widget::text!("macro {}", 7_i64),
                                                iced::widget::row!(iced::widget::text!("nested {}", 11_i64),
                                                __terrane_raised(match
                                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                                iced::widget::text:: < _, _ > (match | | -> Result < _,
                                                crate ::TerraneForeignError > { Ok(__terrane_raised(match
                                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                                (&state).get_label())) { Ok(value) => Ok(value.to_owned()),
                                                Err(payload) => Err(crate
                                                ::__terrane_dependency_panic(payload, "witness",
                                                "witness::State::get_label")) }, 0 /* terrane-site: src/main.trn:10:134-10:150 */)) } () { Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error) }))) { Ok(value)
                                                => Ok(value), Err(payload) => Err(crate
                                                ::__terrane_dependency_panic(payload, "iced",
                                                "iced::widget::text::<_, _>")) }, 1 /* terrane-site: src/main.trn:10:118-10:151 */)), __terrane_raised(match
                                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                                iced::widget::row:: < _, _, _ > (children.into_unique_vec()
                                                .into_iter().map(Into::into).collect:: < Vec < _ > > ()))) {
                                                Ok(value) => Ok(value), Err(payload) => Err(crate
                                                ::__terrane_dependency_panic(payload, "iced",
                                                "iced::widget::row::<_, _, _>")) }, 2 /* terrane-site: src/main.trn:10:156-10:178 */)
                                            )
                                        }),
                                    ) {
                                        Ok(value) => Ok(value),
                                        Err(payload) => {
                                            Err(
                                                crate::__terrane_dependency_panic(
                                                    payload,
                                                    "iced",
                                                    "iced::widget::column",
                                                ),
                                            )
                                        }
                                    },
                                    3 /* terrane-site: src/main.trn:10:31-10:179 */,
                                ),
                            )
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "iced",
                                "iced::widget::container::<_, _, _>",
                            ),
                        )
                    }
                },
                4 /* terrane-site: src/main.trn:10:12-10:180 */,
            )
            .into();
        value
    };
}
fn main() {
    let mut children: terrane_collection_support::List<State>;
    let state: State;
    children = terrane_collection_support::List::<State>::new(vec![]);
    children
        .append(
            __terrane_raised(
                terrane_static_trn_5374617465_new(String::from("first borrowed leaf")),
                5 /* terrane-site: src/main.trn:13:23-13:56 */,
            ),
        );
    children
        .append(
            __terrane_raised(
                terrane_static_trn_5374617465_new(String::from("second borrowed leaf")),
                6 /* terrane-site: src/main.trn:14:23-14:57 */,
            ),
        );
    state = __terrane_raised(
        terrane_static_trn_5374617465_branch(String::from("borrowed root"), children),
        7 /* terrane-site: src/main.trn:15:13-15:53 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(inspect(&state,
        render), 8 /* terrane-site: src/main.trn:16:13-16:35 */))
    );
}
