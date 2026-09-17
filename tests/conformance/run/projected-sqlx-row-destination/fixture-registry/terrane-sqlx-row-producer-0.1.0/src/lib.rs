use sqlx::Connection;

pub async fn sample_row() -> Result<sqlx::sqlite::SqliteRow, sqlx::Error> {
    let mut connection = sqlx::SqliteConnection::connect(":memory:").await?;
    sqlx::query("select x'0102' as body, 42 as count")
        .fetch_one(&mut connection)
        .await
}
