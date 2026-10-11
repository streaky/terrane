// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: system-adapter
fn main() {
    let name: ProcessHostNameResult;
    name = process_host_name();
    println!(
        "{}", terrane_scalar_support::scalar_text(&(name.failed || name.available))
    );
}
