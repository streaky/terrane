// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: integer-euclidean
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).euclidean_div(&terrane_int_support::Int::from(3_i128)),
        0 /* terrane-site: case.trn:4:10-4:16 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(7_i128)
        .euclidean_div(&terrane_int_support::Int::from(- 3_i128)), 1 /* terrane-site: case.trn:5:10-5:16 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).modulo(&terrane_int_support::Int::from(3_i128)), 2 /* terrane-site: case.trn:6:10-6:16 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(7_i128)
        .modulo(&terrane_int_support::Int::from(- 3_i128)), 3 /* terrane-site: case.trn:7:10-7:16 */))
    );
}
