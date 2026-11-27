use flate2::{Compression, write::GzEncoder};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead as _, BufReader, Write as _};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const LOCK: &str = "terrane-dependencies.lock";
const APP_CRATE: &str = "terrane-fetch-app-witness";
const TOOL_CRATE: &str = "terrane-fetch-tool-witness";
const VERSION: &str = "0.1.0";

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        // Contained Cargo builds mount a private /tmp. Keep their read-only
        // registry and configuration visible from inside that sandbox.
        let parent = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target");
        fs::create_dir_all(&parent).unwrap();
        let root = parent.canonicalize().unwrap().join(format!(
            "terrane-cli-fetch-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&root).unwrap();
        Self(root)
    }

    fn write(&self, relative: &str, text: impl AsRef<[u8]>) {
        let path = self.0.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    fn command(&self, program: &str, home: &str, offline: bool) -> Command {
        let mut command = Command::new(program);
        command
            .current_dir(&self.0)
            .env("CARGO_HOME", self.0.join(home))
            .env("CARGO_NET_OFFLINE", if offline { "true" } else { "false" })
            .env("CARGO_NET_RETRY", "0")
            .env("NO_PROXY", "127.0.0.1")
            .env("no_proxy", "127.0.0.1")
            .env_remove("TERRANE_PROJECTION_ARTIFACT_URL");
        command
    }

    fn terrane(&self, home: &str, offline: bool, args: &[&str]) -> Output {
        self.command(env!("CARGO_BIN_EXE_terrane"), home, offline)
            .args(args)
            .output()
            .unwrap()
    }

    fn seed_lock(&self, package: &str, dependency: &str) -> Vec<u8> {
        // Use a throwaway Cargo home to resolve the same root graph that Terrane writes.
        // Nothing from this resolution's downloaded/index cache is copied to the test home.
        self.write(
            "lock-seed/Cargo.toml",
            format!(
                "[package]\nname = 'terrane_dependency_projection'\nversion = '0.0.0'\nedition = '2024'\n\n\
                 [dependencies]\n{dependency} = '={VERSION}'\n\n[workspace]\n"
            ),
        );
        self.write("lock-seed/src/lib.rs", "");
        let lock = self.0.join("lock-seed/Cargo.lock");
        if lock.exists() {
            fs::remove_file(&lock).unwrap();
        }
        success(
            self.command("cargo", "seed-home", false)
                .current_dir(self.0.join("lock-seed"))
                .arg("generate-lockfile")
                .output()
                .unwrap(),
        );
        let bytes = fs::read(lock).unwrap();
        self.write(&format!("{package}/{LOCK}"), &bytes);
        bytes
    }

    fn seed_runtime_cache(&self, home: &str, mirror: &Path) {
        // Native build workspaces include Terrane's support crates, separate
        // from the application/consumer projection locks. Supply the runtime
        // registry inputs already cached by this test binary's own build.
        let host_home = std::env::var_os("CARGO_HOME").map_or_else(
            || PathBuf::from(std::env::var_os("HOME").unwrap()).join(".cargo"),
            PathBuf::from,
        );
        let host_registry = host_home.join("registry");
        let host_indexes = fs::read_dir(host_registry.join("index"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        let lock: toml::Value = include_str!("../../../Cargo.lock").parse().unwrap();
        let index_cache = self
            .0
            .join(home)
            .join("registry/index")
            .join(mirror.file_name().unwrap())
            .join(".cache");
        for package in lock["package"].as_array().unwrap() {
            if package.get("source").and_then(toml::Value::as_str)
                != Some("registry+https://github.com/rust-lang/crates.io-index")
            {
                continue;
            }
            let name = package["name"].as_str().unwrap();
            let version = package["version"].as_str().unwrap();
            let archive = format!("{name}-{version}.crate");
            let index_file = sparse_index_path(name);
            let dest_index = index_cache.join(&index_file);
            let dest_archive = mirror.join(&archive);
            for index in &host_indexes {
                let source_index = index.join(".cache").join(&index_file);
                if source_index.is_file() && !dest_index.exists() {
                    fs::create_dir_all(dest_index.parent().unwrap()).unwrap();
                    fs::copy(source_index, &dest_index).unwrap();
                }
                let source_archive = host_registry
                    .join("cache")
                    .join(index.file_name().unwrap())
                    .join(&archive);
                if source_archive.is_file() && !dest_archive.exists() {
                    // Archive files are immutable; avoid copying when possible.
                    if fs::hard_link(&source_archive, &dest_archive).is_err() {
                        fs::copy(source_archive, &dest_archive).unwrap();
                    }
                }
            }
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    String::from_utf8(output.stdout).unwrap()
}

fn sparse_index_path(name: &str) -> PathBuf {
    let name = name.to_ascii_lowercase();
    match name.len() {
        1 => Path::new("1").join(name),
        2 => Path::new("2").join(name),
        3 => Path::new("3").join(&name[..1]).join(&name),
        _ => Path::new(&name[..2]).join(&name[2..4]).join(&name),
    }
}

fn failure(output: Output) -> String {
    assert!(
        !output.status.success(),
        "unexpected success: {}",
        String::from_utf8_lossy(&output.stdout),
    );
    String::from_utf8(output.stderr).unwrap()
}

struct Registry {
    url: String,
    stop: mpsc::Sender<()>,
    server: Option<thread::JoinHandle<()>>,
}

impl Registry {
    fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let mut routes = BTreeMap::from([(
            "/config.json".to_owned(),
            serde_json::json!({"dl": format!("{url}/crates/{{crate}}/{{version}}")})
                .to_string()
                .into_bytes(),
        )]);
        for (name, value) in [(APP_CRATE, 17), (TOOL_CRATE, 25)] {
            let archive = crate_archive(name, value);
            let checksum = format!("{:x}", Sha256::digest(&archive));
            let entry = serde_json::json!({
                "name": name, "vers": VERSION, "deps": [], "cksum": checksum,
                "features": {}, "yanked": false,
            });
            routes.insert(format!("/te/rr/{name}"), format!("{entry}\n").into_bytes());
            routes.insert(format!("/crates/{name}/{VERSION}"), archive);
        }
        let (stop, stopped) = mpsc::channel();
        let server = thread::spawn(move || {
            loop {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        stream
                            .set_read_timeout(Some(Duration::from_secs(2)))
                            .unwrap();
                        stream
                            .set_write_timeout(Some(Duration::from_secs(2)))
                            .unwrap();
                        let mut request = BufReader::new(&mut stream);
                        let mut line = String::new();
                        request.read_line(&mut line).unwrap();
                        let path = line.split_whitespace().nth(1).unwrap();
                        let (status, body) = routes.get(path).map_or_else(
                            || ("404 Not Found", &[][..]),
                            |body| ("200 OK", body.as_slice()),
                        );
                        // Drain headers before closing so TCP doesn't discard the response.
                        loop {
                            line.clear();
                            if request.read_line(&mut line).unwrap() == 0 || line == "\r\n" {
                                break;
                            }
                        }
                        write!(
                            stream,
                            "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            body.len(),
                        )
                        .unwrap();
                        stream.write_all(body).unwrap();
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if stopped.recv_timeout(Duration::from_millis(5)).is_ok() {
                            break;
                        }
                    }
                    Err(error) => panic!("accepting local registry request: {error}"),
                }
            }
        });
        Self {
            url,
            stop,
            server: Some(server),
        }
    }

    fn configure(&self, fixture: &Fixture, home: &str) {
        fixture.write(
            &format!("{home}/config.toml"),
            format!(
                "[source.crates-io]\nreplace-with = 'fixture-registry'\n\
                 [source.fixture-registry]\nregistry = 'sparse+{}/'\n",
                self.url,
            ),
        );
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        let result = self.server.take().unwrap().join();
        if !thread::panicking() {
            result.unwrap();
        }
    }
}

fn crate_archive(name: &str, value: u8) -> Vec<u8> {
    let mut archive = tar::Builder::new(GzEncoder::new(Vec::new(), Compression::fast()));
    for (file, contents) in [
        (
            "Cargo.toml",
            format!("[package]\nname = '{name}'\nversion = '{VERSION}'\nedition = '2024'\n"),
        ),
        (
            "src/lib.rs",
            format!("pub fn value() -> i64 {{ {value} }}\n"),
        ),
    ] {
        let mut header = tar::Header::new_gnu();
        header.set_mode(0o644);
        header.set_size(u64::try_from(contents.len()).unwrap());
        archive
            .append_data(
                &mut header,
                format!("{name}-{VERSION}/{file}"),
                contents.as_bytes(),
            )
            .unwrap();
    }
    archive.into_inner().unwrap().finish().unwrap()
}

fn write_application_and_consumer(fixture: &Fixture) {
    fixture.write(
        "app/package.toml",
        format!(
            "package = 'fetch-app'\n[namespaces]\napp = 'src'\n\n\
             [rust-dependencies.{APP_CRATE}]\nversion = '={VERSION}'\n\n\
             [terrane-dependencies.fixture.provider]\npath = '../provider'\n\n\
             [consumers.generator]\ntool = 'fixture/provider::generate'\ndeclarations = ['/app::seed']\n"
        ),
    );
    fixture.write(
        "app/src/main.trn",
        concat!(
            "namespace app\nfrom /generated/fetch import tool-value\n\n",
            "public function seed int;\n  return 0\n\n",
            "function main;\n",
            "  app-value int = rust\n",
            "    terrane_int_support::Int::from(terrane_fetch_app_witness::value())\n",
            "  print; (app-value + (tool-value;))\n",
        ),
    );
    fixture.write(
        "provider/package.toml",
        "package = 'fixture/provider'\nartifact = 'library'\n[namespaces]\n'fixture/provider' = 'src'\n\
         [consumer-tools]\ngenerate = 'generator/package.toml'\n",
    );
    fixture.write(
        "provider/src/provider.trn",
        "namespace fixture/provider\npublic constant witness = 0\n",
    );
    fixture.write(
        "provider/generator/package.toml",
        format!(
            "package = 'fetch-generator'\n[namespaces]\ngenerator = 'src'\n\n\
             [rust-dependencies.{TOOL_CRATE}]\nversion = '={VERSION}'\n\n\
             [rust-modules]\nresponse = 'response.rs'\n"
        ),
    );
    fixture.write(
        "provider/generator/src/main.trn",
        "namespace generator\nfunction main;\n  rust\n    crate::response::write();\n",
    );
    fixture.write(
        "provider/generator/response.rs",
        r###"
use std::io::Read as _;

pub fn write() {
    let mut request = String::new();
    std::io::stdin().read_to_string(&mut request).unwrap();
    assert!(request.contains("/app::seed"));
    let value = terrane_fetch_tool_witness::value();
    println!(
        r##"{{"format":1,"generated_sources":[{{"identity":"value","source":"namespace generated/fetch\npublic function tool-value int;\n  return {value}\n"}}],"diagnostics":[]}}"##
    );
}
"###,
    );
}

#[test]
fn fetch_warms_locked_application_and_consumer_crates_without_running_the_consumer() {
    let fixture = Fixture::new();
    let registry = Registry::new();
    registry.configure(&fixture, "seed-home");
    registry.configure(&fixture, "test-home");
    write_application_and_consumer(&fixture);
    let app_lock = fixture.seed_lock("app", APP_CRATE);
    let tool_lock = fixture.seed_lock("provider/generator", TOOL_CRATE);
    let check_locks = || {
        assert_eq!(
            fs::read(fixture.0.join("app").join(LOCK)).unwrap(),
            app_lock
        );
        assert_eq!(
            fs::read(fixture.0.join("provider/generator").join(LOCK)).unwrap(),
            tool_lock,
        );
    };
    let cache = fixture.0.join("test-home/registry/cache");
    assert!(!cache.exists());

    success(fixture.terrane("test-home", false, &["fetch", "app"]));
    check_locks();
    assert!(!fixture.0.join("app/.trn/generated").exists());
    assert!(!fixture.0.join("provider/generator/.trn/build").exists());
    let mut mirrors = fs::read_dir(&cache).unwrap();
    let mirror = mirrors.next().unwrap().unwrap().path();
    assert!(mirrors.next().is_none(), "unexpected second Cargo registry");
    for name in [APP_CRATE, TOOL_CRATE] {
        assert!(
            mirror.join(format!("{name}-{VERSION}.crate")).is_file(),
            "{name} is not cached in the isolated Cargo home",
        );
    }

    drop(registry);
    success(fixture.terrane("test-home", true, &["fetch", "app/package.toml"]));
    check_locks();

    let app_lock_path = fixture.0.join("app").join(LOCK);
    let original = std::str::from_utf8(&app_lock).unwrap();
    let parsed: toml::Value = original.parse().unwrap();
    let checksum = parsed["package"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"].as_str() == Some(APP_CRATE))
        .unwrap()["checksum"]
        .as_str()
        .unwrap();
    let corrupt = original.replace(checksum, &"0".repeat(64));
    fs::write(&app_lock_path, &corrupt).unwrap();
    let error = failure(fixture.terrane("test-home", true, &["fetch", "app"]));
    assert!(error.contains("checksum"), "{error}");
    assert_eq!(fs::read_to_string(&app_lock_path).unwrap(), corrupt);
    fs::write(app_lock_path, &app_lock).unwrap();
    assert!(!fixture.0.join("app/.trn/generated").exists());

    fixture.seed_runtime_cache("test-home", &mirror);

    let artifact = success(fixture.terrane("test-home", true, &["build", "app"]));
    let executable = fixture.0.join(artifact.trim());
    assert_eq!(success(Command::new(executable).output().unwrap()), "42\n");
    assert!(fixture.0.join("app/.trn/generated/generator").is_dir());
    check_locks();
}

#[test]
fn fetch_rejects_a_native_package_without_a_lock() {
    let fixture = Fixture::new();
    fixture.write(
        "package.toml",
        format!(
            "package = 'unlocked-fetch'\n[namespaces]\napp = 'src'\n\n\
             [rust-dependencies.{APP_CRATE}]\nversion = '={VERSION}'\n"
        ),
    );
    fixture.write("src/main.trn", "namespace app\nfunction main;\n  return\n");
    // Offline even on failure: this case must not try to resolve a missing lock.
    let error = failure(fixture.terrane("test-home", true, &["fetch", "package.toml"]));
    assert!(error.contains("missing") && error.contains(LOCK), "{error}");
    assert!(!fixture.0.join(LOCK).exists());
}
