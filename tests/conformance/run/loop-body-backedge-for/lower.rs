// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/loop-body-backedge-for
fn main() {
    let mut first: bool;
    let mut item: String;
    let mut previous: Option<String> = None;
    first = true;
    {
        let __terrane_iterable_0 = &String::from("abc");
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            item = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            if !first {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&previous.as_ref()
                    .unwrap_or_else(| | __terrane_uninitialized_binding("previous",
                    "case.trn", 8, 20)))
                );
            }
            let _ = previous.insert(item);
            first = false;
        }
    }
}
