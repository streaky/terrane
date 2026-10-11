// Generated deterministically by Terrane <version>.
// Source: other/foo.trn
// Namespace: other
#[derive(Clone)]
pub struct TerraneNsOtherFooBarIdentity2F6F7468657200666F6F426172 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsOtherFooBarIdentity2F6F7468657200666F6F426172 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(2_i128),
        }
    }
}
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct TerraneNsAppFooBarIdentity2F61707000666F6F2D626172 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppFooBarIdentity2F61707000666F6F2D626172 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
}
fn main() {
    let first: TerraneNsAppFooBarIdentity2F61707000666F6F2D626172;
    let second: TerraneNsOtherFooBarIdentity2F6F7468657200666F6F426172;
    first = TerraneNsAppFooBarIdentity2F61707000666F6F2D626172::terrane_construct();
    second = TerraneNsOtherFooBarIdentity2F6F7468657200666F6F426172::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&first.value));
    println!("{}", terrane_scalar_support::scalar_text(&second.value));
}
