pub use terrane_query_contract::QueryError;
use terrane_query_contract::{Decode, NativeType, RowAccess, ValueRef};

pub struct Store;
impl NativeType<Store> for Vec<u8> {}
impl<'r> Decode<'r, Store> for Vec<u8> {
    fn decode(value: ValueRef<'r>) -> Result<Self, QueryError> {
        match value {
            ValueRef::Bytes(bytes) => Ok(bytes.to_vec()),
            ValueRef::Count(_) => Err(QueryError::DecodeType),
        }
    }
}

pub struct Connection {
    rows: Vec<Vec<u8>>,
    open: bool,
}

pub trait Executor {
    fn connection(&mut self) -> &mut Connection;
}
impl Executor for &mut Connection {
    fn connection(&mut self) -> &mut Connection {
        self
    }
}

pub struct ConnectionOptions;

impl ConnectionOptions {
    pub fn new() -> Self {
        Self
    }

    pub async fn connect(&self) -> Result<Connection, QueryError> {
        std::future::ready(()).await;
        Ok(Connection {
            rows: vec![vec![1, 2]],
            open: true,
        })
    }
}

impl Connection {
    pub async fn close(self) -> Result<(), QueryError> {
        std::future::ready(()).await;
        if self.open {
            Ok(())
        } else {
            Err(QueryError::InvalidSelector("closed".to_owned()))
        }
    }
}

pub struct Query<'q> {
    selector: &'q str,
}

pub fn query(selector: &str) -> Query<'_> {
    Query { selector }
}

impl Query<'_> {
    pub async fn fetch_one<E: Executor>(self, mut executor: E) -> Result<Row, QueryError> {
        std::future::ready(()).await;
        let connection = executor.connection();
        if self.selector != "first" {
            return Err(QueryError::InvalidSelector(self.selector.to_owned()));
        }
        connection
            .rows
            .first()
            .cloned()
            .map(|body| Row { body, count: 42 })
            .ok_or_else(|| QueryError::InvalidSelector(self.selector.to_owned()))
    }
}

pub struct Row {
    body: Vec<u8>,
    count: i64,
}

impl RowAccess for Row {
    type Database = Store;
    fn raw(&self, column: &str) -> Result<ValueRef<'_>, QueryError> {
        match column {
            "body" => Ok(ValueRef::Bytes(&self.body)),
            "count" => Ok(ValueRef::Count(self.count)),
            _ => Err(QueryError::MissingColumn(column.to_owned())),
        }
    }
}
