//! Narrow bridges for `SQLx` SQLite shapes that Terrane cannot yet project directly.
//!
//! Direct query, bind, execute, fetch, row access, and connection lifecycle operations now use
//! projected upstream APIs. This module remains only for consuming a collected sequence of
//! non-`Clone` rows into ordinary Terrane byte values.

use sqlx::Row;
use sqlx::sqlite::SqliteConnection;

/// Executes a query and extracts one bytes column from every row in result order.
///
/// # Errors
///
/// Returns [`std::io::Error`] when `SQLx` cannot execute the query, the requested column is absent,
/// or a column value is not a SQLite blob. The bridge preserves `SQLx`'s display text while using
/// the established standard-library dependency-error boundary.
pub async fn query_bytes(
    connection: &mut SqliteConnection,
    statement: String,
    column: String,
) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let rows = sqlx::query(&statement)
        .fetch_all(connection)
        .await
        .map_err(std::io::Error::other)?;
    rows.into_iter()
        .map(|row| {
            row.try_get::<Vec<u8>, _>(column.as_str())
                .map_err(std::io::Error::other)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use sqlx::Connection;
    use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};

    use super::query_bytes;

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

        sqlx::query("CREATE TABLE messages (sequence INTEGER PRIMARY KEY, body BLOB NOT NULL)")
            .execute(&mut connection)
            .await
            .expect("schema creation must succeed");
        sqlx::query("INSERT INTO messages (sequence, body) VALUES (1, ?)")
            .bind(b"first".to_vec())
            .execute(&mut connection)
            .await
            .expect("first insertion must succeed");
        sqlx::query("INSERT INTO messages (sequence, body) VALUES (2, ?)")
            .bind(b"second".to_vec())
            .execute(&mut connection)
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
