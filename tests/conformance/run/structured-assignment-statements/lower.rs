// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: structured-assignment-statements
#[derive(Clone)]
pub struct Item {
    __terrane_lifetime: std::sync::Arc<()>,
    pub x: terrane_int_support::Int,
}
impl Item {
    pub fn terrane_construct(x: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            x: terrane_int_support::Int::from(0_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(x);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, x: terrane_int_support::Int) {
        {
            let __terrane_value = x.clone();
            self.x = __terrane_value;
        }
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
    Item::terrane_construct(terrane_int_support::Int::from(0_i128));
    let _ = {
        let mut instance = Item::terrane_construct(
            terrane_int_support::Int::from(0_i128),
        );
        {
            let __terrane_value = field_value();
            instance.x = __terrane_value;
        }
        instance
    };
    Item::terrane_construct(terrane_int_support::Int::from(7_i128));
    println!("{}", String::from("after"));
}
