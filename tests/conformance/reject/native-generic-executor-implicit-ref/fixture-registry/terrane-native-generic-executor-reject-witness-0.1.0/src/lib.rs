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
}

impl<'connection, 'borrow> Executor<'connection> for &'borrow mut Connection {
    type Database = Database;

    fn execute(self) -> i64 {
        self.executions += 1;
        self.executions
    }
}

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
}
