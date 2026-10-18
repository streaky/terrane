#!/usr/bin/env python3
"""Collect Rust implementation coverage, including generated-program execution."""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import datetime as dt
import hashlib
import html
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]


def checked(command, **kwargs):
    return subprocess.run([str(arg) for arg in command], check=True, **kwargs)


def llvm_tools():
    version = subprocess.check_output(["rustc", "-vV"], cwd=ROOT, text=True)
    expected = re.search(r"LLVM version: (\S+)", version).group(1)
    sysroot = Path(subprocess.check_output(["rustc", "--print", "sysroot"], cwd=ROOT, text=True).strip())
    host = re.search(r"host: (\S+)", version).group(1)
    tools = []
    for name in ("llvm-profdata", "llvm-cov"):
        bundled = sysroot / "lib/rustlib" / host / "bin" / name
        path = str(bundled) if bundled.is_file() else shutil.which(name)
        if not path:
            raise ValueError("Missing LLVM tools; run rustup component add llvm-tools-preview")
        actual = subprocess.check_output([path, "--version"], text=True)
        if not re.search(r"LLVM version " + re.escape(expected) + r"(?:\s|$)", actual):
            raise ValueError(f"{path} must match rustc LLVM {expected}; install llvm-tools-preview")
        tools.append(path)
    return tools


def environment(directory, target):
    env = os.environ.copy()
    # Encoded flags take precedence over RUSTFLAGS in Cargo. Normalize once so
    # nested test harnesses can preserve the same instrumentation contract.
    flags = env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    if flags is not None and any(any(char.isspace() for char in flag) for flag in flags.split("\x1f")):
        raise ValueError("Coverage requires Rust flag arguments without embedded whitespace")
    flags = " ".join(flags.split("\x1f")) if flags is not None else env.get("RUSTFLAGS", "")
    env["RUSTFLAGS"] = flags
    env["TERRANE_COVERAGE_TARGET_DIR"] = str(target)
    env["CARGO_TARGET_DIR"] = str(target)
    env["TERRANE_COVERAGE_DIR"] = str(directory)
    env["LLVM_PROFILE_FILE"] = str(directory / "profiles/%m-%p.profraw")
    env["CARGO_TERM_PROGRESS_WHEN"] = "never"
    return env


def build_tests(cargo_args, env, directory):
    command = ["cargo", "test", "--no-run", "--message-format=json", *(cargo_args or ["--workspace"])]
    objects = []
    binaries = []
    with (directory / "build.jsonl").open("w") as log:
        process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=subprocess.PIPE, text=True)
        for line in process.stdout:
            log.write(line)
            try:
                message = json.loads(line)
            except json.JSONDecodeError:
                continue
            if message.get("reason") == "compiler-message":
                print(message["message"].get("rendered", ""), end="", file=sys.stderr)
            if message.get("reason") != "compiler-artifact" or not message.get("executable"):
                continue
            path = Path(message["executable"])
            if "custom-build" not in message["target"]["kind"]:
                with path.open("rb") as executable:
                    digest = hashlib.file_digest(executable, "sha256").hexdigest()
                saved = directory / "aggregate/objects" / f"{digest}-{path.name}"
                saved.parent.mkdir(parents=True, exist_ok=True)
                if not saved.exists():
                    shutil.copy2(path, saved)
                objects.append(saved)
            if message["profile"].get("test") and "custom-build" not in message["target"]["kind"]:
                cwd = Path(message["manifest_path"]).parent
                binaries.append((f'{cwd.name}::{message["target"]["name"]}', path, cwd))
        code = process.wait()
    if code:
        raise subprocess.CalledProcessError(code, command)
    return sorted(set(binaries)), sorted(set(objects))


def response_file(path, arguments):
    path.write_text("\n".join(json.dumps(str(arg), ensure_ascii=False) for arg in arguments) + "\n")
    return "@" + str(path)


def coverage_command(command, directory, **kwargs):
    result = subprocess.run([str(arg) for arg in command], stderr=subprocess.PIPE, text=True, **kwargs)
    if result.stderr:
        print(result.stderr, end="", file=sys.stderr)
        with (directory / "warnings.txt").open("a") as stream:
            stream.write(result.stderr)
    result.check_returncode()
    return result


