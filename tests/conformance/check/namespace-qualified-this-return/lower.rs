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
    TerraneNs3AppChild(TerraneNs3AppChild),
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self::Own(BaseStorage::terrane_construct())
    }
    pub fn copy(&self) -> Base {
        match self {
            Self::Own(value) => value.copy(),
            Self::TerraneNs3AppChild(value) => value.copy(),
        }
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.marker(),
            Self::TerraneNs3AppChild(value) => value.marker(),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNs3AppChild {}
impl TerraneNs3AppChild {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn copy(&self) -> Base {
        return Base::TerraneNs3AppChild(self.clone());
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
fn main() {
    let value: TerraneNs3AppChild = TerraneNs3AppChild::terrane_construct();
    let result: Base = value.copy();
    result.marker();
    return ();
}
// Source: right/other.trn
// Namespace: right
#[derive(Clone)]
pub struct TerraneNs5RightChild {
    pub value: String,
}
impl TerraneNs5RightChild {
    pub fn terrane_construct() -> Self {
        Self {
            value: String::from("right"),
        }
    }
}
