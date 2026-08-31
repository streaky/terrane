#!/usr/bin/env python3
"""Language-neutral correctness and performance runner for the scientific corpus."""

from __future__ import annotations

import argparse
import json
import math
import os
import platform
import select
import shutil
import signal
import subprocess
import sys
import tempfile
import time
from string import Template
from statistics import median
import tomllib
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SUITE = Path(__file__).resolve().parent
REPO = SUITE.parents[1]
PERFORMANCE_ENVIRONMENT_KEYS = (
    "CARGO_BUILD_RUSTFLAGS",
    "CARGO_ENCODED_RUSTFLAGS",
    "CARGO_PROFILE_RELEASE_CODEGEN_UNITS",
    "CARGO_PROFILE_RELEASE_LTO",
    "CARGO_PROFILE_RELEASE_OPT_LEVEL",
    "CFLAGS",
    "CPPFLAGS",
    "LD_PRELOAD",
    "PYTHONDONTWRITEBYTECODE",
    "PYTHONHASHSEED",
    "PYTHONOPTIMIZE",
    "RUSTFLAGS",
    "RUSTC_WRAPPER",
)


class BenchmarkError(RuntimeError):
    pass


@dataclass(frozen=True)
class ProcessResult:
    returncode: int
    stdout: str
    stderr: str
    wall_seconds: float
    peak_memory_bytes: int | None

@dataclass(frozen=True)
class Lane:
    lane_id: str
    name: str
    config_path: Path
    implementation: str
    setup: tuple[str, ...]
    prepare: tuple[str, ...]
    lower: tuple[str, ...]
    lower_output: str | None
    run: tuple[str, ...]
    prepare_output: str
    cache_paths: tuple[str, ...]
    metadata: dict[str, Any]
    environment_commands: tuple[tuple[str, ...], ...]


def wait_for_child(pid: int, timeout: float) -> int | None:
    """Wait for one child without polling when Linux pidfds are available."""
    if hasattr(os, "pidfd_open"):
        pidfd = os.pidfd_open(pid)
        try:
            readable, _, _ = select.select([pidfd], [], [], timeout)
        finally:
            os.close(pidfd)
        if not readable:
            return None
        _, status = os.waitpid(pid, 0)
        return status

    deadline = time.monotonic() + timeout
    while True:
        waited_pid, status = os.waitpid(pid, os.WNOHANG)
        if waited_pid:
            return status
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            return None
        time.sleep(min(0.001, remaining))


@dataclass(frozen=True)
class MemoryCgroup:
    path: Path

    def join_from_child(self) -> None:
        (self.path / "cgroup.procs").write_text(str(os.getpid()))

    def peak_bytes(self) -> int:
        return int((self.path / "memory.peak").read_text().strip())

    def populated(self) -> bool:
        return "populated 1" in (self.path / "cgroup.events").read_text().splitlines()

    def kill(self) -> None:
        kill_file = self.path / "cgroup.kill"
        if kill_file.exists():
            kill_file.write_text("1")

    def remove(self) -> None:
        deadline = time.monotonic() + 5.0
        while self.populated() and time.monotonic() < deadline:
            time.sleep(0.001)
        self.path.rmdir()


def memory_cgroup_parent() -> Path | None:
    if not sys.platform.startswith("linux"):
        return None
    try:
        entry = next(
            line
            for line in Path("/proc/self/cgroup").read_text().splitlines()
            if line.startswith("0::")
        )
    except (OSError, StopIteration):
        return None
    relative = Path(entry[3:].lstrip("/"))
    if ".." in relative.parts:
        return None
    parent = (Path("/sys/fs/cgroup") / relative).parent
    if not (parent / "cgroup.controllers").is_file():
        return None
    return parent


def create_memory_cgroup() -> MemoryCgroup | None:
    parent = memory_cgroup_parent()
    if parent is None:
        return None
    path: Path | None = None
    try:
        path = Path(tempfile.mkdtemp(prefix=f"terrane-sci-{os.getpid()}-", dir=parent))
        if not (path / "memory.peak").is_file():
            path.rmdir()
            return None
        return MemoryCgroup(path)
    except OSError:
        if path is not None:
            try:
                path.rmdir()
            except OSError:
                pass
        return None


def memory_measurement_available() -> bool:
    group = create_memory_cgroup()
    if group is None:
        return False
    group.remove()
    return True


def read_process_output(stdout_file: Any, stderr_file: Any) -> tuple[str, str]:
    stdout_file.seek(0)
    stderr_file.seek(0)
    return (
        stdout_file.read().decode("utf-8", errors="replace"),
        stderr_file.read().decode("utf-8", errors="replace"),
    )

def available_sccache(environment: dict[str, str]) -> str | None:
    sccache = shutil.which("sccache", path=environment.get("PATH"))
    return None if sccache is None else str(Path(sccache).resolve())


def process_environment() -> dict[str, str]:
    environment = os.environ.copy()
    if sccache := available_sccache(environment):
        environment["RUSTC_WRAPPER"] = sccache
    return environment


