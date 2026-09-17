use terrane_trait_collision_contract::ops::Transform;

pub struct First(i64);
pub struct Second(i64);

impl First {
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl Second {
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl Transform for First {}

impl Transform for Second {}

pub fn first(value: i64) -> First {
    First(value)
}

pub fn second(value: i64) -> Second {
    Second(value)
}
