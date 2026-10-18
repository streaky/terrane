#[derive(Debug)]
pub struct LookupError;
impl std::fmt::Display for LookupError {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        output.write_str("missing key")
    }
}
impl std::error::Error for LookupError {}

pub struct Database {
    key: String,
}

pub async fn memory_database() -> Database {
    std::future::ready(()).await;
    Database {
        key: "answer".to_owned(),
    }
}

pub struct ScalarLookup<'q> {
    database: &'q Database,
    key: String,
    value: i64,
}

pub fn lookup_scalar(database: &Database, key: String) -> ScalarLookup<'_> {
    ScalarLookup {
        database,
        key,
        value: 0,
    }
}

impl ScalarLookup<'_> {
    #[must_use]
    pub fn bind(mut self, value: i64) -> Self {
        self.value = value;
        self
    }

    pub async fn fetch_one(self) -> Result<i64, LookupError> {
        std::future::ready(()).await;
        if self.key == self.database.key {
            Ok(self.value)
        } else {
            Err(LookupError)
        }
    }
}

pub struct LineBuilder<'a> {
    prefix: &'a str,
    value: i64,
}

pub fn line(prefix: &str) -> LineBuilder<'_> {
    LineBuilder { prefix, value: 0 }
}

impl LineBuilder<'_> {
    #[must_use]
    pub fn number(mut self, value: i64) -> Self {
        self.value = value;
        self
    }

    #[must_use]
    pub fn finish(self) -> String {
        format!("{}{}", self.prefix, self.value)
    }
}
