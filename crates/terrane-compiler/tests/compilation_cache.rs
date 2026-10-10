use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use terrane_compiler::debugging::ProvenanceRole;
use terrane_compiler::with_compilation_cache;
use terrane_compiler::{CompilerOptions, DebugBuild, Package, compile_package_with_options};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Cleanup(PathBuf);
impl Drop for Cleanup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn changing_debug_options_and_reloading_symbols_preserves_user_breakpoint_coordinates() {
    let root = std::env::temp_dir().join(format!(
        "terrane-debug-cache-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _cleanup = Cleanup(root.clone());
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("package.toml"),
        "package = 'debug-cache'\nprelude = false\n[namespaces]\n'cache/app' = 'src'\n",
    )
    .unwrap();
    fs::write(
        root.join("src/main.trn"),
        "namespace cache/app\nfrom /core/output import print\nfunction main;\n    print; '🙂'\n",
    )
    .unwrap();
    let package = Package::load(&root).unwrap();
    compile_package_with_options(&package, CompilerOptions::default()).unwrap();
    assert!(
        !root.join(".trn/cache").exists(),
        "library compilation must not persist snapshots"
    );
    with_compilation_cache(|| {
        let plain = compile_package_with_options(
            &package,
            CompilerOptions {
                debug_build: DebugBuild::Disabled,
                ..CompilerOptions::default()
            },
        )
        .unwrap();
        assert!(
            plain
                .debug_information(Path::new("src/main.rs"))
                .unwrap()
                .is_none()
        );
        let options = CompilerOptions {
            debug_build: DebugBuild::EmbeddedSources,
            ..CompilerOptions::default()
        };
        compile_package_with_options(&package, options).unwrap();
        let warm = compile_package_with_options(&package, options).unwrap();
        let info = warm
            .debug_information(Path::new("src/main.rs"))
            .unwrap()
            .expect("debug options must not reuse non-debug output");
        let authored = info
            .sources
            .iter()
            .find(|source| source.uri.ends_with("main.trn"))
            .unwrap()
            .id;
        assert!(
            info.generated_files
                .iter()
                .flat_map(|file| &file.associations)
                .filter(|association| association.role == ProvenanceRole::User)
                .flat_map(|association| &association.causes)
                .any(|cause| cause.source_id == authored && cause.line == 4 && cause.column == 5)
        );
    });
}