def prepare_sccache_server() -> None:
    environment = os.environ.copy()
    sccache = available_sccache(environment)
    if sccache is None:
        return
    environment["RUSTC_WRAPPER"] = sccache
    probe = subprocess.run(
        [sccache, "--show-stats"],
        cwd=REPO,
        env=environment,
        capture_output=True,
        text=True,
        check=False,
    )
    if probe.returncode == 0:
        return
    start = subprocess.run(
        [sccache, "--start-server"],
        cwd=REPO,
        env=environment,
        capture_output=True,
        text=True,
        check=False,
    )
    if start.returncode != 0:
        detail = (start.stderr or start.stdout).strip()
        raise BenchmarkError(f"cannot start available sccache: {detail}")



def run_process(command: list[str], *, cwd: Path, timeout: float) -> ProcessResult:
    memory_group = create_memory_cgroup()
    result: ProcessResult | None = None
    primary_error: BaseException | None = None
    try:
        with tempfile.TemporaryFile() as stdout_file, tempfile.TemporaryFile() as stderr_file:
            started = time.perf_counter()
            process = subprocess.Popen(
                command,
                cwd=cwd,
                stdout=stdout_file,
                stderr=stderr_file,
                start_new_session=True,
                preexec_fn=memory_group.join_from_child if memory_group else None,
                env=process_environment(),
            )
            waited = wait_for_child(process.pid, timeout)
            timed_out = waited is None
            if timed_out:
                if memory_group is not None:
                    memory_group.kill()
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                waited = wait_for_child(process.pid, 5.0)
                if waited is None:
                    raise BenchmarkError(
                        f"timed-out process group could not be reaped: {format_command(command)}"
                    )

            process.returncode = os.waitstatus_to_exitcode(waited)
            wall_seconds = time.perf_counter() - started
            stdout, stderr = read_process_output(stdout_file, stderr_file)
            peak_memory_bytes = memory_group.peak_bytes() if memory_group else None
            result = ProcessResult(
                returncode=process.returncode,
                stdout=stdout,
                stderr=stderr,
                wall_seconds=wall_seconds,
                peak_memory_bytes=peak_memory_bytes,
            )
            if memory_group is not None and memory_group.populated():
                memory_group.kill()
                raise BenchmarkError(
                    f"command left descendant processes running: {format_command(command)}"
                )
            if timed_out:
                raise BenchmarkError(f"command timed out after {timeout:g}s: {format_command(command)}")
    except BaseException as error:
        primary_error = error
        raise
    finally:
        if memory_group is not None:
            try:
                memory_group.remove()
            except OSError as error:
                cleanup_message = (
                    f"cannot remove execution memory cgroup {memory_group.path.name}: {error}"
                )
                if primary_error is not None:
                    primary_error.add_note(cleanup_message)
                elif result is not None and result.returncode != 0:
                    separator = "" if not result.stderr or result.stderr.endswith("\n") else "\n"
                    result = ProcessResult(
                        returncode=result.returncode,
                        stdout=result.stdout,
                        stderr=f"{result.stderr}{separator}{cleanup_message}\n",
                        wall_seconds=result.wall_seconds,
                        peak_memory_bytes=result.peak_memory_bytes,
                    )
                else:
                    raise BenchmarkError(cleanup_message) from error
    if result is None:
        raise BenchmarkError(f"command produced no process result: {format_command(command)}")
    return result


def load_toml(path: Path) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            return tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise BenchmarkError(f"cannot load {path.relative_to(SUITE)}: {error}") from error


def string_list(value: Any, description: str, *, required: bool = False) -> tuple[str, ...]:
    if (
        not isinstance(value, list)
        or not all(isinstance(item, str) and item for item in value)
        or (required and not value)
    ):
        qualifier = "a non-empty list of strings" if required else "a list of strings"
        raise BenchmarkError(f"{description} must be {qualifier}")
    return tuple(value)


