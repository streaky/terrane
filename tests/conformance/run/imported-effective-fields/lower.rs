// Generated deterministically by Terrane <version>.
// Source: src/context/main.trn
// Namespace: app/context
#[derive(Clone)]
pub struct ParentStorage {
    pub origin: String,
}
impl ParentStorage {
    pub fn terrane_construct() -> Self {
        Self {
            origin: String::from("base"),
        }
    }
}
#[derive(Clone)]
pub enum Parent {
    Own(ParentStorage),
    TerraneNsAppMountedApiIdentity2F6170702F6D6F756E74656400617069(
        TerraneNsAppMountedApiIdentity2F6170702F6D6F756E74656400617069,
    ),
}
impl Parent {
    pub fn terrane_construct() -> Self {
        Self::Own(ParentStorage::terrane_construct())
    }
    pub fn terrane_field_origin(&self) -> &String {
        match self {
            Self::Own(value) => &value.origin,
            Self::TerraneNsAppMountedApiIdentity2F6170702F6D6F756E74656400617069(
                value,
            ) => &value.origin,
        }
    }
    pub fn terrane_field_origin_mut(&mut self) -> &mut String {
        match self {
            Self::Own(value) => &mut value.origin,
            Self::TerraneNsAppMountedApiIdentity2F6170702F6D6F756E74656400617069(
                value,
            ) => &mut value.origin,
        }
    }
}
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut value: TerraneNsAppMountedApiIdentity2F6170702F6D6F756E74656400617069;
    let request: Option<String>;
    value = TerraneNsAppMountedApiIdentity2F6170702F6D6F756E74656400617069::terrane_construct();
    {
        let __terrane_value: Option<String> = Some(String::from("GET"));
        value.request = __terrane_value;
    }
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            201_i128,
        );
        value.response = __terrane_value;
    }
    request = value.request.clone();
    if (&request).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &request { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&value.response));
    println!("{}", terrane_scalar_support::scalar_text(&value.origin));
}
// Source: src/mounted/main.trn
// Namespace: app/mounted
#[derive(Clone)]
pub struct TerraneNsAppMountedApiIdentity2F6170702F6D6F756E74656400617069 {
    pub origin: String,
    pub request: Option<String>,
    pub response: terrane_int_support::Int,
    pub local: String,
}
impl TerraneNsAppMountedApiIdentity2F6170702F6D6F756E74656400617069 {
    pub fn terrane_construct() -> Self {
        Self {
            origin: String::from("base"),
            request: None,
            response: terrane_int_support::Int::from(0_i128),
            local: String::from("instance"),
        }
    }
}
