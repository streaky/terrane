use super::*;

#[test]
fn missing_rustc_host_is_an_error() {
    let error = host_target("rustc 1.99.0\nrelease: 1.99.0\n").unwrap_err();
    assert!(error.message.contains("did not report a `host:` line"));
}

#[test]
fn target_reads_cargo_config() {
    let directory = std::env::temp_dir().join(format!(
        "terrane-projection-target-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".cargo")).unwrap();
    fs::write(
        directory.join(".cargo/config.toml"),
        "[build]\ntarget = \"wasm32-unknown-unknown\"\n",
    )
    .unwrap();
    assert_eq!(
        selected_target(&directory, "host: x86_64-unknown-linux-gnu").unwrap(),
        "wasm32-unknown-unknown"
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn relative_json_target_origin() {
    let directory = std::env::temp_dir().join(format!(
        "terrane-projection-relative-target-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let workspace = directory.join("projection-workspace");
    fs::create_dir_all(directory.join(".cargo")).unwrap();
    fs::create_dir_all(directory.join("targets")).unwrap();
    fs::create_dir_all(&workspace).unwrap();
    fs::write(directory.join("targets/custom.json"), "{}").unwrap();
    let config = directory.join(".cargo/config.toml");
    for target in ["\"targets/custom.json\"", "[\"targets/custom.json\"]"] {
        fs::write(&config, format!("[build]\ntarget = {target}\n")).unwrap();
        assert_eq!(
            selected_target(&workspace, "host: x86_64-unknown-linux-gnu").unwrap(),
            directory
                .join("targets/custom.json")
                .canonicalize()
                .unwrap()
                .to_string_lossy()
        );
    }
    fs::remove_file(directory.join("targets/custom.json")).unwrap();
    let error = selected_target(&workspace, "host: x86_64-unknown-linux-gnu").unwrap_err();
    assert!(error.message.contains(&format!("`{}`", config.display())));

    fs::remove_dir_all(directory).unwrap();
}
