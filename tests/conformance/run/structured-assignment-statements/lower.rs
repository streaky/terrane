// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: structured-assignment-statements
#[derive(Clone)]
pub struct Item {
    __terrane_lifetime: std::sync::Arc<()>,
    pub x: terrane_int_support::Int,
}
impl Item {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            x: terrane_int_support::Int::from(0_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self) {
        println!("{}", String::from("construct"));
    }
    pub fn destruct(&mut self) {
        println!("{}", terrane_scalar_support::scalar_text(&self.x));
    }
}
impl Drop for Item {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn field_value() -> terrane_int_support::Int {
    println!("{}", String::from("field"));
    return terrane_int_support::Int::from(5_i128);
}
fn main() {
    Item::terrane_construct();
    let _ = {
        let mut instance = Item::terrane_construct();
        {
            let __terrane_value = field_value();
            instance.x = __terrane_value;
        }
        instance
    };
    println!("{}", String::from("after"));
}
