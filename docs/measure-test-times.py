#!/usr/bin/env python3
"""Run stable Rust libtest serially and update the test timing scoreboard."""

from __future__ import annotations

import argparse
import datetime as dt
import platform
import os
import re
import subprocess
import sys
import time
import tempfile
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError as error:
    raise SystemExit("PyYAML is required: install it with `python -m pip install pyyaml`") from error

RUNNING_RE = re.compile(
    r"^\s*Running (.+?) \((?:.*[/\\])?([^/\\]+?)-[0-9a-f]+\)\s*$"
)
DOCTEST_RE = re.compile(r"^\s*Doc-tests (\S+)\s*$")
COUNT_RE = re.compile(r"^running \d+ tests?$")
RESULT_RE = re.compile(r"^test (.+) \.\.\. (ok|FAILED|ignored)$")
TIMING_RECORD_PREFIX = "terrane-test-timing-v1"
HISTORY_LIMIT = 8
RUN_LIMIT = 12


def parse_args() -> argparse.Namespace:
    script_dir = Path(__file__).resolve().parent
    parser = argparse.ArgumentParser(
        description="Measure individual Rust tests with stable libtest and update the scoreboard."
    )
    parser.add_argument("--output", type=Path, default=script_dir / "test-scoreboard.yaml")
    parser.add_argument(
        "--no-html", action="store_true", help="update YAML without regenerating test-scoreboard.html"
    )
    parser.add_argument(
        "cargo_args",
        nargs=argparse.REMAINDER,
        help="Cargo test arguments after --; defaults to --workspace",
    )
    return parser.parse_args()


def base_scoreboard() -> dict[str, Any]:
    return {
        "format": 1,
        "title": "Terrane test timing scoreboard",
        "metadata": {
            "description": (
                "Serial wall-clock measurements from stable Rust libtest plus compiler-owned "
                "nested case timings. Shared conformance build time is amortized across its cases; "
                "durations include small runner and output overhead and are intended for relative "
                "development feedback."
            ),
            "timing_mode": (
                "cargo test with --test-threads=1; libtest completion deltas plus nested timing records"
            ),
            "history_limit": HISTORY_LIMIT,
        },
        "runs": [],
        "tests": [],
    }


def load_scoreboard(path: Path) -> dict[str, Any]:
    if not path.exists():
        return base_scoreboard()
    loaded = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(loaded, dict) or loaded.get("format") != 1:
        raise ValueError(f"unsupported test scoreboard format in {path}")
    return loaded


def normalized_target(description: str, executable: str | None = None) -> str:
    if executable:
        return f"{description} [{executable.replace('-', '_')}]"
    return description


def nested_timings(path: Path) -> list[dict[str, Any]]:
    measured: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        fields = line.split("\t")
        if len(fields) != 5 or fields[0] != TIMING_RECORD_PREFIX:
            continue
        _, kind, name, status, seconds = fields
        if kind != "conformance" or status not in {"passed", "failed", "ignored"}:
            continue
        measured.append(
            {
                "id": f"conformance cases [terrane_compiler]::{name}",
                "target": "conformance cases [terrane_compiler]",
                "name": name,
                "kind": "nested",
                "status": status,
                "seconds": round(float(seconds), 6),
            }
        )
    return measured


