use super::*;

#[test]
fn lock_seeds_complete_graph() {
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

#[test]
fn failed_resolve_keeps_lock() {
    for existing_lock in [false, true] {
        let root = std::env::temp_dir().join(format!(
            "terrane-projection-lock-failure-{}-{existing_lock}",
            std::process::id()
        ));
        let workspace = root.join(".trn/dependencies");
        fs::create_dir_all(root.join(".cargo")).unwrap();
        for name in ["facade", "owner"] {
            let directory = root.join(name);
            fs::create_dir_all(directory.join("src")).unwrap();
            let dependency = if name == "facade" {
                "\n[dependencies]\nfixture-owner = { path = \"../owner\" }\n"
            } else {
                ""
            };
            fs::write(
                directory.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"fixture-{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n{dependency}"
                ),
            )
            .unwrap();
            fs::write(directory.join("src/lib.rs"), "").unwrap();
        }
        fs::write(
            root.join(".cargo/config.toml"),
            format!(
                "[patch.crates-io]\nfixture-facade = {{ path = {:?} }}\nfixture-owner = {{ path = {:?} }}\n",
                root.join("facade"),
                root.join("owner")
            ),
        )
        .unwrap();
        let dependencies = [RustDependency {
            name: "fixture-facade".to_owned(),
            package: "fixture-facade".to_owned(),
            version: "=0.1.0".to_owned(),
            features: Vec::new(),
            default_features: false,
            target: None,
            effects: Vec::new(),
        }];
        write_workspace_with_bound_dependencies(
            &workspace,
            &dependencies,
            &[ProjectedBoundDependency {
                name: "fixture-owner".to_owned(),
                package: "fixture-owner".to_owned(),
                version: "=0.1.0".to_owned(),
                features: Vec::new(),
            }],
        )
        .unwrap();
        let complete = fs::read(workspace.join("Cargo.lock")).unwrap();
        if existing_lock {
            persist_dependency_lock(&root, &workspace).unwrap();
        }
        // Fail after base resolution but before projection finalization.
        fs::create_dir(workspace.join("local-source-identity")).unwrap();
        super::super::resolve(&root, &dependencies, None).unwrap_err();
        assert_eq!(
            fs::read(root.join(DEPENDENCY_LOCK_FILE)).ok().as_deref(),
            existing_lock.then_some(complete.as_slice())
        );
        // Cargo really resolved the narrower workspace; this is not an early fetch failure.
        let intermediate = fs::read(workspace.join("Cargo.lock")).unwrap();
        assert_ne!(intermediate, complete);
        let parsed: toml::Value =
            toml::from_str(std::str::from_utf8(&intermediate).unwrap()).unwrap();
        let packages = parsed["package"].as_array().unwrap();
        let synthetic = packages
            .iter()
            .find(|package| package["name"].as_str() == Some("terrane_dependency_projection"))
            .unwrap();
        assert_eq!(
            synthetic["dependencies"].as_array().unwrap(),
            &[toml::Value::String("fixture-facade".to_owned())]
        );
        fs::remove_dir_all(root).unwrap();
    }
}