def load_lane(config_path: Path) -> Lane:
    config = load_toml(config_path)
    lane_id = config.get("id")
    name = config.get("name")
    implementation = config.get("implementation")
    if not isinstance(lane_id, str) or not lane_id:
        raise BenchmarkError(f"{config_path}: id must be a non-empty string")
    if not isinstance(name, str) or not name:
        raise BenchmarkError(f"{config_path}: name must be a non-empty string")
    if not isinstance(implementation, str) or not implementation:
        raise BenchmarkError(f"{config_path}: implementation must be a non-empty string")
    prepare_output = config.get("prepare-output", "none")
    if prepare_output not in {"none", "executable-path"}:
        raise BenchmarkError(f"{config_path}: unsupported prepare-output value {prepare_output!r}")
    metadata = config.get("metadata", {})
    if not isinstance(metadata, dict):
        raise BenchmarkError(f"{config_path}: metadata must be a table")
    environment = config.get("environment", [])
    if not isinstance(environment, list):
        raise BenchmarkError(f"{config_path}: environment must be a list of command arrays")
    environment_commands = tuple(
        string_list(command, f"{config_path}: environment command", required=True)
        for command in environment
    )
    lower = string_list(config.get("lower", []), f"{config_path}: lower")
    lower_output = config.get("lower-output")
    if lower and (not isinstance(lower_output, str) or not lower_output):
        raise BenchmarkError(f"{config_path}: lower requires a non-empty lower-output")
    if lower_output is not None and not isinstance(lower_output, str):
        raise BenchmarkError(f"{config_path}: lower-output must be a string")
    return Lane(
        lane_id=lane_id,
        name=name,
        config_path=config_path,
        implementation=implementation,
        setup=string_list(config.get("setup", []), f"{config_path}: setup"),
        prepare=string_list(config.get("prepare", []), f"{config_path}: prepare"),
        lower=lower,
        lower_output=lower_output,
        run=string_list(config.get("run"), f"{config_path}: run", required=True),
        prepare_output=prepare_output,
        cache_paths=string_list(config.get("cache-paths", []), f"{config_path}: cache-paths"),
        metadata=dict(metadata),
        environment_commands=environment_commands,
    )

def validate_problem(problem: dict[str, Any], problem_path: Path) -> None:
    result_kind = problem.get("result")
    if result_kind not in {"integer", "float"}:
        raise BenchmarkError(f"{problem_path}: result must be 'integer' or 'float'")
    if not isinstance(problem.get("dataset"), str):
        raise BenchmarkError(f"{problem_path}: dataset must be a string")
    if not isinstance(problem.get("title"), str):
        raise BenchmarkError(f"{problem_path}: title must be a string")
    profiles = problem.get("profiles")
    if not isinstance(profiles, dict):
        raise BenchmarkError(f"{problem_path}: profiles must be a table")
    for profile_name in ("correctness", "performance"):
        profile = profiles.get(profile_name)
        if not isinstance(profile, dict):
            raise BenchmarkError(f"{problem_path}: missing {profile_name} profile")
        size = profile.get("size")
        if isinstance(size, bool) or not isinstance(size, int) or size <= 0:
            raise BenchmarkError(f"{problem_path}: {profile_name}.size must be a positive integer")
        expected = profile.get("expected")
        expected_type = int if result_kind == "integer" else float
        if isinstance(expected, bool) or not isinstance(expected, expected_type):
            raise BenchmarkError(
                f"{problem_path}: {profile_name}.expected must match {result_kind} result kind"
            )
        allowed = {"size", "expected", "absolute-tolerance", "relative-tolerance"}
        unknown = set(profile) - allowed
        if unknown:
            raise BenchmarkError(
                f"{problem_path}: {profile_name} has unknown fields: {', '.join(sorted(unknown))}"
            )
        for tolerance_name in ("absolute-tolerance", "relative-tolerance"):
            tolerance = profile.get(tolerance_name, 0.0)
            if (
                isinstance(tolerance, bool)
                or not isinstance(tolerance, (int, float))
                or not math.isfinite(float(tolerance))
                or tolerance < 0
            ):
                raise BenchmarkError(
                    f"{problem_path}: {profile_name}.{tolerance_name} must be finite and non-negative"
                )


def validate_suite(suite: dict[str, Any]) -> None:
    allowed_top_level = {"format", "name", "measurement", "lanes", "problems"}
    unknown = set(suite) - allowed_top_level
    if unknown:
        raise BenchmarkError(f"suite.toml has unknown fields: {', '.join(sorted(unknown))}")
    if suite.get("format") != 1:
        raise BenchmarkError("suite.toml must declare format = 1")
    if not isinstance(suite.get("name"), str) or not suite["name"]:
        raise BenchmarkError("suite.toml name must be a non-empty string")

    measurement = suite.get("measurement")
    if not isinstance(measurement, dict):
        raise BenchmarkError("suite.toml measurement must be a table")
    measurement_fields = {
        "warmups",
        "runs",
        "setup-timeout-seconds",
        "runtime-timeout-seconds",
    }
    unknown_measurement = set(measurement) - measurement_fields
    if unknown_measurement:
        raise BenchmarkError(
            "suite.toml measurement has unknown fields: "
            + ", ".join(sorted(unknown_measurement))
        )
    for field in ("warmups", "runs"):
        value = measurement.get(field)
        minimum = 0 if field == "warmups" else 1
        if isinstance(value, bool) or not isinstance(value, int) or value < minimum:
            qualifier = "non-negative" if minimum == 0 else "positive"
            raise BenchmarkError(f"suite.toml measurement.{field} must be a {qualifier} integer")
    for field in ("setup-timeout-seconds", "runtime-timeout-seconds"):
        value = measurement.get(field)
        if (
            isinstance(value, bool)
            or not isinstance(value, (int, float))
            or not math.isfinite(float(value))
            or value <= 0
        ):
            raise BenchmarkError(f"suite.toml measurement.{field} must be finite and positive")

    for collection in ("lanes", "problems"):
        entries = suite.get(collection)
        if not isinstance(entries, list) or not entries:
            raise BenchmarkError(f"suite.toml {collection} must be a non-empty array of tables")
        for index, entry in enumerate(entries):
            if (
                not isinstance(entry, dict)
                or set(entry) != {"path"}
                or not isinstance(entry.get("path"), str)
                or not entry["path"]
            ):
                raise BenchmarkError(
                    f"suite.toml {collection}[{index}] must contain exactly one non-empty path"
                )
            resolved = (SUITE / entry["path"]).resolve()
            if not resolved.is_relative_to(SUITE):
                raise BenchmarkError(f"suite.toml {collection}[{index}] path is outside the suite")


