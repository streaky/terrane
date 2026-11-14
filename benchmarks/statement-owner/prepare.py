#!/usr/bin/env python3
"""Build unchanged Rust emitted by one of two explicitly selected compilers."""
from pathlib import Path
import argparse
import filecmp
import hashlib
import json
import os
import shutil
import subprocess
import sys

SUITE = Path(__file__).resolve().parent
REPO = SUITE.parents[1]
TOOLCHAIN = "1.100.0"


def write_changed(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    if not path.exists() or path.read_text() != text:
        path.write_text(text)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("version", choices=("before", "after"))
    args = parser.parse_args()
    variable = f"TERRANE_BENCH_{args.version.upper()}"
    if variable not in os.environ:
        raise SystemExit(f"Set {variable} to the corresponding current compiler executable")
    source = args.source.resolve()
    problem = source.parents[1].name
    emitted = SUITE / ".trn/emitted" / problem / args.version
    emitted.mkdir(parents=True, exist_ok=True)
    generated = subprocess.run([os.environ[variable], "rust", "--require-canonical-rust",
                                "-o", str(emitted / "program.rs"), str(source)],
                               cwd=REPO, text=True, capture_output=True)
    print(generated.stderr, file=sys.stderr, end="")
    if generated.returncode:
        raise SystemExit(generated.returncode)
    if "compiler predates" in generated.stderr:
        raise SystemExit("Refresh the selected compiler before benchmarking")
    # Both versions use the SAME crate identity, source path, flags and dependencies.
    # Keep each original emission separately; never patch emitted Rust.
    build = SUITE / ".trn/crates" / problem
    for file in emitted.glob("*.rs"):
        write_changed(build / "src" / file.name, file.read_text())
    package = f"statement-owner-{problem}"
    dependencies = "\n".join(f'{name} = {{ path = {json.dumps(str(REPO / "crates" / name))} }}' for name in (
        "terrane-int-support", "terrane-collection-support", "terrane-scalar-support",
        "terrane-string-support", "terrane-platform-support", "terrane-signal-support"))
    write_changed(build / "Cargo.toml", f'''[package]
name = "{package}"
version = "0.0.0"
edition = "2024"
[workspace]
[[bin]]
name = "{package}"
path = "src/program.rs"
[dependencies]
{dependencies}
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
''')
    if not (build / "Cargo.lock").exists():
        shutil.copyfile(REPO / "Cargo.lock", build / "Cargo.lock")
    environment = dict(os.environ)
    environment.pop("RUSTC_BOOTSTRAP", None)
    environment["CARGO_ENCODED_RUSTFLAGS"] = "-Dwarnings"
    target = SUITE / ".trn/target"
    subprocess.run(["cargo", f"+{TOOLCHAIN}", "build", "--release", "--offline", "--quiet",
                    "--manifest-path", str(build / "Cargo.toml"), "--target-dir", str(target)],
                   cwd=REPO, env=environment, check=True)
    binary = SUITE / ".trn/bin" / f"{problem}-{args.version:_<6}"
    binary.parent.mkdir(parents=True, exist_ok=True)
    binary.unlink(missing_ok=True)
    shutil.copyfile(target / "release" / package, binary)
    binary.chmod(0o755)
    counterpart = binary.with_name(f"{problem}-{'after_' if args.version == 'before' else 'before'}")
    if counterpart.exists() and filecmp.cmp(binary, counterpart, shallow=False):
        binary.unlink()
        os.link(counterpart, binary)
    launcher = SUITE / ".trn/launch"
    source_digest = hashlib.sha256((SUITE / "launch.rs").read_bytes()).hexdigest()
    stamp = SUITE / ".trn/launch.sha256"
    if not launcher.exists() or not stamp.exists() or stamp.read_text() != source_digest:
        subprocess.run(["rustc", f"+{TOOLCHAIN}", "--edition=2024", "-O", "-Dwarnings",
                        str(SUITE / "launch.rs"), "-o", str(launcher)], check=True, cwd=REPO)
        stamp.write_text(source_digest)
    print(binary)


if __name__ == "__main__":
    main()
