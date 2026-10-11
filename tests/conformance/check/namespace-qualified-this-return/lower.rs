// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
#[derive(Clone)]
pub struct BaseStorage {}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn copy(&self) -> Base {
        return Base::Own(self.clone());
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
#[derive(Clone)]
pub enum Base {
    Own(BaseStorage),
    TerraneNsAppChildIdentity2F617070006368696C64(
        TerraneNsAppChildIdentity2F617070006368696C64,
    ),
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self::Own(BaseStorage::terrane_construct())
    }
    pub fn copy(&self) -> Base {
        match self {
            Self::Own(value) => value.copy(),
            Self::TerraneNsAppChildIdentity2F617070006368696C64(value) => value.copy(),
        }
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.marker(),
            Self::TerraneNsAppChildIdentity2F617070006368696C64(value) => value.marker(),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAppChildIdentity2F617070006368696C64 {}
impl TerraneNsAppChildIdentity2F617070006368696C64 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn copy(&self) -> Base {
        return Base::TerraneNsAppChildIdentity2F617070006368696C64(self.clone());
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
fn main() {
    let value: TerraneNsAppChildIdentity2F617070006368696C64;
    let result: Base;
    value = TerraneNsAppChildIdentity2F617070006368696C64::terrane_construct();
    result = value.copy();
    result.marker();
    return ();
}
// Source: right/other.trn
// Namespace: right
#[derive(Clone)]
pub struct TerraneNsRightChildIdentity2F7269676874006368696C64 {
    pub value: String,
}
impl TerraneNsRightChildIdentity2F7269676874006368696C64 {
    pub fn terrane_construct() -> Self {
        Self {
            value: String::from("right"),
        }
    }
}