def load_suite() -> tuple[dict[str, Any], list[dict[str, Any]], list[Lane]]:
    suite = load_toml(SUITE / "suite.toml")
    validate_suite(suite)

    problems: list[dict[str, Any]] = []
    seen_problems: set[str] = set()
    for item in suite.get("problems", []):
        problem_path = (SUITE / item["path"]).resolve()
        problem = load_toml(problem_path / "problem.toml")
        problem["path"] = problem_path
        validate_problem(problem, problem_path)
        problem_id = problem.get("id")
        if not isinstance(problem_id, str) or problem_id in seen_problems:
            raise BenchmarkError(f"invalid or duplicate problem id in {problem_path}")
        seen_problems.add(problem_id)
        problems.append(problem)

    lanes: list[Lane] = []
    seen_lanes: set[str] = set()
    for item in suite.get("lanes", []):
        config_path = (SUITE / item["path"]).resolve()
        lane = load_lane(config_path)
        if lane.lane_id in seen_lanes:
            raise BenchmarkError(f"duplicate lane id {lane.lane_id!r} in {config_path}")
        seen_lanes.add(lane.lane_id)
        lanes.append(lane)
    return suite, problems, lanes


def command_context(
    problem: dict[str, Any] | None, lane: Lane, prepared: str = ""
) -> dict[str, str]:
    context = {
        "repo": str(REPO),
        "suite": str(SUITE),
        "prepared": prepared,
    }
    if problem is not None:
        problem_path: Path = problem["path"]
        implementation = (problem_path / lane.implementation).resolve()
        context.update(
            {
                "problem": str(problem_path),
                "implementation": str(implementation),
                "implementation_dir": str(implementation.parent),
            }
        )
    return context


def substitute(value: str, context: dict[str, str]) -> str:
    try:
        return Template(value).substitute(context)
    except KeyError as error:
        raise BenchmarkError(f"unknown command placeholder {error.args[0]!r}") from error
    except ValueError as error:
        raise BenchmarkError(f"invalid command template: {error}") from error


def expand(command: tuple[str, ...], context: dict[str, str]) -> list[str]:
    return [substitute(part, context) for part in command]


def format_command(command: list[str]) -> str:
    return " ".join(json.dumps(part) if any(character.isspace() for character in part) else part for part in command)


def require_success(result: ProcessResult, command: list[str]) -> None:
    if result.returncode == 0:
        return
    detail = result.stderr.strip() or result.stdout.strip() or "no diagnostic output"
    raise BenchmarkError(f"command failed ({result.returncode}): {format_command(command)}\n{detail}")


def setup_lane(lane: Lane, timeout: float) -> ProcessResult | None:
    if not lane.setup:
        return None
    command = expand(lane.setup, command_context(None, lane))
    result = run_process(command, cwd=REPO, timeout=timeout)
    require_success(result, command)
    return result


def lane_cache_paths(problem: dict[str, Any], lane: Lane) -> list[Path]:
    context = command_context(problem, lane)
    paths = [Path(substitute(cache_path, context)).resolve() for cache_path in lane.cache_paths]
    for path in paths:
        if not path.is_relative_to(SUITE):
            raise BenchmarkError(f"cache path is outside suite: {path}")
    return paths


def clear_lane_cache(problem: dict[str, Any], lane: Lane) -> None:
    for resolved in lane_cache_paths(problem, lane):
        if resolved.is_dir():
            shutil.rmtree(resolved)
        elif resolved.exists():
            resolved.unlink()


def lane_cache_exists(problem: dict[str, Any], lane: Lane) -> bool:
    paths = lane_cache_paths(problem, lane)
    return bool(paths) and all(path.exists() for path in paths)


def prepare_implementation(
    problem: dict[str, Any],
    lane: Lane,
    timeout: float,
) -> tuple[str, ProcessResult | None]:
    context = command_context(problem, lane)
    implementation = Path(context["implementation"])
    if not implementation.exists():
        raise BenchmarkError(f"missing {lane.lane_id} implementation for {problem['id']}: {implementation}")
    if not lane.prepare:
        return "", None
    command = expand(lane.prepare, context)
    result = run_process(command, cwd=REPO, timeout=timeout)
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


