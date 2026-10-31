// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/loop-body-backedge-inferred-rhs
fn main() {
    let mut first: bool;
    let mut current: String;
    let mut observed: String;
    let mut item: Option<String> = None;
    first = true;
    {
        let __terrane_iterable_0 = &String::from("abc");
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            current = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            if !first {
                observed = item
                    .as_ref()
                    .unwrap_or_else(|| __terrane_uninitialized_binding(
                        "item",
                        "case.trn",
                        8,
                        24,
                    ))
                    .clone();
                println!("{}", terrane_scalar_support::scalar_text(&observed));
            }
            let _ = item.insert(current);
            first = false;
        }
    }
}
