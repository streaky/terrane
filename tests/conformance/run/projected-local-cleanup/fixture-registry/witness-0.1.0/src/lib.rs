pub mod errors {
    #[derive(Debug)]
    pub struct Error;

    impl std::fmt::Display for Error {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("witness error")
        }
    }
}

pub use errors::Error;
pub type Result<T> = std::result::Result<T, errors::Error>;

pub struct Native;

impl Native {
    pub fn new() -> Self {
        Self
    }

    pub async fn inspect(&self, text: &str, bytes: &Vec<u8>) -> Result<i64> {
        Ok(text.bytes().map(i64::from).sum::<i64>()
            + bytes.iter().map(|byte| i64::from(*byte)).sum::<i64>())
    }
}

pub enum Value {
    Integer(i64),
    Unsigned(u64),
    Text(String),
}

pub fn signed_value() -> Value {
    Value::Integer(i64::MIN)
}

pub fn unsigned_value() -> Value {
    Value::Unsigned(u64::MAX)
}

pub fn text_value() -> Value {
    Value::Text(String::from("owned-text"))
}
