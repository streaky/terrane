// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: loop-shadowing-scope
fn value() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn main() {
    let items: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("ab"), String::from("c")]);
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &items,
    );
    loop {
        let items = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&items));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(items
        .length()))
    );
    let groups: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    > = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)]))
        ],
    );
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &groups,
    );
    loop {
        let entry = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
            &entry.value.clone(),
        );
        loop {
            let entry = match __terrane_iterator_2.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&entry));
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(groups
        .length()))
    );
    let numbers: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    if true {
        let numbers: String = String::from("ab");
        println!("{}", terrane_scalar_support::scalar_text(&numbers));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(numbers
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:24:10-24:20 */)), 0 /* terrane-site: case.trn:24:10-24:20 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&value()));
    let value: i64 = 5;
    let copy: terrane_int_support::Int = terrane_int_support::Int::from(value as i128);
    println!("{}", terrane_scalar_support::scalar_text(&copy));
    let _ = &value;
    let value: terrane_int_support::Int = terrane_int_support::Int::from(value as i128)
        + terrane_int_support::Int::from(1_i128);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
