use super::*;

#[test]
fn durable_dependency_lock_seeds_and_records_the_complete_cargo_graph() {
    let root = std::env::temp_dir().join(format!(
        "terrane-durable-dependency-lock-{}",
        std::process::id()
    ));
    let workspace = root.join(".trn/dependencies");
    fs::create_dir_all(&workspace).unwrap();
    let resolved = b"version = 4\n\n[[package]]\nname = \"facade\"\nversion = \"1.0.0\"\n\n[[package]]\nname = \"owner\"\nversion = \"2.0.0\"\n";
    fs::write(workspace.join("Cargo.lock"), resolved).unwrap();
    persist_dependency_lock(&root, &workspace).unwrap();
    assert_eq!(fs::read(root.join(DEPENDENCY_LOCK_FILE)).unwrap(), resolved);
    fs::write(workspace.join("Cargo.lock"), b"stale").unwrap();
    seed_dependency_lock(&root, &workspace).unwrap();
    assert_eq!(fs::read(workspace.join("Cargo.lock")).unwrap(), resolved);
    fs::remove_dir_all(root).unwrap();
}
