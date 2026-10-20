// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let plain: PlainOwner;
    let defaulted: terrane_owner_witness::DefaultOwner<String>;
    let options: Options;
    let html: String;
    plain = __terrane_raised(
        terrane_static_trn_506c61696e4f776e6572_standard(),
        0 /* terrane-site: src/main.trn:7:11-7:32 */,
    );
    defaulted = __terrane_raised(
        terrane_static_trn_44656661756c744f776e6572_standard(),
        1 /* terrane-site: src/main.trn:8:15-8:38 */,
    );
    options = __terrane_raised(
        terrane_static_trn_4f7074696f6e73_gfm(),
        2 /* terrane-site: src/main.trn:9:13-9:26 */,
    );
    html = __terrane_raised(
        to_html_with_options(String::from("|a|\n|-|\n|b|"), &options),
        3 /* terrane-site: src/main.trn:10:17-10:63 */,
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&plain).label())) {
        Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_owner_witness",
        "terrane_owner_witness::PlainOwner::label")) }, 4 /* terrane-site: src/main.trn:11:11-11:23 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&defaulted).label()))
        { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_owner_witness",
        "terrane_owner_witness::DefaultOwner::label")) }, 5 /* terrane-site: src/main.trn:11:27-11:43 */)), terrane_scalar_support::scalar_text(&<
        terrane_owner_witness::DefaultOwner < std::string::String > > ::CODE),
        terrane_scalar_support::scalar_text(&html)
    );
}
