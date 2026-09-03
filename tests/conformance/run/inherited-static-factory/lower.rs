// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: case.trn
// Namespace: inherited-static-factory
#[derive(Clone)]
pub struct AnimalStorage {}
impl AnimalStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "animal",
            name: "animal",
            kind: "class",
        }
            .name
            .to_owned();
    }
}
#[derive(Clone)]
pub enum Animal {
    Own(AnimalStorage),
    Dog(Dog),
}
impl Animal {
    pub fn terrane_construct() -> Self {
        Self::Own(AnimalStorage::terrane_construct())
    }
    pub fn terrane_static_create() -> Animal {
        return Animal::terrane_construct();
    }
    pub fn class_name(&self) -> String {
        match self {
            Self::Own(value) => value.class_name(),
            Self::Dog(value) => value.class_name(),
        }
    }
}
#[derive(Clone)]
pub struct Dog {}
impl Dog {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "dog",
            name: "dog",
            kind: "class",
        }
            .name
            .to_owned();
    }
    pub fn terrane_static_create() -> Dog {
        return Dog::terrane_construct();
    }
}
fn main() {
    let pet: Dog = Dog::terrane_static_create();
    println!("{}", terrane_scalar_support::scalar_text(&pet.class_name()));
}
