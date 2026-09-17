pub fn prepare_missing_path() -> String {
    let path = std::env::temp_dir().join(format!(
        "terrane-projected-sqlx-lifecycle-{}.sqlite3",
        std::process::id()
    ));
    match std::fs::remove_file(&path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to prepare SQLite path: {error}"),
    }
    path.to_string_lossy().into_owned()
}
