use std::error::Error;
use std::fmt::{self, Display, Formatter};

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, Row, SqliteConnection};

#[derive(Debug)]
pub struct SqliteAdapterError(sqlx::Error);

impl Display for SqliteAdapterError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl Error for SqliteAdapterError {}

impl From<sqlx::Error> for SqliteAdapterError {
    fn from(error: sqlx::Error) -> Self {
        Self(error)
    }
}

async fn connection(path: &str) -> Result<SqliteConnection, SqliteAdapterError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    Ok(SqliteConnection::connect_with(&options).await?)
}

/// Executes one SQLite statement without bound values.
///
/// # Errors
///
/// Returns [`SqliteAdapterError`] when the database cannot be opened or `SQLx` cannot execute the
/// statement.
pub async fn execute(path: String, statement: String) -> Result<(), SqliteAdapterError> {
    let mut connection = connection(&path).await?;
    sqlx::query(&statement).execute(&mut connection).await?;
    Ok(())
}

/// Executes one SQLite statement whose single positional parameter is an owned byte value.
///
/// # Errors
///
/// Returns [`SqliteAdapterError`] when the database cannot be opened, the value cannot be bound,
/// or `SQLx` cannot execute the statement.
pub async fn execute_with_bytes(
    path: String,
    statement: String,
    value: Vec<u8>,
) -> Result<(), SqliteAdapterError> {
    let mut connection = connection(&path).await?;
    sqlx::query(&statement)
        .bind(value)
        .execute(&mut connection)
        .await?;
    Ok(())
}

/// Executes a SQLite query and extracts one bytes column from every row in result order.
///
/// # Errors
///
/// Returns [`SqliteAdapterError`] when the database cannot be opened, `SQLx` cannot execute the
/// query, the requested column is absent, or a column value is not a SQLite blob.
pub async fn query_bytes(
    path: String,
    statement: String,
    column: String,
) -> Result<Vec<Vec<u8>>, SqliteAdapterError> {
    let mut connection = connection(&path).await?;
    let rows = sqlx::query(&statement).fetch_all(&mut connection).await?;
    rows.into_iter()
        .map(|row| {
            row.try_get::<Vec<u8>, _>(column.as_str())
                .map_err(Into::into)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{execute, execute_with_bytes, query_bytes};

    #[tokio::test]
    async fn executes_and_queries_byte_rows_in_order() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must follow the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "terrane-sqlx-adapter-{}-{nonce}.db",
            std::process::id()
        ));
        let path_text = path.to_string_lossy().into_owned();

        execute(
            path_text.clone(),
            "CREATE TABLE messages (sequence INTEGER PRIMARY KEY, body BLOB NOT NULL)".to_owned(),
        )
        .await
        .expect("schema creation must succeed");
        execute_with_bytes(
            path_text.clone(),
            "INSERT INTO messages (sequence, body) VALUES (1, ?)".to_owned(),
            b"first".to_vec(),
        )
        .await
        .expect("first insertion must succeed");
        execute_with_bytes(
            path_text.clone(),
            "INSERT INTO messages (sequence, body) VALUES (2, ?)".to_owned(),
            b"second".to_vec(),
        )
        .await
        .expect("second insertion must succeed");

        let rows = query_bytes(
            path_text,
            "SELECT body FROM messages ORDER BY sequence".to_owned(),
            "body".to_owned(),
        )
        .await
        .expect("query must succeed");
        assert_eq!(rows, [b"first".to_vec(), b"second".to_vec()]);

        fs::remove_file(path).expect("temporary database must be removable");
    }
}
