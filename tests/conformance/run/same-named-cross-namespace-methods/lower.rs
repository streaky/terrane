// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: app/main.trn
// Namespace: app
fn main() {
    let first: TerraneNsFirstDuplicateIdentity2F6669727374006475706C6963617465;
    let second: TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465;
    first = TerraneNsFirstDuplicateIdentity2F6669727374006475706C6963617465::terrane_construct();
    second = TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465::terrane_construct(
        terrane_int_support::Int::from(2_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&first.value()),
        terrane_scalar_support::scalar_text(&second.value())
    );
    println!("{}", terrane_scalar_support::scalar_text(&first.local_helper()));
    first.local_destructor();
    println!("{}", String::from("end"));
    println!("{}", terrane_scalar_support::scalar_text(&first.foreign_value()));
}
// Source: first/item.trn
// Namespace: first
#[derive(Clone)]
pub struct TerraneNsFirstDuplicateIdentity2F6669727374006475706C6963617465 {}
impl TerraneNsFirstDuplicateIdentity2F6669727374006475706C6963617465 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn local_helper(&self) -> terrane_int_support::Int {
        fn helper() -> terrane_int_support::Int {
            return terrane_int_support::Int::from(7_i128);
        }
        return helper();
    }
    pub fn local_destructor(&self) {
        fn destruct() {
            println!("{}", String::from("local destruct ran"));
        }
        destruct();
    }
    pub fn value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    pub fn foreign_value(&self) -> terrane_int_support::Int {
        let seed: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
        let other: TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465;
        seed = {
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return terrane_int_support::Int::from(3_i128);
            })
        };
        other = TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465::terrane_construct(
            seed(),
        );
        return other.value();
    }
}
// Source: second/item.trn
// Namespace: second
#[derive(Clone)]
pub struct TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465 {
    __terrane_lifetime: std::sync::Arc<()>,
    pub stored: terrane_int_support::Int,
}
impl TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465 {
    pub fn terrane_construct(value: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            stored: terrane_int_support::Int::from(0_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, value: terrane_int_support::Int) {
        {
            let __terrane_value: terrane_int_support::Int = value.clone();
            self.stored = __terrane_value;
        }
    }
    pub fn value(&self) -> terrane_int_support::Int {
        return self.stored.clone();
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", String::from("d"), terrane_scalar_support::scalar_text(&self.stored)
        );
    }
}
impl Drop for TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465 {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
