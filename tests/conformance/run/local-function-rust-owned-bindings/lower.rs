// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: local-function-rust-owned-bindings
fn main() {
    fn helper(parameter: terrane_int_support::Int) -> terrane_int_support::Int {
        let local: terrane_int_support::Int;
        local = parameter.clone() + terrane_int_support::Int::from(1_i128);
        return parameter + local;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&helper(terrane_int_support::Int::from(4_i128)))
    );
}