def result_matches(
    result_kind: str,
    actual: int | float,
    expected: int | float,
    absolute: float,
    relative: float,
) -> bool:
    if result_kind == "integer":
        return actual == expected
    if result_kind == "float":
        return math.isclose(float(actual), float(expected), abs_tol=absolute, rel_tol=relative)
    raise BenchmarkError(f"unsupported result kind {result_kind!r}")


def execute(
    problem: dict[str, Any],
    lane: Lane,
    prepared: str,
    profile_name: str,
    timeout: float,
) -> tuple[int | float, ProcessResult]:
    profile = problem["profiles"][profile_name]
    context = command_context(problem, lane, prepared)
    command = expand(lane.run, context) + [str(profile["size"])]
    result = run_process(command, cwd=problem["path"], timeout=timeout)
    require_success(result, command)
    actual = parse_result(result.stdout, problem["result"])
    expected, absolute, relative = expected_result(problem, profile_name)
    if not result_matches(problem["result"], actual, expected, absolute, relative):
        raise BenchmarkError(
            f"{problem['id']} / {lane.lane_id} / {profile_name}: expected {expected!r}, got {actual!r} "
            f"(abs={absolute:g}, rel={relative:g})"
        )
    return actual, result

def selected(
    values: list[Any], requested: list[str], identifiers: list[str]
) -> list[Any]:
    if not requested:
        return values
    wanted = set(requested)
    available = set(identifiers)
    missing = wanted - available
    if missing:
        raise BenchmarkError(f"unknown selection: {', '.join(sorted(missing))}")
    return [
        value
        for value, identifier in zip(values, identifiers, strict=True)
        if identifier in wanted
    ]


def process_record(result: ProcessResult | None) -> dict[str, Any] | None:
    if result is None:
        return None
    warning_lines = [
        line for line in result.stderr.splitlines() if "warning" in line.casefold()
    ]
    return {
        "wall_seconds": result.wall_seconds,
        "peak_memory_bytes": result.peak_memory_bytes,
        "stderr": result.stderr,
        "warning_lines": warning_lines,
    }

def materialize_lowering(
    problems: list[dict[str, Any]], lanes: list[Lane], timeout: float
) -> None:
    lowering_lanes = [lane for lane in lanes if lane.lower]
    for lane in lowering_lanes:
        setup_lane(lane, timeout)
    for problem in problems:
        for lane in lowering_lanes:
            if lane.lower_output is None:
                raise BenchmarkError(f"{lane.lane_id} declares lower without lower-output")
            context = command_context(problem, lane)
            command = expand(lane.lower, context)
            result = run_process(command, cwd=problem["path"], timeout=timeout)
            require_success(result, command)
            output = Path(substitute(lane.lower_output, context)).resolve()
            if not output.is_relative_to(problem["path"]):
                raise BenchmarkError(f"refusing to write lowering outside problem: {output}")
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_text(result.stdout)
            print(f"lowered  {problem['id']:<24} {lane.lane_id:<10} {output.relative_to(SUITE)}")



def check(
    problems: list[dict[str, Any]],
    lanes: list[Lane],
    setup_timeout: float,
    runtime_timeout: float,
) -> None:
    for lane in lanes:
        setup_lane(lane, setup_timeout)
    for problem in problems:
        for lane in lanes:
            prepared, _ = prepare_implementation(problem, lane, setup_timeout)
            actual, _ = execute(problem, lane, prepared, "correctness", runtime_timeout)
            print(f"ok  {problem['id']:<24} {lane.lane_id:<10} {actual}")


def machine_environment() -> dict[str, Any]:
    cpu_model = platform.processor()
    memory_bytes = None
    physical_cores = None
    governor = None
    if sys.platform.startswith("linux"):
        try:
            for line in Path("/proc/cpuinfo").read_text().splitlines():
                if line.startswith("model name"):
                    cpu_model = line.split(":", 1)[1].strip()
                    break
        except OSError:
            pass
        try:
            for line in Path("/proc/meminfo").read_text().splitlines():
                if line.startswith("MemTotal:"):
                    memory_bytes = int(line.split()[1]) * 1024
                    break
        except (OSError, ValueError, IndexError):
            pass
        try:
            core_ids = {
                (
                    (path / "physical_package_id").read_text().strip(),
                    (path / "core_id").read_text().strip(),
                )
                for path in Path("/sys/devices/system/cpu").glob("cpu[0-9]*/topology")
            }
            physical_cores = len(core_ids) or None
        except OSError:
            pass
        try:
            governor = Path(
                "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"
            ).read_text().strip() or None
        except OSError:
            pass
    try:
        load_average = list(os.getloadavg())
    except OSError:
        load_average = None
    inherited_environment = {
        key: os.environ[key]
        for key in PERFORMANCE_ENVIRONMENT_KEYS
        if key in os.environ
    }
    return {
        "platform": platform.platform(),
        "kernel": platform.release(),
        "machine": platform.machine(),
        "cpu_model": cpu_model,
        "logical_cpus": os.cpu_count(),
        "physical_cores": physical_cores,
        "memory_bytes": memory_bytes,
        "runner_python": platform.python_version(),
        "cpu_frequency_governor": governor,
        "load_average": load_average,
        "inherited_performance_environment": inherited_environment,
    }

