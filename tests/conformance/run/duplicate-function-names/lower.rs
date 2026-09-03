// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
fn apply(callback: std::sync::Arc<dyn Fn() -> String + Send + Sync>) -> String {
    return callback();
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&apply(std::sync::Arc::new(render_terrane_left)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&apply(std::sync::Arc::new(render_terrane_right)))
    );
}
// Source: left/value.trn
// Namespace: left
fn render_terrane_left() -> String {
    return String::from("left");
}
// Source: right/value.trn
// Namespace: right
fn render_terrane_right() -> String {
    return String::from("right");
}
