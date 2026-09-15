// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-collection-support, terrane-scalar-support
// Source: case.trn
// Namespace: dead-store-warnings
fn main() {
    let mut value: i8 = 1;
    let _ = &mut value;
    value = 2;
    println!("{}", terrane_scalar_support::scalar_text(&value));
    let mut stale: i8 = 3;
    let _ = &mut stale;
    stale = 4;
    let _ = &mut stale;
    let __terrane_iterable_0 = String::from("ab");
    let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let ignored = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &ignored;
        println!("{}", terrane_scalar_support::scalar_text(&String::from("tick")));
    }
    let __terrane_iterable_1 = String::from("ab");
    let mut __terrane_iterator_1 = terrane_collection_support::string_iterator(
        &__terrane_iterable_1,
    );
    loop {
        let mut replaced = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &replaced;
        replaced = String::from("x");
        println!("{}", terrane_scalar_support::scalar_text(&replaced));
    }
    let __terrane_iterable_2 = String::from("c");
    let mut __terrane_iterator_2 = terrane_collection_support::string_iterator(
        &__terrane_iterable_2,
    );
    loop {
        let mut preserved = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&preserved));
        preserved = String::from("y");
        println!("{}", terrane_scalar_support::scalar_text(&preserved));
    }
    let __terrane_iterable_3 = String::from("a");
    let mut __terrane_iterator_3 = terrane_collection_support::string_iterator(
        &__terrane_iterable_3,
    );
    loop {
        let outer = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_iterable_4 = String::from("b");
        let mut __terrane_iterator_4 = terrane_collection_support::string_iterator(
            &__terrane_iterable_4,
        );
        loop {
            let mut inner = match __terrane_iterator_4.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &inner;
            inner = String::from("z");
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&outer),
                terrane_scalar_support::scalar_text(&inner)
            );
        }
    }
}
