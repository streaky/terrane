use std::fs;
use std::io::{BufRead as _, BufReader, Write as _};
use std::net::{SocketAddr, TcpListener};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

const MANIFEST: &str = "package = \"acme/library\"\nartifact = \"library\"\n[namespaces]\n\"acme/library\" = \"src\"\n";
const SOURCE: &str = "namespace acme/library\nfunction answer int;\n  return 42\n";

struct Workspace(PathBuf);
impl Workspace {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "terrane-archive-cli-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("library/src")).unwrap();
        fs::write(root.join("library/package.toml"), MANIFEST).unwrap();
        fs::write(root.join("library/src/main.trn"), SOURCE).unwrap();
        Self(root)
    }
    fn application(&self, name: &str) -> PathBuf {
        let root = self.0.join(name);
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("package.toml"),
            "package = \"app\"\n[namespaces]\napp = \"src\"\n",
        )
        .unwrap();
        fs::write(
            root.join("src/main.trn"),
            "namespace app\nfrom /acme/library import answer\nfunction main;\n  print; (answer;)\n",
        )
        .unwrap();
        root
    }
}
impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn archive(gzip: bool) -> Vec<u8> {
    let files = [
        ("release/package.toml", MANIFEST),
        ("release/src/main.trn", SOURCE),
    ];
    if gzip {
        let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        let mut tar = tar::Builder::new(encoder);
        for (name, text) in files {
            let mut header = tar::Header::new_gnu();
            header.set_size(text.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append_data(&mut header, name, text.as_bytes()).unwrap();
        }
        tar.into_inner().unwrap().finish().unwrap()
    } else {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        for (name, text) in files {
            zip.start_file(name, zip::write::SimpleFileOptions::default())
                .unwrap();
            zip.write_all(text.as_bytes()).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }
}

fn git_archive(library: &Path) -> Vec<u8> {
    fs::create_dir_all(library.join(".trn")).unwrap();
    fs::write(library.join(".trn/cache"), "unhashed build metadata").unwrap();
    for arguments in [
        vec!["init", "--quiet"],
        vec!["add", "."],
        vec![
            "-c",
            "user.name=Archive Test",
            "-c",
            "user.email=archive@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "--quiet",
            "-m",
            "Package source",
        ],
    ] {
        success(
            &Command::new("git")
                .arg("-C")
                .arg(library)
                .args(arguments)
                .output()
                .unwrap(),
        );
    }
    let output = Command::new("git")
        .arg("-C")
        .arg(library)
        .args(["archive", "--format=tar.gz", "--prefix=release/", "HEAD"])
        .output()
        .unwrap();
    success(&output);
    output.stdout
}

struct Server {
    address: SocketAddr,
    stop: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<()>>,
}
impl Server {
    fn new(bytes: Vec<u8>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        listener.set_nonblocking(true).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let stopping = stop.clone();
        let worker = thread::spawn(move || {
            while !stopping.load(Ordering::Acquire) {
                let (mut stream, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                        continue;
                    }
                    Err(error) => panic!("{error}"),
                };
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut reader = BufReader::new(&stream);
                let mut request = String::new();
                reader.read_line(&mut request).unwrap();
                loop {
                    let mut header = String::new();
                    if reader.read_line(&mut header).unwrap() == 0 || header == "\r\n" {
                        break;
                    }
                }
                drop(reader);
                let path = request.split_whitespace().nth(1).unwrap();
                if path == "/redirect" {
                    write!(stream, "HTTP/1.1 302 Found\r\nLocation: /asset\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
                } else if path == "/asset" {
                    write!(
                        stream,
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        bytes.len()
                    )
                    .unwrap();
                    stream.write_all(&bytes).unwrap();
                } else {
                    write!(
                        stream,
                        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                    )
                    .unwrap();
                }
            }
        });
        Self {
            address,
            stop,
            worker: Some(worker),
        }
    }
    fn url(&self, path: &str) -> String {
        format!("http://{}{path}", self.address)
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        self.worker.take().unwrap().join().unwrap();
    }
}

fn cli(root: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terrane"))
        .current_dir(root)
        .args(arguments)
        .output()
        .unwrap()
}
fn success(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn archive_hash_install_and_execution_preserve_identity_and_failed_writes() {
    let workspace = Workspace::new();
    let expected = terrane_compiler::source_tree_hash(&workspace.0.join("library")).unwrap();
    for (name, bytes) in [
        ("zip-app", archive(false)),
        ("tar-app", archive(true)),
        ("git-tar-app", git_archive(&workspace.0.join("library"))),
    ] {
        let app = workspace.application(name);
        let server = Server::new(bytes);
        let url = server.url("/redirect");
        let hash = cli(&app, &["package", "hash", "--archive", &url]);
        success(&hash);
        assert_eq!(String::from_utf8(hash.stdout).unwrap().trim(), expected);
        let install = cli(&app, &["package", "install", "--archive", &url]);
        success(&install);
        let original = fs::read(app.join("package.toml")).unwrap();
        let manifest: toml::Value =
            toml::from_str(std::str::from_utf8(&original).unwrap()).unwrap();
        let entry = &manifest["terrane-dependencies"]["acme"]["library"];
        assert_eq!(entry["archive"].as_str(), Some(url.as_str()));
        assert_eq!(entry["hash"].as_str(), Some(expected.as_str()));
        assert!(
            !cli(&app, &["package", "install", "--archive", &url])
                .status
                .success()
        );
        assert_eq!(fs::read(app.join("package.toml")).unwrap(), original);
        let run = cli(&app, &["run", "package.toml"]);
        success(&run);
        assert_eq!(run.stdout, b"42\n");
        let cache = app
            .join(".trn/packages")
            .join(expected.trim_start_matches("sha256:"));
        assert!(!cache.join(".trn").exists());
        assert!(!cache.join(".git").exists());
        let missing = server.url("/missing");
        assert!(
            !cli(&app, &["package", "install", "--archive", &missing])
                .status
                .success()
        );
        assert_eq!(fs::read(app.join("package.toml")).unwrap(), original);
    }
}