def report(directory, objects, tools, detailed=False):
    profiles = sorted((directory / "profiles").rglob("*.profraw"))
    if not profiles:
        return None
    profdata = directory / "coverage.profdata"
    checked([tools[0], "merge", "-sparse", response_file(directory / "profiles.rsp", profiles), "-o", profdata])
    objects = sorted(set(objects) | set((directory / "objects").glob("*")))
    if not objects:
        raise ValueError(f"No coverage executable objects in {directory}")
    sources = sorted((ROOT / "crates").glob("*/src/**/*.rs"))
    arguments = [objects[0], "-instr-profile=" + str(profdata)]
    for obj in objects[1:]:
        arguments.extend(["-object", obj])
    arguments.extend(sources)
    args = response_file(directory / "objects.rsp", arguments)
    output = directory / "html"
    coverage_command([tools[1], "show", args, "-format=html", "-output-dir=" + str(output), "-show-directory-coverage", "-show-line-counts-or-regions", "-show-instantiations=false"], directory)
    with (directory / "coverage.json").open("w") as stream:
        coverage_command([tools[1], "export", args, *([] if detailed else ["-summary-only"])], directory, stdout=stream)
    data = json.loads((directory / "coverage.json").read_text())
    files = [entry for unit in data["data"] for entry in unit["files"]]
    # Explicit filenames above keep dependencies, generated Rust and integration
    # test sources out. Unbuilt source files remain visible as unmeasured inventory.
    measured = {str(Path(entry["filename"]).resolve()) for entry in files}
    missing = [str(path.relative_to(ROOT)) for path in sources if str(path) not in measured]
    (directory / "unmeasured.json").write_text(json.dumps(missing, indent=2) + "\n")
    if detailed:
        functions = {
            json.dumps([str(Path(function["filenames"][function["regions"][0][5]]).resolve()), *function["regions"][0][:4]])
            for unit in data["data"]
            for function in unit.get("functions", [])
            if function["count"] > 0
            and any(str(Path(name).resolve()) in measured for name in function["filenames"])
        }
        (directory / "functions.json").write_text(json.dumps(sorted(functions)) + "\n")
    return files, missing


def annotate_overlap(directory, records):
    records = [record for record in records if record.get("coverage_available", True)]
    footprints = [
        set(json.loads((directory / record["path"] / "functions.json").read_text()))
        for record in records
    ]
    owners = defaultdict(list)
    for index, functions in enumerate(footprints):
        for function in functions:
            owners[function].append(index)
    for index, (record, functions) in enumerate(zip(records, footprints)):
        record["functions"] = len(functions)
        record["unique_functions"] = sum(len(owners[function]) == 1 for function in functions)
        shared = Counter(other for function in functions for other in owners[function] if other != index)
        if shared:
            other, count = shared.most_common(1)[0]
            record["overlap"] = f'{count} shared with {records[other]["name"]}'
        else:
            record["overlap"] = "No shared functions"


def page(directory, records, missing, metadata):
    esc = html.escape
    rows = []
    for record in records:
        available = record.get("coverage_available", True)
        link = record["path"] + "/html/index.html" if available else record["log"]
        count = record.get("functions") if metadata["per_test"] and available else "—"
        unique = record.get("unique_functions", "—") if available else "—"
        overlap = record.get("overlap", "—") if available else "Unavailable"
        note = record.get("coverage_unavailable_reason", "")
        name = esc(record["name"]) + (f"<br><small>{esc(note)}</small>" if note else "")
        log = esc(record["log"], quote=True)
        rows.append(f'<tr><td><a href="{esc(link, quote=True)}">{name}</a></td><td><a href="{log}">{record["status"]}</a></td><td>{record["seconds"]:.2f}</td><td>{count}</td><td>{unique}</td><td>{esc(overlap)}</td></tr>')
    missing_rows = "".join(f"<li>{esc(path)}</li>" for path in missing)
    document = '''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width">
<title>Terrane Rust coverage</title><style>
:root{color-scheme:light dark}body{font:16px system-ui;max-width:1200px;margin:2rem auto;padding:0 1rem}a{color:light-dark(#1458a5,#8fc5ff)}table{border-collapse:collapse;width:100%}th,td{text-align:left;padding:.65rem;border-bottom:1px solid #8886}th{cursor:pointer}input{font:inherit;padding:.5rem;width:90%}code{overflow-wrap:anywhere}.note{border-left:3px solid #888;padding-left:1rem}
</style><h1>Terrane Rust implementation coverage</h1>'''
    document += f'<p>{esc(metadata["created"])} · <code>{esc(metadata["revision"])}</code></p>'
    document += f'<p><strong>{esc(metadata["scope"])}</strong> · {"FAILED" if metadata["exit_code"] else "Passed"} · {"dirty working tree" if metadata["dirty"] else "committed tree"}</p>'
    if metadata.get("aggregate_available", True):
        document += '<p><a href="aggregate/html/index.html">Browse aggregate source coverage</a> · <a href="aggregate/coverage.json">LLVM JSON</a> · <a href="run.json">Run metadata</a></p>'
    else:
        document += '<p><a href="run.json">Run metadata</a></p>'
        document += f'<p class="note">Coverage unavailable: {esc(metadata["coverage_unavailable_reason"])}</p>'
    document += '<p class="note">Handwritten Rust under crates/*/src only. Includes unit-test bodies colocated in those files. Generated Rust, Terrane source, integration-test files and third-party dependencies are excluded. Unmeasured files below are not counted as covered or silently represented as zero executable lines. This is execution evidence, not assertion quality or proof that a test is redundant. Stable instrumentation reports line, region and function coverage, not a Terrane branch metric.</p>'
    for warning in metadata.get("coverage_warnings", []):
        document += f'<p class="note"><strong>Coverage fidelity warning:</strong> <a href="{esc(warning, quote=True)}">LLVM diagnostics</a>. Mapping warnings mean these totals are not a clean baseline.</p>'
    document += f'<h2>Test executions ({len(records)})</h2><p>Click a test to browse its source coverage. Click a heading to sort; filter by name below. Per-test mode shows covered functions, functions unique within this selection, and the most overlapping test (shared function count). Aggregate mode only records duration/status per binary. Shared setup and compiler traversal are included. Conformance attribution is at the outer Rust harness-test granularity, not individual Terrane cases.</p><input id="filter" aria-label="Filter test executions" placeholder="Filter tests"><table><thead><tr><th>Name</th><th>Status</th><th>Seconds</th><th>Functions exercised</th><th>Unique functions</th><th>Most overlap</th></tr></thead><tbody>{"".join(rows)}</tbody></table>'
    document += f'<h2>Unmeasured source files ({len(missing)})</h2><p>Absent from the selected build’s coverage mappings; these are coverage inventory gaps, not evidence of coverage.</p><ul>{missing_rows}</ul>'
    document += '''<script>
const body=document.querySelector('tbody');document.querySelector('#filter').addEventListener('input',e=>{for(const row of body.rows)row.hidden=!row.cells[0].textContent.toLowerCase().includes(e.target.value.toLowerCase())});
for(const [i,heading] of [...document.querySelectorAll('th')].entries()){let ascending=false;heading.tabIndex=0;const sort=()=>{ascending=!ascending;const rows=[...body.rows];rows.sort((a,b)=>{const x=a.cells[i].textContent,y=b.cells[i].textContent;return (ascending?1:-1)*(i>=2&&i<=4?Number(x)-Number(y):x.localeCompare(y))});body.append(...rows)};heading.addEventListener('click',sort);heading.addEventListener('keydown',e=>{if(e.key==='Enter')sort()})}
</script></html>'''
    (directory / "index.html").write_text(document)


