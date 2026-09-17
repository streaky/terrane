//! Narrow bridges for `SQLx` SQLite shapes that Terrane cannot yet project directly.
//!
//! Connection lifecycle and single-row extraction now use projected upstream trait operations;
//! this module remains only for query-builder chains and non-Clone row collection.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use sqlx::Row;
use sqlx::sqlite::SqliteConnection;

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

/// Executes one SQL statement on an upstream SQLite connection without bound values.
///
/// # Errors
///
/// Returns [`SqliteAdapterError`] when `SQLx` cannot execute the statement.
pub async fn execute(
    connection: &mut SqliteConnection,
    statement: String,
) -> Result<(), SqliteAdapterError> {
    sqlx::query(&statement).execute(connection).await?;
    Ok(())
}

/// Executes one SQL statement whose single positional parameter is an owned byte value.
///
/// # Errors
///
/// Returns [`SqliteAdapterError`] when the value cannot be bound or `SQLx` cannot execute the
/// statement.
pub async fn execute_with_bytes(
    connection: &mut SqliteConnection,
    statement: String,
    value: Vec<u8>,
) -> Result<(), SqliteAdapterError> {
    sqlx::query(&statement)
        .bind(value)
        .execute(connection)
        .await?;
    Ok(())
}

/// Executes a query and extracts one bytes column from every row in result order.
///
/// # Errors
///
/// Returns [`SqliteAdapterError`] when `SQLx` cannot execute the query, the requested column is
/// absent, or a column value is not a SQLite blob.
pub async fn query_bytes(
    connection: &mut SqliteConnection,
    statement: String,
    column: String,
) -> Result<Vec<Vec<u8>>, SqliteAdapterError> {
    let rows = sqlx::query(&statement).fetch_all(connection).await?;
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

    use sqlx::Connection;
    use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};

    use super::{execute, execute_with_bytes, query_bytes};

    struct TempDatabase(std::path::PathBuf);

    impl Drop for TempDatabase {
        fn drop(&mut self) {
            match fs::remove_file(&self.0) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => eprintln!(
                    "failed to remove temporary adapter database `{}`: {error}",
                    self.0.display()
                ),
            }
        }
    }

    #[tokio::test]
    async fn operates_on_the_projected_upstream_connection() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must follow the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "terrane-sqlx-adapter-{}-{nonce}.db",
            std::process::id()
        ));
        let database = TempDatabase(path);
        let path_text = database.0.to_string_lossy().into_owned();
        let options = SqliteConnectOptions::new()
            .filename(path_text)
            .create_if_missing(true);
        let mut connection = SqliteConnection::connect_with(&options)
            .await
            .expect("database connection must open");

        execute(
            &mut connection,
            "CREATE TABLE messages (sequence INTEGER PRIMARY KEY, body BLOB NOT NULL)".to_owned(),
        )
        .await
        .expect("schema creation must succeed");
        execute_with_bytes(
            &mut connection,
            "INSERT INTO messages (sequence, body) VALUES (1, ?)".to_owned(),
            b"first".to_vec(),
        )
        .await
        .expect("first insertion must succeed");
        execute_with_bytes(
            &mut connection,
            "INSERT INTO messages (sequence, body) VALUES (2, ?)".to_owned(),
            b"second".to_vec(),
        )
        .await
        .expect("second insertion must succeed");

        let rows = query_bytes(
            &mut connection,
            "SELECT body FROM messages ORDER BY sequence".to_owned(),
            "body".to_owned(),
        )
        .await
        .expect("query must succeed");
        assert_eq!(rows, [b"first".to_vec(), b"second".to_vec()]);

        connection
            .close()
            .await
            .expect("database connection must close");
        fs::remove_file(&database.0).expect("temporary database must be removable");
    }
}
