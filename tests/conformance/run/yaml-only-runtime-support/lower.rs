// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let parsed: DocumentResult;
    let name: DocumentResult;
    parsed = parse_yaml(String::from("name: Terrane"), default_yaml_options());
    name = parsed.value.field(String::from("name"));
    println!("{}", terrane_scalar_support::scalar_text(&name.value.scalar));
}
