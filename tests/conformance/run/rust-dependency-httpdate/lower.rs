// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
fn main() {
    let moment: SystemTime = __terrane_raised(
        parse_http_date(String::from("Sun, 06 Nov 1994 08:49:37 GMT")),
        0 /* terrane-site: src/main.trn:4:14-4:61 */,
    );
    let rendered: String = __terrane_raised(
        fmt_http_date(moment),
        1 /* terrane-site: src/main.trn:5:23-5:49 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&rendered));
}
// Source: <terrane>/projected/deps/date-codec.trn
// Namespace: deps/date-codec
pub use std::time::SystemTime;
pub fn fmt_http_date(d: SystemTime) -> Result<String, crate::TerraneForeignError> {
    let d = d;
    let value = date_codec::fmt_http_date(d);
    Ok(value)
}
pub fn parse_http_date(s: String) -> Result<SystemTime, crate::TerraneForeignError> {
    let s = s;
    match date_codec::parse_http_date(&s) {
        Ok(value) => Ok(value),
        Err(error) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `date-codec` member `date_codec::parse_http_date` failed: {error}"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
    }
}
