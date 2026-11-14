#!/usr/bin/env python3
"""Prepare, record, and consume LLVM sample profiles for Rust builds."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import re
import subprocess
import sys
import tempfile
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[1]
SEPARATOR = "\x1f"


def digest_file(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def run(command, **kwargs):
    return subprocess.run([str(arg) for arg in command], check=True, **kwargs)


def tool_path(name: str, explicit: str | None = None) -> str:
    path = explicit or shutil.which(name)
    if not path:
        raise ValueError(f"Missing required tool: {name}")
    return str(Path(path).resolve()) if Path(path).exists() else path


def rustc_profdata_tool(explicit: str | None = None) -> str:
    if explicit:
        return tool_path("llvm-profdata", explicit)
    try:
        version = subprocess.check_output(["rustc", "-vV"], cwd=ROOT, text=True)
        host = next(line.split(": ", 1)[1] for line in version.splitlines() if line.startswith("host: "))
        sysroot = Path(subprocess.check_output(["rustc", "--print", "sysroot"], cwd=ROOT, text=True).strip())
        bundled = sysroot / "lib/rustlib" / host / "bin/llvm-profdata"
        if bundled.is_file():
            return str(bundled)
    except (OSError, subprocess.CalledProcessError, StopIteration):
        pass
    return tool_path("llvm-profdata")


def tool_version(path: str) -> str | None:
    result = subprocess.run([path, "--version"], text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, check=False)
    return result.stdout.strip().splitlines()[0] if result.returncode == 0 and result.stdout.strip() else None

def validate_profile(path: Path, profdata: str):
    if not path.is_file() or path.stat().st_size == 0:
        raise ValueError(f"Profile is missing or empty: {path}")
    result = run([profdata, "merge", "--sample", "--text", "--output=-", str(path)],
                 stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if not re.search(r"^\s+\d+(?:\.\d+)?: [1-9]\d*(?:\s|$)", result.stdout, re.MULTILINE):
        raise ValueError(f"Profile contains no positive source-location sample counts: {path}")


def reject_conflicting_flags(flags: list[str]):
    forbidden = ("instrument-coverage", "profile-generate", "profile-use",
                 "profile-sample-use", "profile-instr-generate", "profile-instr-use")
    options = []
    for index, token in enumerate(flags):
        if token == "-C" and index + 1 < len(flags):
            options.append(flags[index + 1].split("=", 1)[0])
        elif token.startswith("-C"):
            options.append(token[2:].split("=", 1)[0])
    found = [flag for flag in forbidden if flag in options]
    if found:
        raise ValueError("Conflicting Rust profiling flags: " + ", ".join(found))


def augment_flags(env: dict[str, str], additions: list[str]):
    encoded = env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    raw = env.pop("RUSTFLAGS", "")
    flags = encoded.split(SEPARATOR) if encoded is not None else raw.split()
    reject_conflicting_flags(flags)
    flags.extend(additions)
    env["CARGO_ENCODED_RUSTFLAGS"] = SEPARATOR.join(flags)


def prepare(command: list[str]) -> int:
    if not command:
        raise ValueError("prepare requires a build command after --")
    env = os.environ.copy()
    augment_flags(env, ["-C", "debuginfo=limited", "-C", "strip=none"])
    return subprocess.run(command, env=env).returncode


def record(binary: Path, output: Path, workload: list[str], converter: str,
           converter_path: str | None, no_lbr: bool, frequency: int):
    binary = binary.resolve()
    if not binary.is_file():
        raise ValueError(f"Binary does not exist: {binary}")
    if frequency <= 0:
        raise ValueError("frequency must be positive")
    if no_lbr and converter != "autofdo":
        raise ValueError("--no-lbr requires --converter autofdo")
    perf = tool_path("perf")
    converter_name = "llvm-profgen" if converter == "llvm" else "create_llvm_prof"
    converter_tool = tool_path(converter_name, converter_path)
    profdata = rustc_profdata_tool()
    output = output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    raw_perf = output.with_suffix(output.suffix + ".perf.data")
    before = digest_file(binary)
    perf_command = [perf, "record", "-o", str(raw_perf), "-e", "cycles:u", "-F", str(frequency)]
    if not no_lbr:
        perf_command.extend(["-j", "any,u"])
    perf_command.extend(["--", str(binary), *workload])
    run(perf_command)
    after = digest_file(binary)
    if before != after:
        raise ValueError("Binary changed during profile capture")
    with tempfile.TemporaryDirectory(prefix=f".{output.name}.", dir=output.parent) as temporary:
        converted = Path(temporary) / "profile"
        if converter == "llvm":
            run([converter_tool, "--binary", str(binary), "--perfdata", str(raw_perf),
                 "--format=text", "--output", str(converted)])
        else:
            run([converter_tool, "--profile", str(raw_perf), "--binary", str(binary),
                 "--out", str(converted), "--format=text",
                 "--use_lbr=" + ("true" if not no_lbr else "false")])
        validate_profile(converted, profdata)
        if digest_file(binary) != before:
            raise ValueError("Binary changed during profile conversion")
        converted.replace(output)
    metadata = {
        "created": datetime.now(timezone.utc).isoformat(),
        "binary": str(binary),
        "binary_sha256": after,
        "profile_sha256": digest_file(output),
        "converter": converter,
        "capture": {"event": "cycles:u", "frequency": frequency,
                    "method": "perf-lbr" if not no_lbr else "perf-no-lbr"},
        "tools": {"perf": {"path": perf, "version": tool_version(perf)},
                  converter_name: {"path": converter_tool, "version": tool_version(converter_tool)},
                  "profile-validation": {"path": profdata, "version": tool_version(profdata)}},
        "flags": perf_command[1:perf_command.index("--")],
    }
    output.with_suffix(output.suffix + ".json").write_text(json.dumps(metadata, indent=2) + "\n")


def use(profile: Path, command: list[str], cache_dir: Path | None = None) -> int:
    if not command:
        raise ValueError("use requires a build command after --")
    profile = profile.resolve()
    profdata = rustc_profdata_tool()
    validate_profile(profile, profdata)
    digest = digest_file(profile)
    cache = (cache_dir or ROOT / ".trn" / "sample-profiles").resolve()
    cache.mkdir(parents=True, exist_ok=True)
    snapshot = cache / f"{digest}.prof"
    if snapshot.exists() and digest_file(snapshot) != digest:
        raise ValueError(f"Corrupt immutable profile snapshot: {snapshot}")
    if not snapshot.exists():
        temporary = cache / f".{digest}.{os.getpid()}.{id(profile)}.tmp"
        try:
            shutil.copyfile(profile, temporary)
            if digest_file(temporary) != digest:
                raise ValueError("Profile changed while creating immutable snapshot")
            try:
                os.link(temporary, snapshot)
            except FileExistsError:
                if digest_file(snapshot) != digest:
                    raise ValueError(f"Corrupt immutable profile snapshot: {snapshot}")
        finally:
            temporary.unlink(missing_ok=True)
    validate_profile(snapshot, profdata)
    env = os.environ.copy()
    augment_flags(env, ["-C", "debuginfo=limited", "-C", "strip=none",
                       "-C", f"profile-sample-use={snapshot}"])
    return subprocess.run(command, env=env).returncode


def parser():
    root = argparse.ArgumentParser(description=__doc__)
    commands = root.add_subparsers(dest="action", required=True)
    prepare_parser = commands.add_parser("prepare", help="build with stable function and line debug info")
    prepare_parser.add_argument("command", nargs=argparse.REMAINDER)
    record_parser = commands.add_parser("record", help="record cycles:u samples and convert them")
    record_parser.add_argument("--binary", required=True, type=Path)
    record_parser.add_argument("--output", required=True, type=Path)
    record_parser.add_argument("--converter", choices=("llvm", "autofdo"), default="llvm")
    record_parser.add_argument("--converter-path")
    record_parser.add_argument("--no-lbr", action="store_true")
    record_parser.add_argument("--frequency", type=int, default=999)
    record_parser.add_argument("command", nargs=argparse.REMAINDER)
    use_parser = commands.add_parser("use", help="build with an immutable sample-profile snapshot")
    use_parser.add_argument("--profile", required=True, type=Path)
    use_parser.add_argument("--cache-dir", type=Path)
    use_parser.add_argument("command", nargs=argparse.REMAINDER)
    return root


def cli(argv=None) -> int:
    args = parser().parse_args(argv)
    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    try:
        if args.action == "prepare":
            return prepare(command)
        if args.action == "record":
            record(args.binary, args.output, command, args.converter, args.converter_path,
                   args.no_lbr, args.frequency)
            return 0
        return use(args.profile, command, args.cache_dir)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"sample-profile: {error}", file=sys.stderr)
        return error.returncode if isinstance(error, subprocess.CalledProcessError) else 1


if __name__ == "__main__":
    raise SystemExit(cli())
