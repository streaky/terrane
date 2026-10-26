pub struct Database;

pub trait Executor<'connection> {
    type Database;
    fn execute(self) -> i64;
}

pub struct Connection {
    executions: i64,
}

impl Connection {
    pub fn new() -> Self {
        Self { executions: 0 }
    }

    pub fn executions(&self) -> i64 {
        self.executions
    }
}

impl<'connection, 'borrow> Executor<'connection> for &'borrow mut Connection {
    type Database = Database;

    fn execute(self) -> i64 {
        self.executions += 1;
        self.executions
    }
}

#[derive(Clone)]
pub struct Query;

pub fn query(_sql: &str) -> Query {
    Query
}

impl Query {
    pub fn fetch<'connection, E>(self, executor: E) -> i64
    where
        E: Executor<'connection, Database = Database>,
    {
        executor.execute()
    }

    pub fn fetch_outlives<'executor, 'connection: 'executor, E>(self, executor: E) -> i64
    where
        E: 'executor + Executor<'connection, Database = Database>,
    {
        executor.execute()
    }
}

pub trait Advance {
    fn advance(&mut self) -> i64;
}

impl Advance for Connection {
    fn advance(&mut self) -> i64 {
        self.executions += 1;
        self.executions
    }
}

pub fn advance<T: Advance>(connection: &mut T) -> i64 {
    connection.advance()
}

#[derive(Clone, Copy)]
pub enum Stage {
    Idle,
    Done,
}

impl Advance for Stage {
    fn advance(&mut self) -> i64 {
        *self = Stage::Done;
        7
    }
}

pub trait View {
    fn read(&self) -> i64;
}

impl View for Connection {
    fn read(&self) -> i64 {
        self.executions
    }
}

pub fn observed<T: View>(connection: &T) -> i64 {
    connection.read()
}

pub fn outlives<'region, T: 'region>(_value: T) -> i64 {
    0
}

pub trait DatabaseKind {}

impl DatabaseKind for Database {}

pub struct OwnerQuery<DB: DatabaseKind> {
    marker: std::marker::PhantomData<DB>,
}

pub fn owner_query(_sql: &str) -> OwnerQuery<Database> {
    OwnerQuery { marker: std::marker::PhantomData }
}

impl<DB: DatabaseKind> OwnerQuery<DB>
where
    for<'connection> &'connection mut Connection: Executor<'connection, Database = DB>,
{
    pub fn fetch<'connection, E>(self, executor: E) -> i64
    where
        E: Executor<'connection, Database = DB>,
    {
        executor.execute()
    }
}
