// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: references-provenance
fn main() {
    let values: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    > = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        ),
    );
    let owner: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    > = values.clone();
    let observer: std::sync::Weak<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    > = std::sync::Arc::downgrade(&values.clone());
    owner
        .lock()
        .expect("shared reference lock poisoned")
        .append(terrane_int_support::Int::from(2_i128));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = values.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised({ let __terrane_owner =
        observer.upgrade().expect("reference expired"); let __terrane_value =
        __terrane_owner.lock().expect("reference lock poisoned").clone(); __terrane_value
        }
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:10:25-10:36 */)), 0 /* terrane-site: case.trn:10:25-10:36 */))
    );
    let owned: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(9_i128)]);
    let transferred: terrane_collection_support::List<terrane_int_support::Int> = owned;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(transferred
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:13:10-13:24 */)), 1 /* terrane-site: case.trn:13:10-13:24 */))
    );
}