def portable_command(command: list[str]) -> list[str]:
    repository = str(REPO)
    return [
        f"$repo{part[len(repository):]}" if part.startswith(repository + os.sep) else part
        for part in command
    ]


def capture_lane_environment(lane: Lane, timeout: float) -> list[dict[str, Any]]:
    captured = []
    context = command_context(None, lane)
    for template in lane.environment_commands:
        command = expand(template, context)
        result = run_process(command, cwd=REPO, timeout=timeout)
        require_success(result, command)
        captured.append(
            {
                "command": portable_command(command),
                "stdout": result.stdout,
                "stderr": result.stderr,
            }
        )
    return captured


def benchmark(
    suite: dict[str, Any],
    problems: list[dict[str, Any]],
    lanes: list[Lane],
    *,
    setup_timeout: float,
    runtime_timeout: float,
    runs: int,
    warmups: int,
    cold_builds: bool,
) -> dict[str, Any]:
    memory_available = memory_measurement_available()
    report: dict[str, Any] = {
        "format": 2,
        "suite": suite["name"],
        "created_at": datetime.now(timezone.utc).isoformat(),
        "environment": machine_environment(),
        "measurement": {
            "clock": "time.perf_counter",
            "runs": runs,
            "warmups": warmups,
            "cold_builds": cold_builds,
            "setup_timeout_seconds": setup_timeout,
            "runtime_timeout_seconds": runtime_timeout,
            "execution_timing": "process spawn to exit, as observed by the parent; setup and preparation complete before spawning",
            "run_order": "problem-major and lane-minor within each warm-up or measured run index",
            "build_cache": (
                "adapter-declared caches cleared before initial preparation"
                if cold_builds
                else "existing adapter-declared caches preserved"
            ),
            "memory": (
                "peak cgroup-v2 memory charged to a fresh cgroup containing the launched process and all descendants"
                if memory_available
                else "unavailable because delegated cgroup-v2 memory accounting is not accessible"
            ),
            "memory_limitations": (
                "memory.peak includes anonymous memory, charged page cache, and kernel memory. "
                "Shared pages are charged once, potentially to a cgroup outside the measured "
                "execution, so small-footprint results depend on page-cache state and are not "
                "directly comparable across machines or reboots. It is not an RSS measurement."
                if memory_available
                else None
            ),
        },
        "lane_setup": {},
        "results": [],
    }
    for lane in lanes:
        setup = setup_lane(lane, setup_timeout)
        report["lane_setup"][lane.lane_id] = {
            "name": lane.name,
            "metadata": lane.metadata,
            "environment": capture_lane_environment(lane, setup_timeout),
            "measurement": process_record(setup),
        }

    cases: list[dict[str, Any]] = []
    for problem in problems:
        for lane in lanes:
            cache_was_present = lane_cache_exists(problem, lane)
            cold_prepare = None
            incremental_prepare = None
            if cold_builds and lane.cache_paths:
                clear_lane_cache(problem, lane)
                prepared, cold_prepare = prepare_implementation(
                    problem, lane, setup_timeout
                )
                prepared, incremental_prepare = prepare_implementation(
                    problem, lane, setup_timeout
                )
            else:
                prepared, preparation = prepare_implementation(
                    problem, lane, setup_timeout
                )
                if cache_was_present:
                    incremental_prepare = preparation
                else:
                    cold_prepare = preparation
            correctness, _ = execute(
                problem, lane, prepared, "correctness", runtime_timeout
            )
            result_record = {
                "problem": problem["id"],
                "title": problem["title"],
                "dataset": problem["dataset"],
                "correctness_profile": dict(problem["profiles"]["correctness"]),
                "performance_profile": dict(problem["profiles"]["performance"]),
                "lane": lane.lane_id,
                "correctness_result": correctness,
                "performance_result": None,
                "cold_prepare": process_record(cold_prepare),
                "incremental_prepare": process_record(incremental_prepare),
                "runs": [],
            }
            report["results"].append(result_record)
            cases.append(
                {
                    "problem": problem,
                    "lane": lane,
                    "prepared": prepared,
                    "record": result_record,
                }
            )

    for _ in range(warmups):
        for case in cases:
            execute(
                case["problem"],
                case["lane"],
                case["prepared"],
                "performance",
                runtime_timeout,
            )

    for _ in range(runs):
        for case in cases:
            observed, measurement = execute(
                case["problem"],
                case["lane"],
                case["prepared"],
                "performance",
                runtime_timeout,
            )
            case["record"]["performance_result"] = observed
            case["record"]["runs"].append(process_record(measurement))

    for case in cases:
        print(
            f"benchmarked  {case['problem']['id']:<24} {case['lane'].lane_id}",
            file=sys.stderr,
        )
    return report


