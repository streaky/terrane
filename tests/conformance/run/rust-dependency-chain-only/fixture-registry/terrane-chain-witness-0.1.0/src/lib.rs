pub struct Database(sqlx::SqlitePool);

pub async fn memory_database() -> Result<Database, sqlx::Error> {
    Ok(Database(
        sqlx::SqlitePool::connect("sqlite::memory:").await?,
    ))
}

pub struct ScalarQuery<'q> {
    sql: String,
    value: i64,
    marker: std::marker::PhantomData<&'q ()>,
}

pub fn query_scalar(sql: String) -> ScalarQuery<'static> {
    ScalarQuery {
        sql,
        value: 0,
        marker: std::marker::PhantomData,
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

    pub async fn fetch_one(self, database: &Database) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(&self.sql)
            .bind(self.value)
            .fetch_one(&database.0)
            .await
    }
}

pub struct LineBuilder<'a> {
    prefix: String,
    value: i64,
    marker: std::marker::PhantomData<&'a ()>,
}

pub fn line(prefix: String) -> LineBuilder<'static> {
    LineBuilder {
        prefix,
        value: 0,
        marker: std::marker::PhantomData,
    }
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
