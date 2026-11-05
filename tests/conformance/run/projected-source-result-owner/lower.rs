// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: helpers/maker.trn
// Namespace: helpers
fn make_terrane_helpers() -> Holder {
    return __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: helpers/maker.trn:6:10-6:17 */,
    );
}
// Source: src/main.trn
// Namespace: app
fn same(h: Holder) -> Holder {
    return h;
}
fn identity<TerraneType54>(value: TerraneType54) -> TerraneType54 {
    return value;
}
fn selected<TerraneType54, TerraneType55>(
    ignored: TerraneType55,
    value: TerraneType54,
) -> TerraneType54 {
    let _ = &ignored;
    return value;
}
fn make_terrane_app() -> Holder {
    return __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        1 /* terrane-site: src/main.trn:16:10-16:22 */,
    );
}
fn with_text(text: String) -> Holder {
    let _ = &text;
    return __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        2 /* terrane-site: src/main.trn:19:10-19:22 */,
    );
}
async fn async_make() -> Holder {
    return __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        3 /* terrane-site: src/main.trn:22:10-22:22 */,
    );
}
#[derive(Clone)]
pub struct Maker {}
impl Maker {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn make(&self) -> Holder {
        return __terrane_raised(
            terrane_static_trn_486f6c646572_new(),
            4 /* terrane-site: src/main.trn:26:12-26:24 */,
        );
    }
}
fn main() {
    __terrane_run(async move {
        let a: Holder;
        let b: terrane_query_witness::Holder;
        let c: Holder;
        let d: Holder;
        let e: Holder;
        let maker: Maker;
        let f: Holder;
        let g: terrane_query_witness::Holder;
        let h: Holder;
        a = same(
            __terrane_raised(
                terrane_static_trn_486f6c646572_new(),
                5 /* terrane-site: src/main.trn:29:14-29:26 */,
            ),
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&a).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 6 /* terrane-site: src/main.trn:30:11-30:23 */))
        );
        b = identity(
            __terrane_raised(
                terrane_static_trn_486f6c646572_new(),
                7 /* terrane-site: src/main.trn:31:18-31:30 */,
            ),
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&b).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 8 /* terrane-site: src/main.trn:32:11-32:23 */))
        );
        c = make_terrane_app();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&c).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 9 /* terrane-site: src/main.trn:34:11-34:23 */))
        );
        d = with_text(String::from("abc"));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&d).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 10 /* terrane-site: src/main.trn:36:11-36:23 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&make_terrane_app()).label_len()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 11 /* terrane-site: src/main.trn:37:11-37:29 */))
        );
        e = __terrane_await(async_make()).await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&e).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 12 /* terrane-site: src/main.trn:39:11-39:23 */))
        );
        maker = Maker::terrane_construct();
        f = maker.make();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&f).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 13 /* terrane-site: src/main.trn:42:11-42:23 */))
        );
        g = selected(
            __terrane_raised(
                terrane_static_trn_4f74686572_new(),
                15 /* terrane-site: src/main.trn:43:52-43:63 */,
            ),
            __terrane_raised(
                terrane_static_trn_486f6c646572_new(),
                14 /* terrane-site: src/main.trn:43:26-43:38 */,
            ),
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&g).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 16 /* terrane-site: src/main.trn:44:11-44:23 */))
        );
        h = make_terrane_helpers();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 17 /* terrane-site: src/main.trn:46:11-46:23 */))
        );
    });
}
