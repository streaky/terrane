// Generated deterministically by Terrane <version>.
// Source: src/main.trn
// Namespace: annotation-source-free
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&describe_parent()));
}
// Source: .trn/generated/schema/typed-codec-_2fdeclaration-annotations_3a_3aparent.trn
// Namespace: generated/annotation-codec/declaration-annotations/parent
fn describe_parent() -> String {
    return String::from(
        "{\"description\":\"Configured schema description\",\"fields\":{\"field\":{\"description\":null,\"nullable\":false,\"required\":false,\"type\":\"string\"}}}",
    );
}
