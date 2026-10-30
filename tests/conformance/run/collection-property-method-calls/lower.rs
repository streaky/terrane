// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: collection-property-method-calls
fn main() {
    let values: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut pair: terrane_collection_support::Entry<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut appended: terrane_collection_support::List<terrane_int_support::Int>;
    let mut sorted: terrane_collection_support::List<terrane_int_support::Int>;
    values = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("alpha"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i128)]))
        ],
    );
    let __terrane_iterable_0 = {
        let __terrane_result = values.entries();
        __terrane_result
    };
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        pair = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        appended = {
            let __terrane_result = {
                let __terrane_value = terrane_int_support::Int::from(2_i128);
                let collection = &mut pair.value.clone();
                collection.append(__terrane_value);
                collection.clone()
            };
            __terrane_result
        };
        sorted = {
            let __terrane_result = {
                let collection = &mut pair.value.clone();
                collection.sort_by(|left, right| left.cmp(right));
                collection.clone()
            };
            __terrane_result
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
            &pair.key.clone(); let __terrane_string_argument_0 = "al";
            __terrane_receiver_value_0.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(appended
            .length())), terrane_scalar_support::scalar_text(&__terrane_raised(appended
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
            0 /* terrane-site: case.trn:10:29-10:40 */)), 0 /* terrane-site: case.trn:10:29-10:40 */))
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(sorted
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            1 /* terrane-site: case.trn:11:12-11:21 */)), 1 /* terrane-site: case.trn:11:12-11:21 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(sorted
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: case.trn:11:23-11:32 */)), 2 /* terrane-site: case.trn:11:23-11:32 */))
        );
    }
}
