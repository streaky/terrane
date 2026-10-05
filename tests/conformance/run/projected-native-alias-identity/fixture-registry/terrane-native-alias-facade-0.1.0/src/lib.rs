pub use terrane_native_alias_owner::Holder as PublicHolder;
pub use terrane_native_alias_owner::Holder as OtherHolder;
pub use terrane_native_alias_owner::PublicValue;

pub type Alias<T = u8> = terrane_native_alias_owner::Holder<T>;
pub type StringAlias<T = String> = Alias<T>;

pub fn make_default() -> Alias {
    PublicHolder::new(7)
}

pub fn make_string() -> Alias<String> {
    PublicHolder::new(String::from("selected-string"))
}

pub fn make_string_default() -> StringAlias {
    PublicHolder::new(String::from("default-string"))
}

pub fn take_byte(value: terrane_native_alias_owner::Holder<u8>) -> u8 {
    value.into_value()
}

pub fn take_text(value: terrane_native_alias_owner::Holder<String>) -> String {
    value.into_value()
}
