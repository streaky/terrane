// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-collection-support, terrane-scalar-support
// Source: case.trn
// Namespace: dead-store-warnings
fn main() {
    let mut value_terrane_f0_s47: i8;
    let mut stale_terrane_f0_s91: i8;
    let mut ignored_terrane_f0_s124: String;
    let mut replaced_terrane_f0_s164: String;
    let mut preserved_terrane_f0_s226: String;
    let mut outer_terrane_f0_s311: String;
    let mut inner_terrane_f0_s332: String;
    value_terrane_f0_s47 = 1;
    let _ = &mut value_terrane_f0_s47;
    value_terrane_f0_s47 = 2;
    println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s47));
    stale_terrane_f0_s91 = 3;
    let _ = &mut stale_terrane_f0_s91;
    stale_terrane_f0_s91 = 4;
    let _ = &mut stale_terrane_f0_s91;
    let __terrane_iterable_0 = String::from("ab");
    let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(
        &__terrane_iterable_0,
    );
    loop {
        ignored_terrane_f0_s124 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &ignored_terrane_f0_s124;
        println!("{}", terrane_scalar_support::scalar_text(&String::from("tick")));
    }
    let __terrane_iterable_1 = String::from("ab");
    let mut __terrane_iterator_1 = terrane_collection_support::string_iterator(
        &__terrane_iterable_1,
    );
    loop {
        replaced_terrane_f0_s164 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &replaced_terrane_f0_s164;
        replaced_terrane_f0_s164 = String::from("x");
        println!("{}", terrane_scalar_support::scalar_text(&replaced_terrane_f0_s164));
    }
    let __terrane_iterable_2 = String::from("c");
    let mut __terrane_iterator_2 = terrane_collection_support::string_iterator(
        &__terrane_iterable_2,
    );
    loop {
        preserved_terrane_f0_s226 = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&preserved_terrane_f0_s226));
        preserved_terrane_f0_s226 = String::from("y");
        println!("{}", terrane_scalar_support::scalar_text(&preserved_terrane_f0_s226));
    }
    let __terrane_iterable_3 = String::from("a");
    let mut __terrane_iterator_3 = terrane_collection_support::string_iterator(
        &__terrane_iterable_3,
    );
    loop {
        outer_terrane_f0_s311 = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_iterable_4 = String::from("b");
        let mut __terrane_iterator_4 = terrane_collection_support::string_iterator(
            &__terrane_iterable_4,
        );
        loop {
            inner_terrane_f0_s332 = match __terrane_iterator_4.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &inner_terrane_f0_s332;
            inner_terrane_f0_s332 = String::from("z");
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&outer_terrane_f0_s311),
                terrane_scalar_support::scalar_text(&inner_terrane_f0_s332)
            );
        }
    }
}
