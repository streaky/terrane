// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
fn main() {
    let left: TerraneNs4LeftResponse = TerraneNs4LeftResponse::terrane_construct();
    let right: TerraneNs5RightResponse = TerraneNs5RightResponse::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&left.value));
    println!("{}", terrane_scalar_support::scalar_text(&right.value));
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
}
