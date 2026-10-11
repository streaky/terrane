// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: restricted-standard-capabilities
fn main() {
    let cell: IntMutex;
    let name: ProcessHostNameResult;
    cell = IntMutex::terrane_construct(terrane_int_support::Int::from(1_i128));
    println!("{}", terrane_scalar_support::scalar_text(&cell.failed));
    name = process_host_name();
    println!(
        "{}", terrane_scalar_support::scalar_text(&(name.failed || name.available))
    );
}