def format_seconds(value: float | None) -> str:
    if value is None:
        return "—"
    if value < 0.001:
        return f"{value * 1_000_000:.0f} µs"
    if value < 1:
        return f"{value * 1_000:.2f} ms"
    return f"{value:.3f} s"


def format_bytes(value: int | None) -> str:
    if value is None:
        return "—"
    amount = float(value)
    for unit in ("B", "KiB", "MiB", "GiB", "TiB"):
        if amount < 1024 or unit == "TiB":
            return f"{amount:.1f} {unit}"
        amount /= 1024
    raise AssertionError("unreachable")


def markdown_text(value: Any) -> str:
    return str(value).replace("|", "\\|").replace("\n", " ")


def render_markdown(report: dict[str, Any], raw_report_name: str) -> str:
    environment = report["environment"]
    measurement = report["measurement"]
    load_average = environment["load_average"]
    load_average_text = (
        " / ".join(f"{value:.2f}" for value in load_average)
        if load_average
        else "not reported"
    )
    inherited_environment = environment["inherited_performance_environment"]
    inherited_environment_text = (
        "; ".join(f"{key}={value}" for key, value in inherited_environment.items())
        or "none of the recorded variables set"
    )
    lines = [
        f"# {report['suite']}",
        "",
        f"Generated at `{report['created_at']}`.",
        "",
        "## Environment",
        "",
        "| Property | Value |",
        "|---|---|",
        f"| Platform | {markdown_text(environment['platform'])} |",
        f"| Kernel | {markdown_text(environment['kernel'])} |",
        f"| Machine | {markdown_text(environment['machine'])} |",
        f"| CPU model | {markdown_text(environment['cpu_model'] or 'not reported')} |",
        f"| Physical cores | {environment['physical_cores'] or 'not reported'} |",
        f"| Logical CPUs | {environment['logical_cpus'] or 'not reported'} |",
        f"| Memory | {format_bytes(environment['memory_bytes'])} |",
        f"| CPU frequency governor | {markdown_text(environment['cpu_frequency_governor'] or 'not reported')} |",
        f"| Load average at start (1 / 5 / 15 min) | {markdown_text(load_average_text)} |",
        f"| Inherited performance environment | {markdown_text(inherited_environment_text)} |",
        f"| Runner Python | {markdown_text(environment['runner_python'])} |",
        "",
        "## Measurement",
        "",
        f"- Warm-up executions per problem and lane: **{measurement['warmups']}**",
        f"- Measured executions per problem and lane: **{measurement['runs']}**",
        f"- Clock: `{measurement['clock']}`",
        f"- Build cache: {measurement['build_cache']}.",
        f"- Execution timing: {measurement['execution_timing']}.",
        f"- Run order: {measurement['run_order']}.",
        f"- Setup timeout: {format_seconds(measurement['setup_timeout_seconds'])}; runtime timeout: {format_seconds(measurement['runtime_timeout_seconds'])}.",
        f"- Memory: {measurement['memory']}.",
    ]
    if measurement["memory_limitations"]:
        lines.append(f"- Memory limitations: {measurement['memory_limitations']}")

    lines.extend(
        [
            "",
            "## Lanes",
            "",
            "| Lane | Implementation | Native build profile | Captured environment |",
            "|---|---|---|---|",
        ]
    )
    for lane_id, lane in report["lane_setup"].items():
        metadata = lane["metadata"]
        environment_evidence = "; ".join(
            (item["stdout"].strip() or item["stderr"].strip()).replace("\n", " / ")
            for item in lane["environment"]
        )
        lines.append(
            "| "
            + " | ".join(
                [
                    markdown_text(lane["name"]),
                    markdown_text(metadata.get("implementation", lane_id)),
                    markdown_text(metadata.get("native-build-profile", "not applicable")),
                    markdown_text(environment_evidence or "not declared"),
                ]
            )
            + " |"
        )

    lines.extend(
        [
            "",
            "## Execution results",
            "",
            "| Problem | Lane | Size | Result | Median wall time | Range | Median peak memory | Peak memory range | Warnings |",
            "|---|---|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for result in report["results"]:
        runs = result["runs"]
        times = [run["wall_seconds"] for run in runs]
        peaks = [run["peak_memory_bytes"] for run in runs if run["peak_memory_bytes"] is not None]
        time_range = f"{format_seconds(min(times))}–{format_seconds(max(times))}"
        lines.append(
            "| "
            + " | ".join(
                [
                    markdown_text(result["title"]),
                    markdown_text(result["lane"]),
                    str(result["performance_profile"]["size"]),
                    markdown_text(result["performance_result"]),
                    format_seconds(median(times)),
                    time_range,
                    format_bytes(int(median(peaks))) if peaks else "—",
                    f"{format_bytes(min(peaks))}–{format_bytes(max(peaks))}" if peaks else "—",
                    str(sum(len(run["warning_lines"]) for run in runs)),
                ]
            )
            + " |"
        )

    preparation_results = [
        result
        for result in report["results"]
        if result["cold_prepare"] is not None or result["incremental_prepare"] is not None
    ]
    if measurement["cold_builds"] and preparation_results:
        lines.extend(
            [
                "",
                "## Preparation results",
                "",
                "| Problem | Lane | Cold preparation | Cold peak memory | Incremental preparation | Incremental peak memory |",
                "|---|---|---:|---:|---:|---:|",
            ]
        )
        for result in preparation_results:
            cold = result["cold_prepare"]
            incremental = result["incremental_prepare"]
            lines.append(
                "| "
                + " | ".join(
                    [
                        markdown_text(result["title"]),
                        markdown_text(result["lane"]),
                        format_seconds(cold["wall_seconds"] if cold else None),
                        format_bytes(cold["peak_memory_bytes"] if cold else None),
                        format_seconds(incremental["wall_seconds"] if incremental else None),
                        format_bytes(incremental["peak_memory_bytes"] if incremental else None),
                    ]
                )
                + " |"
            )

    warning_count = 0
    for lane in report["lane_setup"].values():
        if lane["measurement"]:
            warning_count += len(lane["measurement"]["warning_lines"])
    for result in report["results"]:
        for key in ("cold_prepare", "incremental_prepare"):
            if result[key]:
                warning_count += len(result[key]["warning_lines"])
        warning_count += sum(len(run["warning_lines"]) for run in result["runs"])
    lines.extend(
        [
            "",
            f"Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **{warning_count} warning line(s)** were detected.",
            "",
            f"Complete measurements: [{raw_report_name}]({raw_report_name})",
            "",
        ]
    )
    return "\n".join(lines)


def add_measurement_arguments(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--runs", type=int, default=None)
    parser.add_argument("--warmups", type=int, default=None)
    parser.add_argument("--output", type=Path)
    parser.add_argument(
        "--cold-builds",
        action="store_true",
        help="clear adapter-declared caches and measure cold plus incremental preparation",
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--problem", action="append", default=[], help="problem id (repeatable)")
    parser.add_argument("--lane", action="append", default=[], help="lane id (repeatable)")
    parser.add_argument(
        "--setup-timeout",
        type=float,
        default=None,
        help="setup, preparation, and lowering timeout in seconds",
    )
    parser.add_argument(
        "--runtime-timeout",
        type=float,
        default=None,
        help="correctness, warm-up, and measured execution timeout in seconds",
    )
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("list", help="list problems and lanes")
    commands.add_parser("lower", help="refresh inspectable lowered sources declared by lane adapters")
    commands.add_parser("check", help="build and verify correctness profiles")
    benchmark_parser = commands.add_parser("benchmark", help="verify and emit raw JSON measurements")
    add_measurement_arguments(benchmark_parser)
    report_parser = commands.add_parser(
        "report", help="run the corpus and write Markdown plus complete JSON measurements"
    )
    add_measurement_arguments(report_parser)
    return parser


def main() -> int:
    parser = build_parser()
    arguments = parser.parse_args()
    try:
        suite, problems, lanes = load_suite()
        problems = selected(problems, arguments.problem, [problem["id"] for problem in problems])
        lanes = selected(lanes, arguments.lane, [lane.lane_id for lane in lanes])
        measurement = suite["measurement"]
        setup_timeout = (
            arguments.setup_timeout
            if arguments.setup_timeout is not None
            else float(measurement["setup-timeout-seconds"])
        )
        runtime_timeout = (
            arguments.runtime_timeout
            if arguments.runtime_timeout is not None
            else float(measurement["runtime-timeout-seconds"])
        )
        if setup_timeout <= 0 or runtime_timeout <= 0:
            raise BenchmarkError("setup and runtime timeouts must be positive")
        if arguments.command == "list":
            print("Problems:")
            for problem in problems:
                print(f"  {problem['id']:<24} {problem['title']}")
            print("Lanes:")
            for lane in lanes:
                print(f"  {lane.lane_id:<24} {lane.name}")
            return 0
        prepare_sccache_server()
        if arguments.command == "lower":
            materialize_lowering(problems, lanes, setup_timeout)
            return 0
        if arguments.command == "check":
            check(problems, lanes, setup_timeout, runtime_timeout)
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
            setup_timeout=setup_timeout,
            runtime_timeout=runtime_timeout,
            runs=runs,
            warmups=warmups,
            cold_builds=arguments.cold_builds,
        )
        rendered = json.dumps(report, indent=2) + "\n"
        if arguments.command == "report":
            timestamp = datetime.fromisoformat(report["created_at"]).strftime("%Y%m%dT%H%M%S")
            output = (
                arguments.output.resolve()
                if arguments.output
                else (SUITE / "reports" / f"report-{timestamp}.md")
            )
            if output.suffix.lower() != ".md":
                raise BenchmarkError("report output must use a .md extension")
            output.parent.mkdir(parents=True, exist_ok=True)
            raw_output = output.with_suffix(".json")
            raw_output.write_text(rendered)
            output.write_text(render_markdown(report, raw_output.name))
            print(output)
        elif arguments.output:
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
