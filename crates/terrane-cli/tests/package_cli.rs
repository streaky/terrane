use std::fs;
use std::io::{Read as _, Write as _};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct TempPackage(PathBuf);

impl TempPackage {
    fn new() -> Self {
        let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "terrane-cli-package-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        fs::write(
            path.join("package.toml"),
            "package = \"cli-package\"\nprelude = false\n[namespaces]\n\"cli/app\" = \"app\"\n\"cli/support\" = \"support\"\n",
        )
        .unwrap();
        fs::create_dir_all(path.join("app")).unwrap();
        fs::create_dir_all(path.join("support")).unwrap();
        fs::write(
            path.join("app/main.trn"),
            concat!(
                "namespace cli/app\n",
                "from /core/output import print\n",
                "function main;\n",
                "  print; 'manifest CLI'\n",
            ),
        )
        .unwrap();
        fs::write(
            path.join("support/support.trn"),
            "namespace cli/support\npublic constant value = 1\n",
        )
        .unwrap();
        Self(path)
    }
}

impl Drop for TempPackage {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn manifest_file_and_package_directory_use_the_shared_cli_pipeline() {
    let package = TempPackage::new();
    let executable = env!("CARGO_BIN_EXE_terrane");

    let rust = Command::new(executable)
        .args(["rust", package.0.join("package.toml").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        rust.status.success(),
        "{}",
        String::from_utf8_lossy(&rust.stderr)
    );
    let generated = String::from_utf8(rust.stdout).unwrap();
    assert!(generated.contains("// Source: app/main.trn"));
    assert!(generated.contains("// Namespace: cli/app"));

    let run = Command::new(executable)
        .args(["run", package.0.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(String::from_utf8(run.stdout).unwrap(), "manifest CLI\n");

    let build = Command::new(executable)
        .args(["build", package.0.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let executable_path = PathBuf::from(String::from_utf8(build.stdout).unwrap().trim());
    let build_root = executable_path
        .ancestors()
        .find(|path| path.file_name().is_some_and(|name| name == ".trn"))
        .unwrap();
    assert_eq!(build_root.parent(), Some(package.0.as_path()));
    let generated_project = fs::read_dir(build_root.join("build"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let metadata = fs::read_to_string(generated_project.join("terrane-build.toml")).unwrap();
    assert!(metadata.contains("path = \"app/main.trn\""));
    assert!(metadata.contains("path = \"support/support.trn\""));
    assert!(
        generated_project
            .join("src/authored/app/main.trn.rs")
            .is_file()
    );
    assert!(
        generated_project
            .join("src/authored/support/support.trn.rs")
            .is_file()
    );
    assert_eq!(
        fs::read(generated_project.join("support/terrane-int-support/src/lib.rs")).unwrap(),
        fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../terrane-int-support/src/lib.rs")
        )
        .unwrap()
    );
    fs::remove_dir_all(build_root.join("cache/target")).unwrap();
    let cached_build = Command::new(executable)
        .args(["build", package.0.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(cached_build.status.success());
    assert_eq!(
        PathBuf::from(String::from_utf8(cached_build.stdout).unwrap().trim()),
        executable_path
    );
}

#[test]
fn projected_reqwest_runs_against_a_loopback_server() {
    let package = TempPackage::new();
    fs::write(
        package.0.join("package.toml"),
        concat!(
            "package = \"reqwest-loopback\"\n",
            "[namespaces]\n",
            "app = \"app\"\n",
            "[rust-dependencies.reqwest]\n",
            "version = \"=0.12.23\"\n",
            "features = [\"blocking\", \"rustls-tls-webpki-roots\"]\n",
            "default-features = false\n",
        ),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    fs::write(
        package.0.join("app/main.trn"),
        format!(
            concat!(
                "namespace app\n",
                "from /deps/reqwest/blocking import get\n",
                "function main;\n",
                "    response = get; >http://{}/\n",
                "    response.headers_mut;\n",
                "    body string = response.text;\n",
                "    print; body\n",
            ),
            address
        ),
    )
    .unwrap();
    let server = std::thread::spawn(move || {
        let (mut connection, _) = listener.accept().unwrap();
        let mut request = [0_u8; 4096];
        let _ = connection.read(&mut request).unwrap();
        connection
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 21\r\nConnection: close\r\n\r\nprojected dependency\n")
            .unwrap();
    });
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["run", package.0.join("package.toml").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    server.join().unwrap();
    assert_eq!(output.stdout, b"projected dependency\n\n");
    let generated = fs::read_dir(package.0.join(".trn/build"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let manifest = fs::read_to_string(generated.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("default-features = false"));
    assert!(manifest.contains("\"blocking\""));
    assert!(manifest.contains("\"rustls-tls-webpki-roots\""));
}
