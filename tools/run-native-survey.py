#!/usr/bin/env python3
"""Survey and exercise one named public native-integration profile.

Profiles that declare enforced containment require Linux bubblewrap (`bwrap`);
the runner never silently downgrades them to an uncontained process. A profile
whose dedicated runtime environment is unavailable still produces runnable
offline native-survey evidence and skips only its conformance/runtime fixture.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import shutil
import subprocess
import sys
import tomllib


ROOT = pathlib.Path(__file__).resolve().parents[1]
CORPUS = ROOT / "tests" / "conformance"
PROFILE_MANIFEST = ROOT / "tests" / "native-survey" / "corpus.json"
WORK_ROOT = ROOT / "target" / "native-survey"
SURVEY_BINARY = ROOT / "target" / "debug" / "terrane-rust-survey"


def load_profile(name: str) -> dict:
    try:
        manifest = json.loads(PROFILE_MANIFEST.read_text())
        if manifest.get("schema") != 1:
            raise ValueError("unsupported corpus schema")
        profiles = manifest["profiles"]
    except (OSError, ValueError, KeyError, json.JSONDecodeError) as error:
        raise argparse.ArgumentTypeError(f"invalid native survey corpus: {error}") from error
    matches = [item for item in profiles if item.get("name") == name]
    if len(matches) != 1:
        raise argparse.ArgumentTypeError(f"unknown or duplicate native survey profile: {name}")
    profile = matches[0]
    required = {
        "fixture", "packages", "target", "rustdoc-toolchain", "containment", "selection"
    }
    missing = sorted(required - profile.keys())
    if missing:
        raise argparse.ArgumentTypeError(
            f"native survey profile {name!r} lacks: {', '.join(missing)}"
        )
    if profile["rustdoc-toolchain"] != "nightly-2026-04-29":
        raise argparse.ArgumentTypeError(f"profile {name!r} uses an unexpected Rustdoc toolchain")
    if profile["containment"] not in {"enforced", "unavailable"}:
        raise argparse.ArgumentTypeError(
            f"profile {name!r} has an invalid containment tier"
        )
    profile["fixture"] = validate_fixture(profile["fixture"])
    return profile


def validate_fixture(value: str) -> str:
    path = pathlib.PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts:
        raise argparse.ArgumentTypeError("fixture must be a relative conformance path")
    manifest = CORPUS / path / "case.toml"
    if not manifest.is_file():
        raise argparse.ArgumentTypeError(f"unknown conformance fixture: {value}")
    matches = [
        item.parent.relative_to(CORPUS).as_posix()
        for item in CORPUS.rglob("case.toml")
        if value in item.parent.relative_to(CORPUS).as_posix()
    ]
    if matches != [value]:
        raise argparse.ArgumentTypeError(
            f"fixture filter {value!r} is not exact; it matches {matches!r}"
        )
    return value


def dependency_line(package: dict) -> str:
    alias = package.get("alias", package["name"]).replace("-", "_")
    values = [f'package = {json.dumps(package["name"])}']
    if "path" in package:
        path = (ROOT / package["path"]).resolve()
        values.append(f"path = {json.dumps(str(path))}")
    else:
        values.append(f'version = {json.dumps(package["version"])}')
    if package.get("features"):
        values.append(f'features = {json.dumps(package["features"])}')
    if package.get("default-features") is False:
        values.append("default-features = false")
    return f'{alias} = {{ {", ".join(values)} }}'


def materialize_profile(profile: dict) -> pathlib.Path:
    directory = WORK_ROOT / "workspaces" / profile["name"]
    if directory.exists():
        shutil.rmtree(directory)
    (directory / "src").mkdir(parents=True)
    manifest = [
        "[package]",
        f'name = "terrane-native-survey-{profile["name"]}"',
        'version = "0.0.0"',
        'edition = "2024"',
        "",
        "[dependencies]",
        *(dependency_line(package) for package in profile["packages"]),
        "",
        "[workspace]",
        "",
    ]
    (directory / "Cargo.toml").write_text("\n".join(manifest))
    (directory / "src" / "lib.rs").write_text("")
    subprocess.run(
        ["cargo", "generate-lockfile", "--offline"],
        cwd=directory,
        check=True,
    )
    return directory / "Cargo.toml"


def run_surveys(profile: dict, manifest: pathlib.Path) -> list[dict]:
    subprocess.run(
        ["cargo", "build", "--package", "terrane-rust-analysis", "--bin", "terrane-rust-survey"],
        cwd=ROOT,
        check=True,
    )
    reports = []
    selected = [package for package in profile["packages"] if package.get("survey", True)]
    for package in selected:
        selector = f'{package["name"]}@{package["version"].removeprefix("=")}'
        command = [
            str(SURVEY_BINARY),
            str(manifest),
            "--package",
            selector,
            "--target",
            profile["target"],
            "--containment",
            profile["containment"],
        ]
        if package.get("probes"):
            probe_path = manifest.parent / f'{package["name"]}-probes.json'
            probe_path.write_text(json.dumps(package["probes"], sort_keys=True) + "\n")
            command.extend(["--probes", str(probe_path)])
        output = subprocess.run(
            command,
            cwd=ROOT,
            check=True,
            stdout=subprocess.PIPE,
            text=True,
        )
        reports.append(json.loads(output.stdout))
    return reports


def run_fixture(profile: dict) -> dict:
    case = tomllib.loads((CORPUS / profile["fixture"] / "case.toml").read_text())
    environment = os.environ.copy()
    environment["TERRANE_CONFORMANCE_FILTER"] = profile["fixture"]
    command = [
        "cargo",
        "test",
        "--package",
        "terrane-compiler",
        "--test",
        "conformance",
        "every_manifest_drives_a_conformance_case",
        "--",
        "--exact",
        "--nocapture",
    ]
    completed = subprocess.run(command, cwd=ROOT, env=environment, check=False)
    if completed.returncode != 0:
        outcome = "failed"
    elif case["status"] == "reject":
        outcome = "declined-as-expected"
    elif case["phase"] == "run":
        outcome = "admitted-and-ran"
    else:
        outcome = "admitted"
    result = {
        "fixture": profile["fixture"],
        "phase": case["phase"],
        "expected-status": case["status"],
        "outcome": profile.get("status", outcome),
        "fixture-outcome": outcome,
        "exit-code": completed.returncode,
    }
    return result


def unavailable_environment(profile: dict) -> dict | None:
    if profile.get("status") != "environment-unavailable":
        return None
    return {
        "fixture": profile["fixture"],
        "outcome": "environment-unavailable",
        "fixture-outcome": "not-attempted",
        "required-environment": profile.get("required-environment", []),
        "exit-code": None,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("profile", help="for example markdown-associated-owner")
    parser.add_argument("--report", type=pathlib.Path)
    arguments = parser.parse_args()
    try:
        profile = load_profile(arguments.profile)
        manifest = materialize_profile(profile)
        surveys = run_surveys(profile, manifest)
        compiler = unavailable_environment(profile) or run_fixture(profile)
    except (argparse.ArgumentTypeError, subprocess.CalledProcessError, OSError, ValueError) as error:
        parser.error(str(error))
    report = {
        "schema": 1,
        "profile": profile,
        "native-surveys": surveys,
        "compiler-application": compiler,
    }
    report_path = arguments.report or WORK_ROOT / "reports" / f"{profile['name']}.json"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(report_path.relative_to(ROOT))
    return 0 if compiler["exit-code"] in {0, None} else compiler["exit-code"]


if __name__ == "__main__":
    sys.exit(main())
