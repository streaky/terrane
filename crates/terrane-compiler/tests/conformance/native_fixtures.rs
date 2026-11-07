use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_PUBLICATION_ID: AtomicUsize = AtomicUsize::new(0);

/// Stage native fixture sources and Cargo configuration at stable, content-addressed paths.
/// Cargo therefore sees one path package for identical fixture graphs, while each case keeps
/// its own staged workspace and generated Rust crate.
pub(super) fn stage(case: &Path, destination: &Path, target: &Path) {
    let registry = case.join("fixture-registry");
    if !registry.is_dir() {
        return;
    }
    let config = case.join(".cargo");
    let identity = graph_identity(&registry).expect("hash native fixture graph");
    let shared = target.join("native-fixtures").join(identity);
    publish_registry(&registry, &shared).expect("publish native fixture graph");

    let staged_registry = destination.join("fixture-registry");
    let staged_config = destination.join(".cargo");
    link_or_copy(&shared.join("fixture-registry"), &staged_registry)
        .expect("stage shared native fixture registry");
    if config.is_dir() {
        if staged_config.exists() {
            fs::remove_dir_all(&staged_config).expect("replace staged native Cargo configuration");
        }
        copy_tree(&config, &staged_config).expect("stage native Cargo configuration");
        rewrite_patch_paths(&staged_config, &shared.join("fixture-registry"));
    }
}

fn rewrite_patch_paths(config_dir: &Path, registry: &Path) {
    let registry = fs::canonicalize(registry).expect("resolve shared native registry");
    for name in ["config.toml", "config"] {
        let path = config_dir.join(name);
        if !path.is_file() {
            continue;
        }
        let contents = fs::read_to_string(&path).expect("read staged native Cargo configuration");
        let mut config = contents
            .parse::<toml::Table>()
            .expect("parse native Cargo configuration");
        let Some(patches) = config.get_mut("patch").and_then(toml::Value::as_table_mut) else {
            continue;
        };
        for (_, source) in patches.iter_mut() {
            let Some(entries) = source.as_table_mut() else {
                continue;
            };
            for (_, dependency) in entries.iter_mut() {
                let Some(value) = dependency.get_mut("path") else {
                    continue;
                };
                let Some(relative) = value
                    .as_str()
                    .and_then(|path| Path::new(path).strip_prefix("fixture-registry").ok())
                else {
                    continue;
                };
                // Cargo package identities retain logical path aliases. Use the
                // immutable graph path directly, not the replaceable staged link.
                *value = toml::Value::String(
                    registry
                        .join(relative)
                        .to_str()
                        .expect("UTF-8 fixture path")
                        .to_owned(),
                );
            }
        }
        fs::write(
            &path,
            toml::to_string(&config).expect("serialize native Cargo configuration"),
        )
        .expect("write resolved native Cargo configuration");
    }
}

fn graph_identity(registry: &Path) -> io::Result<String> {
    let mut hash = Sha256::new();
    hash_tree(&mut hash, registry, "fixture-registry")?;
    Ok(format!("{:x}", hash.finalize()))
}

fn hash_tree(hash: &mut Sha256, root: &Path, prefix: &str) -> io::Result<()> {
    let mut entries = fs::read_dir(root)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == ".trn" || name == "target" || name == "Cargo.lock" {
            continue;
        }
        let path = entry.path();
        let relative = format!("{prefix}/{name}");
        let metadata = fs::metadata(&path)?;
        if metadata.is_dir() {
            hash_tree(hash, &path, &relative)?;
        } else if metadata.is_file() {
            let contents = fs::read(path)?;
            hash.update((relative.len() as u64).to_le_bytes());
            hash.update(relative.as_bytes());
            hash.update((contents.len() as u64).to_le_bytes());
            hash.update(contents);
        }
    }
    Ok(())
}

