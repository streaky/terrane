pub struct Pair<T = String, U = T> {
    first: T,
    second: U,
}

pub struct Opaque<T = String> {
    value: T,
}

impl<T> Opaque<T> {
    pub fn from_value(value: T) -> Self {
        Self { value }
    }

    pub fn into_value(self) -> T {
        self.value
    }
}

pub fn make_text() -> Opaque<String> {
    Opaque::from_value(String::from("text"))
}

pub fn make_flag() -> Opaque<bool> {
    Opaque::from_value(true)
}

pub fn pair_code(value: (bool, bool)) -> u8 {
    u8::from(value.0) * 2 + u8::from(value.1)
}

pub fn flag_code(value: std::collections::BTreeSet<bool>) -> u8 {
    u8::from(value.contains(&true)) * 2 + u8::from(value.contains(&false))
}

pub fn mapping_value(value: std::collections::BTreeMap<String, bool>) -> bool {
    value["present"]
}

pub fn pair_text(value: Pair) -> String {
    value.first + "/" + &value.second
}

pub fn make_nested() -> Opaque<Opaque<bool>> {
    Opaque::from_value(make_flag())
}

pub fn pair() -> (bool, bool) {
    (true, false)
}

pub fn flags() -> std::collections::BTreeSet<bool> {
    [true, false].into_iter().collect()
}

pub fn lookup() -> std::collections::BTreeMap<String, bool> {
    std::collections::BTreeMap::from([(String::from("present"), true)])
}

pub fn default_pair() -> Pair {
    Pair {
        first: String::from("first"),
        second: String::from("second"),
    }
}
