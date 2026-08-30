#!/usr/bin/env python3
"""Language-neutral correctness and performance runner for the scientific corpus."""

from __future__ import annotations

import argparse
import json
import math
import os
import platform
import shutil
import subprocess
import sys
import threading
import time
import tomllib
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SUITE = Path(__file__).resolve().parent
REPO = SUITE.parents[1]


class BenchmarkError(RuntimeError):
    pass


@dataclass(frozen=True)
class ProcessResult:
    returncode: int
    stdout: str
    stderr: str
    wall_seconds: float
    peak_rss_bytes: int | None
    memory_trace: list[dict[str, int]] | None


@dataclass(frozen=True)
class Lane:
    lane_id: str
    name: str
    config_path: Path
    implementation: str
    setup: tuple[str, ...]
    prepare: tuple[str, ...]
    run: tuple[str, ...]
    prepare_output: str
    cache_paths: tuple[str, ...]
    metadata: dict[str, Any]


class MemorySampler:
    """Best-effort Linux RSS sampling for a process and all of its descendants."""

    def __init__(self, pid: int, retain_trace: bool) -> None:
        self.pid = pid
        self.retain_trace = retain_trace
        self.peak_kib = 0
        self.trace: list[dict[str, int]] = []
        self.started_ns = time.monotonic_ns()
        self.stopped = threading.Event()
        self.thread = threading.Thread(target=self._sample, daemon=True)

    def start(self) -> None:
        self.thread.start()

    def finish(self) -> tuple[int | None, list[dict[str, int]] | None]:
        self.stopped.set()
        self.thread.join()
        if not sys.platform.startswith("linux"):
            return None, None
        trace = self.trace if self.retain_trace else None
        peak = self.peak_kib * 1024 if self.peak_kib else None
        return peak, trace

    def _sample(self) -> None:
        if not sys.platform.startswith("linux"):
            return
        while not self.stopped.is_set():
            rss_kib = process_tree_rss_kib(self.pid)
            self.peak_kib = max(self.peak_kib, rss_kib)
            if self.retain_trace and rss_kib:
                elapsed_ms = (time.monotonic_ns() - self.started_ns) // 1_000_000
                self.trace.append({"elapsed_ms": elapsed_ms, "rss_bytes": rss_kib * 1024})
            self.stopped.wait(0.005)
        rss_kib = process_tree_rss_kib(self.pid)
        self.peak_kib = max(self.peak_kib, rss_kib)


def process_tree_rss_kib(root_pid: int) -> int:
    proc = Path("/proc")
    parents: dict[int, int] = {}
    for entry in proc.iterdir():
        if not entry.name.isdigit():
            continue
        try:
            stat = (entry / "stat").read_text()
            close = stat.rfind(")")
            fields = stat[close + 2 :].split()
            parents[int(entry.name)] = int(fields[1])
        except (OSError, ValueError, IndexError):
            continue

    descendants = {root_pid}
    changed = True
    while changed:
        changed = False
        for pid, parent in parents.items():
            if parent in descendants and pid not in descendants:
                descendants.add(pid)
                changed = True

    total = 0
    for pid in descendants:
        try:
            for line in (proc / str(pid) / "status").read_text().splitlines():
                if line.startswith("VmRSS:"):
                    total += int(line.split()[1])
                    break
        except (OSError, ValueError, IndexError):
            continue
    return total