fn publish_registry(registry: &Path, destination: &Path) -> io::Result<()> {
    if destination.is_dir() {
        return Ok(());
    }
    let parent = destination.parent().expect("shared graph has parent");
    fs::create_dir_all(parent)?;
    let unique = NEXT_PUBLICATION_ID.fetch_add(1, Ordering::Relaxed);
    let temporary = parent.join(format!(".native-fixture-{}-{unique}", std::process::id()));
    fs::create_dir(&temporary)?;
    copy_tree(registry, &temporary.join("fixture-registry"))?;
    match fs::rename(&temporary, destination) {
        Ok(()) => Ok(()),
        Err(_error) if destination.is_dir() => {
            fs::remove_dir_all(temporary)?;
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn copy_tree(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".trn" || name == "target" {
            continue;
        }
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_tree(&source_path, &destination_path)?;
        } else if kind.is_file() {
            fs::copy(source_path, destination_path)?;
        } else if kind.is_symlink() {
            let metadata = fs::metadata(&source_path)?;
            if metadata.is_dir() {
                copy_tree(&source_path, &destination_path)?;
            } else if metadata.is_file() {
                fs::copy(source_path, destination_path)?;
            }
        }
    }
    Ok(())
}

fn link_or_copy(source: &Path, destination: &Path) -> io::Result<()> {
    if destination.exists() {
        if destination.is_dir() {
            fs::remove_dir_all(destination)?;
        } else {
            fs::remove_file(destination)?;
        }
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(source, destination)
    }
    #[cfg(not(unix))]
    {
        copy_tree(source, destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_root() -> PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("native-fixtures-{}-{id}", std::process::id()))
    }

    #[test]
    fn staged_graph_resolves_shared_relative_dependencies() {
        let root = temporary_root();
        let first = root.join("first");
        let second = root.join("second");
        for case in [&first, &second] {
            fs::create_dir_all(case.join("fixture-registry/example/src")).unwrap();
            fs::create_dir_all(case.join("fixture-registry/child/src")).unwrap();
            fs::create_dir_all(case.join(".cargo")).unwrap();
            fs::write(
                case.join("fixture-registry/example/Cargo.toml"),
                "[package]\nname='example'\nversion='0.1.0'\nedition='2024'\n\n[dependencies]\nchild={path='../child'}\n",
            )
            .unwrap();
            fs::write(
                case.join("fixture-registry/example/src/lib.rs"),
                "pub fn value() { child::value(); }\n",
            )
            .unwrap();
            fs::write(
                case.join("fixture-registry/child/Cargo.toml"),
                "[package]\nname='child'\nversion='0.1.0'\nedition='2024'\n",
            )
            .unwrap();
            fs::write(
                case.join("fixture-registry/child/src/lib.rs"),
                "pub fn value() {}\n",
            )
            .unwrap();
        }
        fs::write(
            first.join(".cargo/config.toml"),
            "[patch.crates-io]\nexample = { path = 'fixture-registry/example' }\n",
        )
        .unwrap();
        fs::write(
            second.join(".cargo/config.toml"),
            "[patch.crates-io]\nexample = { path = 'fixture-registry/example' }\n\n[net]\noffline = true\n",
        )
        .unwrap();
        assert_eq!(
            graph_identity(&first.join("fixture-registry")).unwrap(),
            graph_identity(&second.join("fixture-registry")).unwrap()
        );

        let target = root.join("target");
        let first_workspace = root.join("workspace-one");
        let second_workspace = root.join("workspace-two");
        stage(&first, &first_workspace, &target);
        stage(&second, &second_workspace, &target);
        for (workspace, name) in [
            (&first_workspace, "fixture_consumer_one"),
            (&second_workspace, "fixture_consumer_two"),
        ] {
            fs::create_dir_all(workspace.join("src")).unwrap();
            fs::write(
                workspace.join("src/main.rs"),
                "fn main() { example::value(); }\n",
            )
            .unwrap();
            fs::write(
                workspace.join("Cargo.toml"),
                format!(
                    "[package]\nname='{name}'\nversion='0.1.0'\nedition='2021'\n\n[workspace]\n\n[dependencies]\nexample='0.1.0'\n"
                ),
            )
            .unwrap();
        }
        assert_eq!(
            cargo_package_id(&first_workspace, "example"),
            cargo_package_id(&second_workspace, "example")
        );
        assert_eq!(
            cargo_package_id(&first_workspace, "child"),
            cargo_package_id(&second_workspace, "child")
        );
        fs::remove_dir_all(root).unwrap();
    }

    fn cargo_package_id(workspace: &Path, package: &str) -> String {
        let output = Command::new("cargo")
            .args(["metadata", "--offline", "--format-version=1"])
            .current_dir(workspace)
            .output()
            .expect("run Cargo metadata for fixture graph");
        assert!(
            output.status.success(),
            "Cargo metadata failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        metadata["packages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["name"] == package)
            .unwrap_or_else(|| panic!("Cargo metadata omitted package {package}"))["id"]
            .as_str()
            .unwrap()
            .to_owned()
    }

    #[test]
    fn same_version_variants_keep_distinct_cargo_ids() {
        let root = temporary_root();
        let mut identities = Vec::new();
        let workspace = root.join("workspace");
        fs::create_dir_all(workspace.join("src")).unwrap();
        fs::write(workspace.join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(
            workspace.join("Cargo.toml"),
            "[package]\nname='consumer'\nversion='0.1.0'\n[workspace]\n[dependencies]\nsame='0.1.0'\n",
        )
        .unwrap();
        for (case, source) in [("one", "pub fn first() {}"), ("two", "pub fn second() {}")] {
            let path = root.join(case);
            fs::create_dir_all(path.join("fixture-registry/shared/src")).unwrap();
            fs::write(
                path.join("fixture-registry/shared/Cargo.toml"),
                "[package]\nname='same'\nversion='0.1.0'\n",
            )
            .unwrap();
            fs::write(path.join("fixture-registry/shared/src/lib.rs"), source).unwrap();
            fs::create_dir_all(path.join(".cargo")).unwrap();
            fs::write(
                path.join(".cargo/config.toml"),
                "[patch.crates-io]\nsame={path='fixture-registry/shared'}\n",
            )
            .unwrap();
            stage(&path, &workspace, &root.join("target"));
            identities.push(cargo_package_id(&workspace, "same"));
        }
        assert_ne!(identities[0], identities[1]);
        fs::remove_dir_all(root).unwrap();
    }
}