def current_warning_files(directory):
    candidates = [directory / "aggregate/warnings.txt", *(directory / "tests").glob("*/warnings.txt")]
    return sorted(str(path.relative_to(directory)) for path in candidates if path.is_file())


def prepare_output(directory):
    directory.mkdir(parents=True, exist_ok=True)
    marker = directory / ".terrane-coverage-output"
    artifacts = [
        directory / name
        for name in (
            "aggregate", "tests", "build-profiles", "discovery-profiles",
            "index.html", "run.json", "build.jsonl",
        )
    ]
    owned = marker.is_file() and marker.read_text() == "terrane-rust-coverage-v1\n"
    if not owned and any(path.exists() or path.is_symlink() for path in artifacts):
        raise ValueError(f"Refusing to overwrite unrelated contents in {directory}")
    for path in artifacts:
        if path.is_dir() and not path.is_symlink():
            shutil.rmtree(path)
        else:
            path.unlink(missing_ok=True)
    marker.write_text("terrane-rust-coverage-v1\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "target/coverage", help="exact report directory (default: target/coverage); replaces previous report, retaining build cache")
    parser.add_argument("--per-test", action="store_true", help="run each libtest separately and generate individual HTML reports")
    parser.add_argument("--filter", default="", help="libtest name substring, applied to each test executable")
    parser.add_argument("cargo_args", nargs=argparse.REMAINDER, help="Cargo selection after -- (default --workspace)")
    args = parser.parse_args()
    cargo_args = args.cargo_args
    if cargo_args[:1] == ["--"]:
        cargo_args = cargo_args[1:]
    tools = llvm_tools()
    directory = args.output.resolve()
    prepare_output(directory)
    aggregate = directory / "aggregate"
    aggregate.mkdir()
    target = directory / "build"
    env = environment(aggregate, target)
    env["RUSTFLAGS"] += " -C instrument-coverage"
    env["LLVM_PROFILE_FILE"] = str(directory / "build-profiles/%m-%p.profraw")
    metadata = {
        "created": dt.datetime.now(dt.timezone.utc).isoformat(),
        "revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "dirty": bool(subprocess.check_output(["git", "status", "--porcelain"], cwd=ROOT, text=True)),
        "cargo_args": cargo_args or ["--workspace"],
        "filter": args.filter,
        "per_test": args.per_test,
        "doctests": "excluded (stable libtest executable collection)",
        "rustc": subprocess.check_output(["rustc", "-vV"], cwd=ROOT, text=True),
    }
    metadata["environment"] = {key: value for key, value in os.environ.items() if key.startswith("TERRANE_") or key in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "TMPDIR"}}
    metadata["llvm_tools"] = tools
    print(f"Coverage run: {directory}", flush=True)
    binaries, objects = build_tests(cargo_args, env, directory)
    if not binaries:
        raise ValueError("Cargo selection produced no test executables")
    records = []
    failed = False
    env = environment(aggregate, target)
    env["LLVM_PROFILE_FILE"] = str(directory / "discovery-profiles/%m-%p.profraw")
    env.pop("CARGO_TARGET_DIR", None)
    for target_name, binary, cwd in binaries:
        listing = subprocess.check_output([str(binary), "--list", "--format=terse", args.filter], env=env, cwd=cwd, text=True)
        selected = [line.removesuffix(": test") for line in listing.splitlines() if line.endswith(": test")]
        if not selected:
            continue
        if args.per_test:
            ignored = subprocess.check_output([str(binary), "--list", "--ignored", "--format=terse", args.filter], env=env, cwd=cwd, text=True)
            ignored_names = {line.removesuffix(": test") for line in ignored.splitlines() if line.endswith(": test")}
            selections = [name for name in selected if name not in ignored_names]
        else:
            selections = [None]
        for selection in selections:
            name = f"{target_name}::{selection}" if selection else target_name
            slug = hashlib.sha256((str(binary) + "::" + str(selection)).encode()).hexdigest()[:20]
            relative = "tests/" + slug
            work = directory / relative
            work.mkdir(parents=True)
            run_env = environment(work if args.per_test else aggregate, target)
            # Nested Cargo/projection probes own their target directories. A
            # shared outer override can escape sandbox write mounts and caches.
            run_env.pop("CARGO_TARGET_DIR", None)
            command = [str(binary)]
            if selection is not None:
                command += [selection, "--exact"]
            elif args.filter:
                command += [args.filter]
            start = time.monotonic()
            print(f"Running {name}", flush=True)
            with (work / "output.txt").open("w") as log:
                result = subprocess.run(command, env=run_env, cwd=cwd, stdout=log, stderr=subprocess.STDOUT)
            seconds = time.monotonic() - start
            failed |= result.returncode != 0
            record = {"name": name, "path": relative if args.per_test else "aggregate", "log": relative + "/output.txt", "status": "passed" if result.returncode == 0 else "failed", "returncode": result.returncode, "seconds": seconds, "functions": None}
            if args.per_test:
                record["coverage_available"] = report(work, objects, tools, detailed=True) is not None
                if not record["coverage_available"]:
                    failed = True
                    record["coverage_unavailable_reason"] = "No execution profile; the process may have crashed or terminated before flushing counters."
            records.append(record)
    if not records:
        raise ValueError("No non-ignored tests matched the selection")
    if args.per_test:
        # Profiles retain their original per-test directories; hardlink them into
        # the aggregate input without modifying or coalescing attribution.
        for profile in (directory / "tests").rglob("*.profraw"):
            destination = aggregate / "profiles" / profile.relative_to(directory / "tests")
            destination.parent.mkdir(parents=True, exist_ok=True)
            os.link(profile, destination)
        for obj in (directory / "tests").rglob("objects/*"):
            destination = aggregate / "objects" / obj.name
            destination.parent.mkdir(parents=True, exist_ok=True)
            if not destination.exists():
                os.link(obj, destination)
    aggregate_report = report(aggregate, objects, tools)
    metadata["aggregate_available"] = aggregate_report is not None
    if aggregate_report is None:
        failed = True
        missing = [str(path.relative_to(ROOT)) for path in sorted((ROOT / "crates").glob("*/src/**/*.rs"))]
        (aggregate / "unmeasured.json").write_text(json.dumps(missing, indent=2) + "\n")
        metadata["coverage_unavailable_reason"] = "No test execution profiles were produced; coverage is unavailable, not zero."
    else:
        _, missing = aggregate_report
    if not args.per_test:
        for record in records:
            record["coverage_available"] = metadata["aggregate_available"]
    if args.per_test:
        annotate_overlap(directory, records)
    metadata["tests"] = records
    metadata["exit_code"] = 1 if failed else 0
    metadata["scope"] = "partial selection" if args.filter or cargo_args or os.environ.get("TERRANE_CONFORMANCE_FILTER") else "workspace libtest suite (no doctests)"
    metadata["coverage_warnings"] = current_warning_files(directory)
    (directory / "run.json").write_text(json.dumps(metadata, indent=2) + "\n")
    page(directory, records, missing, metadata)
    print(f"HTML report: {directory / 'index.html'}", flush=True)
    return metadata["exit_code"]


def cli():
    try:
        return main()
    except (ValueError, subprocess.CalledProcessError) as error:
        print(f"Coverage failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(cli())