def run_tests(cargo_args: list[str]) -> tuple[int, float, list[dict[str, Any]], list[str]]:
    command = ["cargo", "test", *(cargo_args or ["--workspace"]), "--", "--test-threads=1"]
    timing_file = tempfile.NamedTemporaryFile(prefix="terrane-test-timings-", delete=False)
    timing_path = Path(timing_file.name)
    timing_file.close()
    environment = os.environ.copy()
    environment["TERRANE_TEST_TIMING_FILE"] = str(timing_path)
    started = time.monotonic()
    measured: list[dict[str, Any]] = []
    try:
        process = subprocess.Popen(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
            env=environment,
        )
        assert process.stdout is not None
        target = "unknown test binary"
        previous_event = started
        for line in process.stdout:
            print(line, end="", flush=True)
            text = line.rstrip("\n")
            running = RUNNING_RE.match(text)
            if running:
                target = normalized_target(running.group(1), running.group(2))
                previous_event = time.monotonic()
                continue
            doctest = DOCTEST_RE.match(text)
            if doctest:
                target = f"doc tests [{doctest.group(1)}]"
                previous_event = time.monotonic()
                continue
            if COUNT_RE.match(text):
                previous_event = time.monotonic()
                continue
            result = RESULT_RE.match(text)
            if result:
                now = time.monotonic()
                status = {
                    "ok": "passed",
                    "FAILED": "failed",
                    "ignored": "ignored",
                }[result.group(2)]
                name = result.group(1)
                measured.append(
                    {
                        "id": f"{target}::{name}",
                        "target": target,
                        "name": name,
                        "kind": "libtest",
                        "status": status,
                        "seconds": round(max(0.0, now - previous_event), 6),
                    }
                )
                previous_event = now
        exit_code = process.wait()
        measured.extend(nested_timings(timing_path))
    finally:
        timing_path.unlink(missing_ok=True)
    if any(test.get("kind") == "nested" for test in measured):
        measured = [
            test
            for test in measured
            if test["name"] != "every_manifest_drives_a_conformance_case"
        ]
    return exit_code, time.monotonic() - started, measured, command


def command_text(command: list[str]) -> str:
    completed = subprocess.run(command, text=True, capture_output=True)
    return completed.stdout.strip() if completed.returncode == 0 else "unavailable"


def update_scoreboard(
    data: dict[str, Any], measured: list[dict[str, Any]], command: list[str], elapsed: float, exit_code: int
) -> dict[str, Any]:
    data["metadata"] = base_scoreboard()["metadata"]
    measured_at = dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    replacing_aggregate = any(result.get("kind") == "nested" for result in measured)
    prior = {
        test["id"]: test
        for test in data.get("tests", [])
        if isinstance(test, dict)
        and not (
            replacing_aggregate
            and test.get("name") == "every_manifest_drives_a_conformance_case"
        )
    }
    for result in measured:
        old = prior.get(result["id"], {})
        history = [
            *old.get("history", []),
            {"measured_at": measured_at, "seconds": result["seconds"], "status": result["status"]},
        ][-HISTORY_LIMIT:]
        result["previous_seconds"] = old.get("seconds")
        result["measured_at"] = measured_at
        result["history"] = history
        prior[result["id"]] = result
    tests = sorted(prior.values(), key=lambda test: test["id"])
    summary = {
        "measured_at": measured_at,
        "command": command,
        "environment": {
            "host": platform.node(),
            "platform": platform.platform(),
            "rustc": command_text(["rustc", "-V"]),
            "revision": command_text(["git", "rev-parse", "--short", "HEAD"]),
        },
        "exit_code": exit_code,
        "cargo_seconds": round(elapsed, 6),
        "test_seconds": round(sum(test["seconds"] for test in measured), 6),
        "measured_tests": len(measured),
        "passed": sum(test["status"] == "passed" for test in measured),
        "failed": sum(test["status"] == "failed" for test in measured),
        "ignored": sum(test["status"] == "ignored" for test in measured),
    }
    data["runs"] = [*data.get("runs", []), summary][-RUN_LIMIT:]
    data["tests"] = tests
    return data


def main() -> int:
    args = parse_args()
    cargo_args = args.cargo_args
    if cargo_args[:1] == ["--"]:
        cargo_args = cargo_args[1:]
    if "--" in cargo_args:
        raise SystemExit("pass Cargo options only; the collector supplies libtest's serial options")
    try:
        data = load_scoreboard(args.output)
        exit_code, elapsed, measured, command = run_tests(cargo_args)
        data = update_scoreboard(data, measured, command, elapsed, exit_code)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(
            yaml.safe_dump(data, sort_keys=False, allow_unicode=True, width=100), encoding="utf-8"
        )
        print(f"updated {args.output} with {len(measured)} test timings")
        if not args.no_html:
            generator = Path(__file__).with_name("generate-test-scoreboard.py")
            generated = subprocess.run([sys.executable, str(generator), "--input", str(args.output)])
            if generated.returncode != 0:
                return generated.returncode
        return exit_code
    except (OSError, ValueError, yaml.YAMLError) as error:
        print(f"test timing collection failed: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
