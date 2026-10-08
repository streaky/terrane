// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: app/main.trn
// Namespace: app
fn main() {
    let left_terrane_f0_s155: TerraneNs4LeftResponse;
    let right_terrane_f0_s190: TerraneNs5RightResponse;
    let left_value_terrane_f0_s227: String;
    let right_value_terrane_f0_s264: String;
    left_terrane_f0_s155 = TerraneNs4LeftResponse::terrane_construct();
    right_terrane_f0_s190 = TerraneNs5RightResponse::terrane_construct();
    left_value_terrane_f0_s227 = left_terrane_f0_s155.render();
    right_value_terrane_f0_s264 = right_terrane_f0_s190.render();
    println!("{}", terrane_scalar_support::scalar_text(&left_value_terrane_f0_s227));
    println!("{}", terrane_scalar_support::scalar_text(&right_value_terrane_f0_s264));
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
