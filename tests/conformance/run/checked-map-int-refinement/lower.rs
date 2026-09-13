// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: checked-map-int-refinement
fn main() {
    let mut counts: terrane_collection_support::Map<String, terrane_int_support::Int> = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("apple"),
            terrane_int_support::Int::from(1_i128))
        ],
    );
    let current: Option<terrane_int_support::Int> = counts
        .get(&String::from("apple"))
        .cloned();
    if current.is_some() {
        let _ = counts
            .set(
                String::from("apple"),
                (*current.as_ref().expect("semantic optional narrowing")).clone()
                    + terrane_int_support::Int::from(1_i128),
            );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(counts
        .get_or_error(&String::from("apple")), 0 /* terrane-site: case.trn:10:10-10:25 */))
    );
}
