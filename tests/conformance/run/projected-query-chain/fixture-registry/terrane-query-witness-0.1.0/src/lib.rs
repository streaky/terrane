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

pub trait Database {
    type QueryResult;
    fn query_result(rows_affected: u64, persistent: bool) -> Self::QueryResult;
}

pub struct StoreResult {
    persistent: bool,
    rows_affected: u64,
}
impl StoreResult {
    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
    pub fn persistent(&self) -> bool {
        self.persistent
    }
}
impl Database for Store {
    type QueryResult = StoreResult;
    fn query_result(rows_affected: u64, persistent: bool) -> Self::QueryResult {
        StoreResult {
            rows_affected,
            persistent,
        }
    }
}
pub trait Executor {
    type Database: Database;
    fn connection(&mut self) -> &mut Connection;
}
impl Executor for &mut Connection {
    type Database = Store;
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

pub struct Query<'q, DB: Database, A> {
    operation: &'q str,
    bound: Option<Vec<u8>>,
    persistent: bool,
    database: std::marker::PhantomData<DB>,
    arguments: std::marker::PhantomData<A>,
}

pub fn insert_sql() -> String {
    "insert".to_owned()
}

pub fn query(operation: &str) -> Query<'_, Store, ()> {
    Query {
        operation,
        bound: None,
        persistent: true,
        database: std::marker::PhantomData,
        arguments: std::marker::PhantomData,
    }
}

impl<'q, DB: Database, A> Query<'q, DB, A> {
    pub fn bind(mut self, value: Vec<u8>) -> Self {
        self.bound = Some(value);
        self
    }

    pub fn persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }

    pub async fn execute<E>(self, mut executor: E) -> Result<DB::QueryResult, QueryError>
    where
        E: Executor<Database = DB>,
    {
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
                let affected = if let Some(row) = self.bound {
                    connection.rows.push(row);
                    1
                } else {
                    0
                };
                Ok(DB::query_result(affected, self.persistent))
            }
            other => Err(QueryError::InvalidOperation(other.to_owned())),
        }
    }

    pub async fn fetch_all<E>(self, mut executor: E) -> Result<Vec<Row>, QueryError>
    where
        E: Executor<Database = DB>,
    {
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

    pub async fn fetch_one<E>(self, mut executor: E) -> Result<Row, QueryError>
    where
        E: Executor<Database = DB>,
    {
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
