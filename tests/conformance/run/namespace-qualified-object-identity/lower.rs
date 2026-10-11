// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
fn main() {
    let left: TerraneNsLeftResponseIdentity2F6C65667400726573706F6E7365;
    let right: TerraneNsRightResponseIdentity2F726967687400726573706F6E7365;
    let left_value: String;
    let right_value: String;
    left = TerraneNsLeftResponseIdentity2F6C65667400726573706F6E7365::terrane_construct();
    right = TerraneNsRightResponseIdentity2F726967687400726573706F6E7365::terrane_construct();
    left_value = left.render();
    right_value = right.render();
    println!("{}", terrane_scalar_support::scalar_text(&left_value));
    println!("{}", terrane_scalar_support::scalar_text(&right_value));
}
// Source: left/response.trn
// Namespace: left
#[derive(Clone)]
pub struct TerraneNsLeftResponseIdentity2F6C65667400726573706F6E7365 {
    pub value: String,
}
impl TerraneNsLeftResponseIdentity2F6C65667400726573706F6E7365 {
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
pub struct TerraneNsRightResponseIdentity2F726967687400726573706F6E7365 {
    pub value: String,
}
impl TerraneNsRightResponseIdentity2F726967687400726573706F6E7365 {
    pub fn terrane_construct() -> Self {
        Self {
            value: String::from("right"),
        }
    }
    pub fn render(&self) -> String {
        return self.value.clone();
    }
}
