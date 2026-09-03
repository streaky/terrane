// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
fn apply(callback: std::sync::Arc<dyn Fn() -> String + Send + Sync>) -> String {
    return callback();
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&apply(std::sync::Arc::new(render_terrane_f1)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&apply(std::sync::Arc::new(render_terrane_f2)))
    );
}
// Source: left/value.trn
// Namespace: left
fn render_terrane_f1() -> String {
    return String::from("left");
}
// Source: right/value.trn
// Namespace: right
fn render_terrane_f2() -> String {
    return String::from("right");
}
