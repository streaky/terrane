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
        Ok(Connection { rows: Vec::new() })
    }
}

impl Connection {
    pub async fn close(self) -> Result<(), QueryError> {
        std::future::ready(()).await;
        Ok(())
    }
}

pub struct Query<'q> {
    operation: &'q str,
    bound: Option<Vec<u8>>,
}

pub fn query(operation: &str) -> Query<'_> {
    Query {
        operation,
        bound: None,
    }
}

impl Query<'_> {
    pub fn bind(mut self, value: Vec<u8>) -> Self {
        self.bound = Some(value);
        self
    }

    pub async fn execute<E: Executor>(self, mut executor: E) -> Result<(), QueryError> {
        let connection = executor.connection();
        let mut yielded = false;
        std::future::poll_fn(|context| {
            if yielded {
                std::task::Poll::Ready(())
            } else {
                yielded = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        })
        .await;
        match self.operation {
            "insert" => {
                if let Some(row) = self.bound {
                    connection.rows.push(row);
                }
                Ok(())
            }
            other => Err(QueryError::InvalidOperation(other.to_owned())),
        }
    }

    pub async fn fetch_all<E: Executor>(self, mut executor: E) -> Result<Vec<Row>, QueryError> {
        std::future::ready(()).await;
        let connection = executor.connection();
        if self.operation != "select" {
            return Err(QueryError::InvalidOperation(self.operation.to_owned()));
        }
        Ok(connection
            .rows
            .iter()
            .cloned()
            .map(|body| Row { body })
            .collect())
    }

    pub async fn fetch_one<E: Executor>(self, mut executor: E) -> Result<Row, QueryError> {
        std::future::ready(()).await;
        let connection = executor.connection();
        let index = match self.operation {
            "select" => 0,
            "select-second" => 1,
            other => return Err(QueryError::InvalidOperation(other.to_owned())),
        };
        connection
            .rows
            .get(index)
            .cloned()
            .map(|body| Row { body })
            .ok_or_else(|| QueryError::InvalidOperation(self.operation.to_owned()))
    }
}

pub struct Row {
    body: Vec<u8>,
}

impl RowAccess for Row {
    type Database = Store;
    fn raw(&self, column: &str) -> Result<ValueRef<'_>, QueryError> {
        if column == "panic" {
            panic!("row accessor panic");
        }
        match column {
            "body" => Ok(ValueRef::Bytes(&self.body)),
            _ => Err(QueryError::MissingColumn(column.to_owned())),
        }
    }
}

pub fn panic_row() -> Row {
    Row { body: Vec::new() }
}
