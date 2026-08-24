// Generated deterministically by Terrane <version>.
struct TerraneCapability;
// Source: app/main.trn
// Namespace: host-capability-authority
fn emit(authority: TerraneCapability) {
    let _ = &authority;
    println!("{}", terrane_scalar_support::scalar_text(&(String::from("host authority"))));
}
fn __terrane_main(authority: TerraneCapability) {
    emit(authority);
}
fn main() {
    __terrane_main(TerraneCapability);
}
