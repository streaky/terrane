pub struct Database(sqlx::SqlitePool);

pub async fn memory_database() -> Result<Database, sqlx::Error> {
    Ok(Database(
        sqlx::SqlitePool::connect("sqlite::memory:").await?,
    ))
}

pub struct ScalarQuery<'q> {
    database: &'q Database,
    sql: String,
    value: i64,
}

pub fn query_scalar(database: &Database, sql: String) -> ScalarQuery<'_> {
    ScalarQuery {
        database,
        sql,
        value: 0,
    }
}

impl ScalarQuery<'_> {
    #[must_use]
    pub fn bind(mut self, value: i64) -> Self {
        self.value = value;
        self
    }

    pub async fn pause(self) -> Self {
        std::future::ready(()).await;
        self
    }

    pub async fn fetch_one(self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(&self.sql)
            .bind(self.value)
            .fetch_one(&self.database.0)
            .await
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

    pub async fn pause(self) -> Self {
        std::future::ready(()).await;
        self
    }

    #[must_use]
    pub fn finish(self) -> String {
        format!("{}{}", self.prefix, self.value)
    }
}
