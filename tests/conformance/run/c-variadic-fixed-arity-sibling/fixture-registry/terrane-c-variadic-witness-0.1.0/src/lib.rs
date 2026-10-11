unsafe extern "C" {
    pub fn variable_value(first: i32, ...);
}

pub extern "C" fn fixed_value(value: i32) -> i32 {
    value + 37
}
