// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn present(value: witness::Opaque<String>) -> bool {
    return __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| value.into_value()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Opaque<T>::into_value",
                        ),
                    )
                }
            },
            0 /* terrane-site: src/main.trn:6:13-6:30 */,
        )
        .as_str() == "text";
}
fn main() {
    let text: witness::Opaque<String>;
    let flag: witness::Opaque<bool>;
    let defaulted: witness::Opaque<String>;
    let nested: witness::Opaque<witness::Opaque<bool>>;
    let aggregate: terrane_collection_support::Tuple<bool>;
    let selected: terrane_collection_support::Set<bool>;
    let mapping: terrane_collection_support::Map<String, bool>;
    let defaulted_pair: witness::Pair<String, String>;
    let inner: witness::Opaque<bool>;
    text = __terrane_raised(
        make_text(),
        1 /* terrane-site: src/main.trn:9:29-9:39 */,
    );
    flag = __terrane_raised(
        make_flag(),
        2 /* terrane-site: src/main.trn:10:27-10:37 */,
    );
    defaulted = __terrane_raised(
        make_text(),
        3 /* terrane-site: src/main.trn:11:24-11:34 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&present(text)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | flag.into_value())) {
        Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::Opaque<T>::into_value")) }, 4 /* terrane-site: src/main.trn:13:13-13:29 */))
    );
    nested = __terrane_raised(
        make_nested(),
        5 /* terrane-site: src/main.trn:14:39-14:51 */,
    );
    aggregate = __terrane_raised(
        pair(),
        6 /* terrane-site: src/main.trn:15:17-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(pair_code(aggregate),
        7 /* terrane-site: src/main.trn:16:13-16:33 */))
    );
    selected = __terrane_raised(
        flags(),
        8 /* terrane-site: src/main.trn:17:16-17:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(flag_code(selected),
        9 /* terrane-site: src/main.trn:18:13-18:32 */))
    );
    mapping = __terrane_raised(
        lookup(),
        10 /* terrane-site: src/main.trn:19:15-19:22 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(mapping_value(mapping),
        11 /* terrane-site: src/main.trn:20:13-20:35 */))
    );
    defaulted_pair = __terrane_raised(
        default_pair(),
        12 /* terrane-site: src/main.trn:21:27-21:40 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(pair_text(defaulted_pair),
        13 /* terrane-site: src/main.trn:22:13-22:38 */))
    );
    inner = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| nested.into_value()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Opaque<T>::into_value",
                    ),
                )
            }
        },
        14 /* terrane-site: src/main.trn:23:28-23:46 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | inner.into_value())) {
        Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::Opaque<T>::into_value")) }, 15 /* terrane-site: src/main.trn:24:13-24:30 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | defaulted
        .into_value())) { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::Opaque<T>::into_value")) }, 16 /* terrane-site: src/main.trn:25:13-25:34 */))
    );
}
