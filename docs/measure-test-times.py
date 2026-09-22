#!/usr/bin/env python3
"""Run stable Rust libtest with bounded parallelism and update the timing scoreboard."""

from __future__ import annotations

import argparse
from collections.abc import Mapping
import datetime as dt
import os
import platform
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
MAX_SCORECARD_JOBS = 8


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
                "Bounded-parallel wall-clock measurements from stable Rust libtest plus "
                "compiler-owned nested case timings. Libtest durations are inferred from its "
                "deterministic alphabetical queue and completion events. Dependency-free "
                "generated-crate compilation has its own shared timing row instead of being "
                "estimated per case. A successful workspace run prunes rows not observed in that "
                "run; partial or failed runs preserve them. Durations include small runner and "
                "output overhead and are intended for relative development feedback."
            ),
            "timing_mode": (
                "cargo test with bounded --test-threads; scheduler-inferred active durations plus "
                "nested timing records"
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


def scorecard_jobs() -> int:
    available = os.cpu_count() or 1
    written = os.environ.get("TERRANE_SCORECARD_JOBS")
    if written is None:
        requested = MAX_SCORECARD_JOBS
    else:
        try:
            requested = int(written)
        except ValueError as error:
            raise ValueError(
                "TERRANE_SCORECARD_JOBS must be a positive integer"
            ) from error
        if requested < 1:
            raise ValueError("TERRANE_SCORECARD_JOBS must be a positive integer")
    return min(requested, MAX_SCORECARD_JOBS, available)


def inferred_parallel_timings(
    target: str,
    started: float,
    events: list[tuple[str, str, float]],
    jobs: int,
) -> list[dict[str, Any]]:
    queue = sorted(name for name, _, _ in events)
    active = {name: started for name in queue[:jobs]}
    next_index = len(active)
    measured = []
    for name, status, completed in events:
        test_started = active.pop(name, started)
        measured.append(
            {
                "id": f"{target}::{name}",
                "target": target,
                "name": name,
                "kind": "libtest",
                "status": status,
                "seconds": round(max(0.0, completed - test_started), 6),
            }
        )
        if next_index < len(queue):
            active[queue[next_index]] = completed
            next_index += 1
    return measured


def run_tests(cargo_args: list[str]) -> tuple[int, float, list[dict[str, Any]], list[str]]:
    jobs = scorecard_jobs()
    command = [
        "cargo",
        "test",
        *(cargo_args or ["--workspace"]),
        "--",
        f"--test-threads={jobs}",
    ]
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
        target_started = started
        target_events: list[tuple[str, str, float]] = []
        for line in process.stdout:
            print(line, end="", flush=True)
            text = line.rstrip("\n")
            running = RUNNING_RE.match(text)
            if running:
                measured.extend(
                    inferred_parallel_timings(target, target_started, target_events, jobs)
                )
                target_events = []
                target = normalized_target(running.group(1), running.group(2))
                target_started = time.monotonic()
                continue
            doctest = DOCTEST_RE.match(text)
            if doctest:
                measured.extend(
                    inferred_parallel_timings(target, target_started, target_events, jobs)
                )
                target_events = []
                target = f"doc tests [{doctest.group(1)}]"
                target_started = time.monotonic()
                continue
            if COUNT_RE.match(text):
                target_started = time.monotonic()
                continue
            result = RESULT_RE.match(text)
            if result:
                status = {
                    "ok": "passed",
                    "FAILED": "failed",
                    "ignored": "ignored",
                }[result.group(2)]
                target_events.append((result.group(1), status, time.monotonic()))
        measured.extend(inferred_parallel_timings(target, target_started, target_events, jobs))
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

def is_complete_workspace_run(
    command: list[str], exit_code: int, environment: Mapping[str, str]
) -> bool:
    if exit_code != 0 or environment.get("TERRANE_CONFORMANCE_FILTER", "").strip():
        return False
    try:
        cargo_end = command.index("--")
    except ValueError:
        cargo_end = len(command)
    cargo_args = command[2:cargo_end]
    if "--workspace" not in cargo_args:
        return False
    long_selectors = ("--exclude", "--package")
    return not any(
        argument == selector or argument.startswith(f"{selector}=")
        for argument in cargo_args
        for selector in long_selectors
    ) and not any(
        argument == "-p" or (argument.startswith("-p") and not argument.startswith("--"))
        for argument in cargo_args
    )


def update_scoreboard(
    data: dict[str, Any],
    measured: list[dict[str, Any]],
    command: list[str],
    elapsed: float,
    exit_code: int,
    environment: Mapping[str, str] | None = None,
) -> dict[str, Any]:
    metadata = base_scoreboard()["metadata"]
    previous_mode = data.get("metadata", {}).get("timing_mode")
    reset_history = previous_mode != metadata["timing_mode"]
    data["metadata"] = metadata
    measured_at = dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    replacing_aggregate = any(result.get("kind") == "nested" for result in measured)
    measured_ids = {result["id"] for result in measured}
    replace_all_tests = is_complete_workspace_run(command, exit_code, environment or os.environ)
    prior = {
        test["id"]: test
        for test in ([] if reset_history else data.get("tests", []))
        if isinstance(test, dict)
        and (not replace_all_tests or test.get("id") in measured_ids)
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
        raise SystemExit("pass Cargo options only; the collector supplies libtest's worker options")
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
