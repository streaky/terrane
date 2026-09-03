// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: app/main.trn
// Namespace: app
fn main() {
    let left: TerraneNs4LeftResponse = TerraneNs4LeftResponse::terrane_construct();
    let right: TerraneNs5RightResponse = TerraneNs5RightResponse::terrane_construct();
    let left_value: String = left.render();
    let right_value: String = right.render();
    println!("{}", terrane_scalar_support::scalar_text(&left_value));
    println!("{}", terrane_scalar_support::scalar_text(&right_value));
}
// Source: left/response.trn
// Namespace: left
#[derive(Clone)]
pub struct TerraneNs4LeftResponse {
    pub value: String,
}
impl TerraneNs4LeftResponse {
    pub fn terrane_construct() -> Self {
        Self {
            value: String::from("left"),
        }
    }
    pub fn render(&self) -> String {
        return self.value.clone();
    }
}
// Source: right/response.trn
// Namespace: right
#[derive(Clone)]
pub struct TerraneNs5RightResponse {
    pub value: String,
}
impl TerraneNs5RightResponse {
    pub fn terrane_construct() -> Self {
        Self {
            value: String::from("right"),
        }
    }
    pub fn render(&self) -> String {
        return self.value.clone();
    }
}
