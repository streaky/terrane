pub struct Row(String);

pub fn row() -> Row {
    Row("owned beyond row".to_owned())
}

impl Row {
    pub fn decoded<T>(&self) -> T
    where
        for<'row> T: From<&'row str>,
    {
        T::from(self.0.as_str())
    }
}
