// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: negated-presence-narrowing
fn main() {
    let value: Option<i8> = Some(7);
    if !value.is_none() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
}
