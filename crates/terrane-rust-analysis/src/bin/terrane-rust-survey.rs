use std::path::Path;
fn main() {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: terrane-rust-survey <rustdoc-json> <toolchain>");
        std::process::exit(2);
    };
    let Some(toolchain) = args.next() else {
        eprintln!("usage: terrane-rust-survey <rustdoc-json> <toolchain>");
        std::process::exit(2);
    };
    if args.next().is_some() {
        eprintln!("usage: terrane-rust-survey <rustdoc-json> <toolchain>");
        std::process::exit(2);
    }
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| fail(&format!("cannot read `{path}`: {error}")));
    let package = Path::new(&path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("rustdoc");
    let document = terrane_rust_analysis::parse_rustdoc(package, &bytes, &toolchain)
        .unwrap_or_else(|error| fail(&error.message));
    let paths = terrane_rust_analysis::public_paths(&document);
    println!(
        "{}",
        serde_json::json!({"rustdoc-format": document.format_version, "public-item-count": paths.len(), "public-paths": paths.values().collect::<Vec<_>>() })
    );
}
fn fail(message: &str) -> ! {
    eprintln!("terrane-rust-survey: {message}");
    std::process::exit(1)
}
