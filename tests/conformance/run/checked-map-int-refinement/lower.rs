// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: checked-map-int-refinement
fn main() {
    let mut counts: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let current: Option<terrane_int_support::Int>;
    counts = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("apple"),
            terrane_int_support::Int::from(1_i128))
        ],
    );
    current = counts.get(&String::from("apple")).cloned();
    if (&current).is_some() {
        {
            let __terrane_key = String::from("apple");
            let __terrane_value = match &current {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .clone() + terrane_int_support::Int::from(1_i128);
            counts.set(__terrane_key, __terrane_value);
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(counts
        .get_or_error(&String::from("apple")), 0 /* terrane-site: case.trn:10:10-10:25 */))
    );
}