def run_process(
    command: list[str],
    *,
    cwd: Path,
    timeout: float,
    retain_memory_trace: bool,
) -> ProcessResult:
    started = time.perf_counter()
    process = subprocess.Popen(
        command,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    sampler = MemorySampler(process.pid, retain_memory_trace)
    sampler.start()
    try:
        stdout, stderr = process.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        process.kill()
        stdout, stderr = process.communicate()
        sampler.finish()
        raise BenchmarkError(f"command timed out after {timeout:g}s: {format_command(command)}")
    peak_rss_bytes, memory_trace = sampler.finish()
    return ProcessResult(
        returncode=process.returncode,
        stdout=stdout,
        stderr=stderr,
        wall_seconds=time.perf_counter() - started,
        peak_rss_bytes=peak_rss_bytes,
        memory_trace=memory_trace,
    )


def load_toml(path: Path) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            return tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise BenchmarkError(f"cannot load {path.relative_to(SUITE)}: {error}") from error


def load_suite() -> tuple[dict[str, Any], list[dict[str, Any]], list[Lane]]:
    suite = load_toml(SUITE / "suite.toml")
    if suite.get("format") != 1:
        raise BenchmarkError("suite.toml must declare format = 1")

    problems: list[dict[str, Any]] = []
    seen_problems: set[str] = set()
    for item in suite.get("problems", []):
        problem_path = (SUITE / item["path"]).resolve()
        problem = load_toml(problem_path / "problem.toml")
        problem["path"] = problem_path
        problem_id = problem.get("id")
        if not isinstance(problem_id, str) or problem_id in seen_problems:
            raise BenchmarkError(f"invalid or duplicate problem id in {problem_path}")
        seen_problems.add(problem_id)
        problems.append(problem)

    lanes: list[Lane] = []
    seen_lanes: set[str] = set()
    for item in suite.get("lanes", []):
        config_path = (SUITE / item["path"]).resolve()
        config = load_toml(config_path)
        lane_id = config.get("id")
        if not isinstance(lane_id, str) or lane_id in seen_lanes:
            raise BenchmarkError(f"invalid or duplicate lane id in {config_path}")
        seen_lanes.add(lane_id)
        lanes.append(
            Lane(
                lane_id=lane_id,
                name=config["name"],
                config_path=config_path,
                implementation=config["implementation"],
                setup=tuple(config.get("setup", [])),
                prepare=tuple(config.get("prepare", [])),
                run=tuple(config["run"]),
                prepare_output=config.get("prepare-output", "none"),
                cache_paths=tuple(config.get("cache-paths", [])),
                metadata=dict(config.get("metadata", {})),
            )
        )
    return suite, problems, lanes


def command_context(problem: dict[str, Any], lane: Lane, prepared: str = "") -> dict[str, str]:
    problem_path: Path = problem["path"]
    implementation = (problem_path / lane.implementation).resolve()
    return {
        "repo": str(REPO),
        "suite": str(SUITE),
        "problem": str(problem_path),
        "implementation": str(implementation),
        "implementation_dir": str(implementation.parent),
        "prepared": prepared,
    }


def expand(command: tuple[str, ...], context: dict[str, str]) -> list[str]:
    try:
        return [part.format_map(context) for part in command]
    except KeyError as error:
        raise BenchmarkError(f"unknown command placeholder {error.args[0]!r}") from error


def format_command(command: list[str]) -> str:
    return " ".join(json.dumps(part) if any(character.isspace() for character in part) else part for part in command)


def require_success(result: ProcessResult, command: list[str]) -> None:
    if result.returncode == 0:
        return
    detail = result.stderr.strip() or result.stdout.strip() or "no diagnostic output"
    raise BenchmarkError(f"command failed ({result.returncode}): {format_command(command)}\n{detail}")


def setup_lane(lane: Lane, timeout: float, retain_trace: bool) -> ProcessResult | None:
    if not lane.setup:
        return None
    command = expand(lane.setup, command_context({"path": SUITE}, lane))
    result = run_process(command, cwd=REPO, timeout=timeout, retain_memory_trace=retain_trace)
    require_success(result, command)
    return result


def clear_lane_cache(problem: dict[str, Any], lane: Lane) -> None:
    context = command_context(problem, lane)
    for cache_path in lane.cache_paths:
        resolved = Path(cache_path.format_map(context)).resolve()
        if not resolved.is_relative_to(SUITE):
            raise BenchmarkError(f"refusing to remove cache outside suite: {resolved}")
        if resolved.is_dir():
            shutil.rmtree(resolved)
        elif resolved.exists():
            resolved.unlink()


def prepare_implementation(
    problem: dict[str, Any],
    lane: Lane,
    timeout: float,
    retain_trace: bool,
) -> tuple[str, ProcessResult | None]:
    context = command_context(problem, lane)
    implementation = Path(context["implementation"])
    if not implementation.exists():
        raise BenchmarkError(f"missing {lane.lane_id} implementation for {problem['id']}: {implementation}")
    if not lane.prepare:
        return "", None
    command = expand(lane.prepare, context)
    result = run_process(command, cwd=REPO, timeout=timeout, retain_memory_trace=retain_trace)
    require_success(result, command)
    if lane.prepare_output == "executable-path":
        lines = [line.strip() for line in result.stdout.splitlines() if line.strip()]
        if len(lines) != 1 or not Path(lines[0]).is_file():
            raise BenchmarkError(f"prepare command did not emit one executable path: {format_command(command)}")
        return lines[0], result
    if lane.prepare_output != "none":
        raise BenchmarkError(f"unsupported prepare-output value {lane.prepare_output!r}")
    return "", result


def parse_result(output: str, result_kind: str) -> int | float:
    lines = [line.strip() for line in output.splitlines() if line.strip()]
    if len(lines) != 1:
        raise BenchmarkError(f"implementation must print exactly one non-empty result line, got {lines!r}")
    try:
        if result_kind == "integer":
            return int(lines[0])
        if result_kind == "float":
            value = float(lines[0])
            if not math.isfinite(value):
                raise ValueError("non-finite result")
            return value
    except ValueError as error:
        raise BenchmarkError(f"invalid {result_kind} result {lines[0]!r}") from error
    raise BenchmarkError(f"unsupported result kind {result_kind!r}")


def expected_result(problem: dict[str, Any], profile_name: str) -> tuple[int | float, float, float]:
    profile = problem["profiles"][profile_name]
    return profile["expected"], float(profile.get("absolute-tolerance", 0.0)), float(
        profile.get("relative-tolerance", 0.0)
    )


def result_matches(actual: int | float, expected: int | float, absolute: float, relative: float) -> bool:
    if isinstance(actual, int) and isinstance(expected, int):
        return actual == expected
    return math.isclose(float(actual), float(expected), abs_tol=absolute, rel_tol=relative)


def execute(
    problem: dict[str, Any],
    lane: Lane,
    prepared: str,
    profile_name: str,
    timeout: float,
    retain_trace: bool,
) -> tuple[int | float, ProcessResult]:
    profile = problem["profiles"][profile_name]
    context = command_context(problem, lane, prepared)
    command = expand(lane.run, context) + list(profile.get("arguments", []))
    result = run_process(command, cwd=problem["path"], timeout=timeout, retain_memory_trace=retain_trace)
    require_success(result, command)
    actual = parse_result(result.stdout, problem["result"])
    expected, absolute, relative = expected_result(problem, profile_name)
    if not result_matches(actual, expected, absolute, relative):
        raise BenchmarkError(
            f"{problem['id']} / {lane.lane_id} / {profile_name}: expected {expected!r}, got {actual!r} "
            f"(abs={absolute:g}, rel={relative:g})"
        )
    return actual, result


def selected(values: list[Any], requested: list[str], attribute: str) -> list[Any]:
    if not requested:
        return values
    wanted = set(requested)
    available = {getattr(value, attribute) if hasattr(value, attribute) else value[attribute] for value in values}
    missing = wanted - available
    if missing:
        raise BenchmarkError(f"unknown selection: {', '.join(sorted(missing))}")
    return [
        value
        for value in values
        if (getattr(value, attribute) if hasattr(value, attribute) else value[attribute]) in wanted
    ]


def process_record(result: ProcessResult | None) -> dict[str, Any] | None:
    if result is None:
        return None
    record: dict[str, Any] = {
        "wall_seconds": result.wall_seconds,
        "peak_rss_bytes": result.peak_rss_bytes,
    }
    if result.memory_trace is not None:
        record["memory_trace"] = result.memory_trace
    return record


def check(problems: list[dict[str, Any]], lanes: list[Lane], timeout: float) -> None:
    for lane in lanes:
        setup_lane(lane, timeout, False)
    for problem in problems:
        for lane in lanes:
            prepared, _ = prepare_implementation(problem, lane, timeout, False)
            actual, _ = execute(problem, lane, prepared, "correctness", timeout, False)
            print(f"ok  {problem['id']:<24} {lane.lane_id:<10} {actual}")


def benchmark(
    suite: dict[str, Any],
    problems: list[dict[str, Any]],
    lanes: list[Lane],
    *,
    timeout: float,
    runs: int,
    warmups: int,
    retain_trace: bool,
) -> dict[str, Any]:
    report: dict[str, Any] = {
        "format": 1,
        "suite": suite["name"],
        "created_at": datetime.now(timezone.utc).isoformat(),
        "environment": {
            "platform": platform.platform(),
            "machine": platform.machine(),
            "processor": platform.processor(),
            "python": platform.python_version(),
            "logical_cpus": os.cpu_count(),
        },
        "measurement": {
            "clock": "time.perf_counter",
            "runs": runs,
            "warmups": warmups,
            "memory": (
                "sum of sampled Linux /proc VmRSS for the process and observed descendants"
                if sys.platform.startswith("linux")
                else "unavailable on this platform"
            ),
            "memory_limitations": (
                "The sampler waits 5 ms between scans, so its effective cadence also includes /proc scan time. Sampling can miss short-lived peaks; RSS includes runtime and allocator-retained pages and does not separate shared mappings."
                if sys.platform.startswith("linux")
                else None
            ),
        },
        "lane_setup": {},
        "results": [],
    }
    for lane in lanes:
        setup = setup_lane(lane, timeout, retain_trace)
        report["lane_setup"][lane.lane_id] = {
            "name": lane.name,
            "metadata": lane.metadata,
            "measurement": process_record(setup),
        }

    for problem in problems:
        for lane in lanes:
            clear_lane_cache(problem, lane)
            prepared, cold_prepare = prepare_implementation(problem, lane, timeout, retain_trace)
            prepared, incremental_prepare = prepare_implementation(problem, lane, timeout, retain_trace)
            correctness, _ = execute(problem, lane, prepared, "correctness", timeout, False)
            for _ in range(warmups):
                execute(problem, lane, prepared, "performance", timeout, False)
            measured_runs = []
            observed = None
            for _ in range(runs):
                observed, measurement = execute(
                    problem, lane, prepared, "performance", timeout, retain_trace
                )
                measured_runs.append(process_record(measurement))
            report["results"].append(
                {
                    "problem": problem["id"],
                    "title": problem["title"],
                    "lane": lane.lane_id,
                    "correctness_result": correctness,
                    "performance_result": observed,
                    "cold_prepare": process_record(cold_prepare),
                    "incremental_prepare": process_record(incremental_prepare),
                    "runs": measured_runs,
                }
            )
            print(f"benchmarked  {problem['id']:<24} {lane.lane_id}", file=sys.stderr)
    return report


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--problem", action="append", default=[], help="problem id (repeatable)")
    parser.add_argument("--lane", action="append", default=[], help="lane id (repeatable)")
    parser.add_argument("--timeout", type=float, default=300.0, help="per-command timeout in seconds")
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("list", help="list problems and lanes")
    commands.add_parser("check", help="build and verify correctness profiles")
    benchmark_parser = commands.add_parser("benchmark", help="verify and measure performance profiles")
    benchmark_parser.add_argument("--runs", type=int, default=None)
    benchmark_parser.add_argument("--warmups", type=int, default=None)
    benchmark_parser.add_argument("--memory-trace", action="store_true")
    benchmark_parser.add_argument("--output", type=Path)
    return parser


def main() -> int:
    parser = build_parser()
    arguments = parser.parse_args()
    try:
        suite, problems, lanes = load_suite()
        problems = selected(problems, arguments.problem, "id")
        lanes = selected(lanes, arguments.lane, "lane_id")
        if arguments.command == "list":
            print("Problems:")
            for problem in problems:
                print(f"  {problem['id']:<24} {problem['title']}")
            print("Lanes:")
            for lane in lanes:
                print(f"  {lane.lane_id:<24} {lane.name}")
            return 0
        if arguments.command == "check":
            check(problems, lanes, arguments.timeout)
            return 0
        measurement = suite["measurement"]
        runs = arguments.runs if arguments.runs is not None else int(measurement["runs"])
        warmups = arguments.warmups if arguments.warmups is not None else int(measurement["warmups"])
        if runs < 1 or warmups < 0:
            raise BenchmarkError("runs must be positive and warmups must be non-negative")
        report = benchmark(
            suite,
            problems,
            lanes,
            timeout=arguments.timeout,
            runs=runs,
            warmups=warmups,
            retain_trace=arguments.memory_trace,
        )
        rendered = json.dumps(report, indent=2) + "\n"
        if arguments.output:
            output = arguments.output.resolve()
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_text(rendered)
            print(output)
        else:
            print(rendered, end="")
        return 0
    except BenchmarkError as error:
        print(f"error: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
