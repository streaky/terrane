// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: app/main.trn
// Namespace: app
fn main() {
    let left: TerraneNs4LeftResponse;
    let right: TerraneNs5RightResponse;
    let left_value: String;
    let right_value: String;
    left = TerraneNs4LeftResponse::terrane_construct();
    right = TerraneNs5RightResponse::terrane_construct();
    left_value = left.render();
    right_value = right.render();
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
