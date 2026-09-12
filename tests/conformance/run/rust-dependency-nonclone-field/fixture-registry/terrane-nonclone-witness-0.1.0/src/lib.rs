pub struct NonClone {
    value: i64,
}

impl NonClone {
    pub fn value(&self) -> i64 {
        self.value
    }
}

pub fn make_non_clone(value: i64) -> NonClone {
    NonClone { value }
}
