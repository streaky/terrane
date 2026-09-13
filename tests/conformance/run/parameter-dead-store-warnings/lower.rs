// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: parameter-dead-store-warnings
fn take(mut v: terrane_collection_support::List<terrane_int_support::Int>) {
    let _ = &v;
    v = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(7_i128),
            terrane_int_support::Int::from(8_i128)
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(v
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:6:10-6:14 */)), 0 /* terrane-site: case.trn:6:10-6:14 */))
    );
}
fn scalar(mut v: terrane_int_support::Int) {
    let _ = &v;
    v = terrane_int_support::Int::from(4_i128);
    println!("{}", terrane_scalar_support::scalar_text(&v));
}
fn unused(v: terrane_collection_support::List<terrane_int_support::Int>) {
    let _ = &v;
    println!("{}", terrane_scalar_support::scalar_text(&String::from("ignored")));
}
fn optional(v: terrane_int_support::Int) {
    println!("{}", terrane_scalar_support::scalar_text(&v));
}
fn main() {
    take(
        terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(1_i128),
                terrane_int_support::Int::from(2_i128)
            ],
        ),
    );
    scalar(terrane_int_support::Int::from(3_i128));
    unused(
        terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(1_i128),
                terrane_int_support::Int::from(2_i128)
            ],
        ),
    );
    optional(terrane_int_support::Int::from(7_i128));
    optional(terrane_int_support::Int::from(5_i128));
}
