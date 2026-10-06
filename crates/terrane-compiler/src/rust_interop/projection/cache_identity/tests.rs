use super::*;

#[test]
fn target_identity_reads_effective_cargo_configuration() {
    let directory =
        std::env::temp_dir().join(format!("terrane-projection-target-{}", std::process::id()));
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
