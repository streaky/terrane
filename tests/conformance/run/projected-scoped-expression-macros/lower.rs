// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn render<'view>(state: &'view State) -> witness::Element<'view, ()> {
    let mut children: terrane_collection_support::List<witness::Element<'_, ()>>;
    let mut child: &State;
    children = terrane_collection_support::List::<witness::Element<'_, ()>>::new(vec![]);
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
        let value: witness::Element<'_, ()> = witness::container(
                __terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > {
                                    Ok(
                                        witness::column!(
                                            witness::text!("macro {}", 7_i64),
                                            witness::row!(witness::text!("nested {}", 11_i64), { let
                                            __terrane_conversion_input_0 = __terrane_raised(| | ->
                                            Result < _, crate ::TerraneForeignError > { { let value =
                                            match
                                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                                            Result < _, crate ::TerraneForeignError > { Ok((&* state)
                                            .get_label()) })) { Ok(result) => result ?, Err(payload) =>
                                            return Err(crate ::__terrane_dependency_panic(payload,
                                            "witness", "witness::State::get_label")) }; Ok:: < _, crate
                                            ::TerraneForeignError > (value.to_owned()) } } (),
                                            0 /* terrane-site: src/main.trn:9:134-9:150 */);
                                            __terrane_raised(| | -> Result < _, crate
                                            ::TerraneForeignError > { { let value = match
                                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                                            Result < _, crate ::TerraneForeignError > {
                                            Ok(witness::text(__terrane_conversion_input_0)) })) {
                                            Ok(result) => result ?, Err(payload) => return Err(crate
                                            ::__terrane_dependency_panic(payload, "witness",
                                            "witness::text")) }; Ok:: < _, crate ::TerraneForeignError >
                                            (value) } } (), 1 /* terrane-site: src/main.trn:9:118-9:151 */) }), witness::row(children
                                            .into_unique_vec().into_iter().map(Into::into).collect:: <
                                            Vec < _ > > ())
                                        ),
                                    )
                                }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "witness",
                                            "witness::column",
                                        ),
                                    );
                                }
                            };
                            Ok::<_, crate::TerraneForeignError>(value)
                        }
                    }(),
                    2 /* terrane-site: src/main.trn:9:31-9:179 */,
                ),
            )
            .into();
        value
    };
}
fn main() {
    let mut children: terrane_collection_support::List<State>;
    let state: State;
    children = terrane_collection_support::List::<State>::new(vec![]);
    {
        let __terrane_collection_argument_0 = {
            let __terrane_conversion_input_0 = String::from("first borrowed leaf");
            __terrane_raised(
                terrane_static_trn_5374617465_new(__terrane_conversion_input_0),
                3 /* terrane-site: src/main.trn:12:23-12:56 */,
            )
        };
        children.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = {
            let __terrane_conversion_input_0 = String::from("second borrowed leaf");
            __terrane_raised(
                terrane_static_trn_5374617465_new(__terrane_conversion_input_0),
                4 /* terrane-site: src/main.trn:13:23-13:57 */,
            )
        };
        children.append(__terrane_collection_argument_0);
    };
    state = {
        let __terrane_conversion_input_0 = String::from("borrowed root");
        let __terrane_conversion_input_1 = children;
        __terrane_raised(
            terrane_static_trn_5374617465_branch(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            5 /* terrane-site: src/main.trn:14:13-14:53 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(inspect(&state,
        render), 6 /* terrane-site: src/main.trn:15:13-15:35 */))
    );
}
