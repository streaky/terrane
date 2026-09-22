pub struct Row {
    value: i64,
}

impl Row {
    pub fn value(&self) -> i64 {
        self.value
    }
}

pub fn rows() -> Vec<Row> {
    vec![Row { value: 1 }, Row { value: 2 }]
}
