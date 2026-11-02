// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let moment: SystemTime;
    let rendered: String;
    moment = __terrane_raised(
        parse_http_date("Sun, 06 Nov 1994 08:49:37 GMT"),
        0 /* terrane-site: src/main.trn:4:14-4:61 */,
    );
    rendered = {
        let __terrane_conversion_input_0 = moment;
        __terrane_raised(
            fmt_http_date(__terrane_conversion_input_0),
            1 /* terrane-site: src/main.trn:5:23-5:49 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&rendered));
}
