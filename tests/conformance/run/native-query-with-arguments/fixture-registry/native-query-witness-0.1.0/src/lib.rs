pub struct Store;
pub struct StoreResult {
    rows_affected: u64,
    persistent: bool,
}
impl StoreResult {
    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
    pub fn persistent(&self) -> bool {
        self.persistent
    }
}
pub struct Connection {
    rows: Vec<i64>,
}
impl Connection {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }
}
pub struct Arguments {
    values: Vec<i64>,
}
impl Arguments {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, value: i64) {
        self.values.push(value);
    }
}
pub struct SqlToken(String);
impl SqlToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

pub trait ArgumentValues {
    fn into_values(self) -> Vec<i64>;
}
impl ArgumentValues for Arguments {
    fn into_values(self) -> Vec<i64> {
        self.values
    }
}

pub trait Database {
    type QueryResult;
    fn query_result(rows_affected: u64, persistent: bool) -> Self::QueryResult;
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
pub trait IntoArguments<'q, DB: Database>: ArgumentValues + Send {}
impl<'q> IntoArguments<'q, Store> for Arguments {}
pub trait Executor {
    type Database: Database;
    fn rows(&mut self) -> &mut Vec<i64>;
}
impl Executor for &mut Connection {
    type Database = Store;
    fn rows(&mut self) -> &mut Vec<i64> {
        &mut self.rows
    }
}

pub struct Query<'q, DB: Database, A> {
    sql: SqlToken,
    arguments: A,
    persistent: bool,
    lifetime: std::marker::PhantomData<&'q ()>,
    database: std::marker::PhantomData<DB>,
}
pub fn query_with<'q, A>(sql: SqlToken, arguments: A) -> Query<'q, Store, A>
where
    A: IntoArguments<'q, Store>,
{
    Query {
        sql,
        arguments,
        persistent: true,
        lifetime: std::marker::PhantomData,
        database: std::marker::PhantomData,
    }
}
impl<'q, DB: Database, A: ArgumentValues + Send> Query<'q, DB, A> {
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }
    pub async fn execute<E>(self, mut executor: E) -> DB::QueryResult
    where
        E: Executor<Database = DB>,
    {
        let mut yielded = false;
        std::future::poll_fn(|cx| {
            if yielded {
                std::task::Poll::Ready(())
            } else {
                yielded = true;
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        })
        .await;
        let values = self.arguments.into_values();
        executor.rows().extend(values);
        assert_eq!(self.sql.0, "insert");
        DB::query_result(executor.rows().len() as u64, self.persistent)
    }
}
