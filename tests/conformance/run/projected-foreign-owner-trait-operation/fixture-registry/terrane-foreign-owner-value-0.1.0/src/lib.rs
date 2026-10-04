pub struct Value {
    pub amount: i64,
}

impl Value {
    pub const BASE: Self = Self { amount: 19 };

    pub fn new(amount: i64) -> Self {
        Self { amount }
    }
}
