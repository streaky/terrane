#[derive(Debug)]
pub enum QueryError {
    MissingColumn(String),
    InvalidSelector(String),
    InvalidOperation(String),
    DecodeType,
}
impl std::fmt::Display for QueryError {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingColumn(column) => write!(output, "missing column: {column}"),
            Self::InvalidSelector(selector) => write!(output, "invalid selector: {selector}"),
            Self::InvalidOperation(operation) => write!(output, "invalid operation: {operation}"),
            Self::DecodeType => output.write_str("incompatible value type"),
        }
    }
}
impl std::error::Error for QueryError {}

pub enum ValueRef<'a> {
    Bytes(&'a [u8]),
    Count(i64),
}

pub trait Decode<'r, Database>: Sized {
    fn decode(value: ValueRef<'r>) -> Result<Self, QueryError>;
}
pub trait NativeType<Database> {}

pub trait ColumnIndex<R: ?Sized> {
    fn name(&self) -> &str;
}
impl<R: ?Sized> ColumnIndex<R> for &str {
    fn name(&self) -> &str {
        self
    }
}

// The lifetime bound, unsupported required borrowed result, and provided method
// with input- and destination-selected generics are the original projection shape.
pub trait RowAccess: 'static {
    type Database;
    fn raw(&self, column: &str) -> Result<ValueRef<'_>, QueryError>;

    fn try_get<'r, T, I>(&'r self, column: I) -> Result<T, QueryError>
    where
        T: Decode<'r, Self::Database> + NativeType<Self::Database>,
        I: ColumnIndex<Self>,
        Self: Sized,
    {
        T::decode(self.raw(column.name())?)
    }
}
