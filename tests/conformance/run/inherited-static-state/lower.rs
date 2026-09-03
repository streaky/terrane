// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: case.trn
// Namespace: inherited-static-state
pub static TERRANE_STATIC_ANIMAL_CALLS: std::sync::LazyLock<
    std::sync::Mutex<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct AnimalStorage {}
impl AnimalStorage {
    pub fn terrane_construct() -> Self {
        Self {}
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
    pub fn terrane_static_increase() -> terrane_int_support::Int {
        {
            let __terrane_static_value = TERRANE_STATIC_ANIMAL_CALLS
                .lock()
                .expect("static field lock poisoned")
                .clone() + terrane_int_support::Int::from(1_i128);
            *TERRANE_STATIC_ANIMAL_CALLS.lock().expect("static field lock poisoned") = __terrane_static_value;
        }
        return TERRANE_STATIC_ANIMAL_CALLS
            .lock()
            .expect("static field lock poisoned")
            .clone();
    }
}
pub static TERRANE_STATIC_DOG_CALLS: std::sync::LazyLock<
    std::sync::Mutex<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct Dog {}
impl Dog {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_increase() -> terrane_int_support::Int {
        {
            let __terrane_static_value = TERRANE_STATIC_DOG_CALLS
                .lock()
                .expect("static field lock poisoned")
                .clone() + terrane_int_support::Int::from(1_i128);
            *TERRANE_STATIC_DOG_CALLS.lock().expect("static field lock poisoned") = __terrane_static_value;
        }
        return TERRANE_STATIC_DOG_CALLS
            .lock()
            .expect("static field lock poisoned")
            .clone();
    }
}
fn main() {
    let animal_first: terrane_int_support::Int = Animal::terrane_static_increase();
    let dog_first: terrane_int_support::Int = Dog::terrane_static_increase();
    let animal_second: terrane_int_support::Int = Animal::terrane_static_increase();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&animal_first),
        terrane_scalar_support::scalar_text(&dog_first),
        terrane_scalar_support::scalar_text(&animal_second)
    );
}
