// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: function-scope-release
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::new(),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        self.name = name;
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("released ")),
            terrane_scalar_support::scalar_text(&self.name)
        );
    }
}
impl Drop for Marker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn main() {
    let mut value: Marker;
    let mut count: terrane_int_support::Int;
    let value_2: String;
    let retained: Marker;
    value = Marker::terrane_construct(String::from("first"));
    println!("{}", terrane_scalar_support::scalar_text(&value.name));
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-if")));
    count = terrane_int_support::Int::from(0_i128);
    while count.clone() < terrane_int_support::Int::from(2_i128) {
        value = Marker::terrane_construct(String::from("loop"));
        println!("{}", terrane_scalar_support::scalar_text(&value.name));
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-loop")));
    let __terrane_replacement = String::from("done");
    drop(value);
    value_2 = __terrane_replacement;
    println!("{}", terrane_scalar_support::scalar_text(&value_2));
    retained = Marker::terrane_construct(String::from("retained"));
    println!("{}", terrane_scalar_support::scalar_text(&retained.name));
    println!("{}", terrane_scalar_support::scalar_text(&String::from("function-end")));
}
