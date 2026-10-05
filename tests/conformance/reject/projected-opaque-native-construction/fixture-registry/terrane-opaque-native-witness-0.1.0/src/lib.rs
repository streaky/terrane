pub struct Opaque<T> {
    value: T,
}

impl<T> Opaque<T> {
    pub fn from_value(value: T) -> Self {
        Self { value }
    }

    pub fn is_present(&self) -> bool {
        let _ = &self.value;
        true
    }
}

pub fn make_text() -> Opaque<String> {
    Opaque::from_value(String::from("text"))
}

pub fn make_flag() -> Opaque<bool> {
    Opaque::from_value(true)
}
