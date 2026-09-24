#!/usr/bin/env python3
"""Run the compiler-backed projection census outside the compiler test corpus."""

from __future__ import annotations

import argparse
import collections
import json
import pathlib
import subprocess
import shutil
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = ROOT / "compatibility/projection-census/packages.yaml"
WORK_ROOT = ROOT / "target/projection-census"
TERRANE = ROOT / "target/debug/terrane"


def load_manifest(path: pathlib.Path) -> dict:
    document = yaml.safe_load(path.read_text())
    if not isinstance(document, dict) or document.get("schema") != 1:
        raise ValueError("package manifest must be a mapping with schema: 1")
    if not isinstance(document.get("targets"), dict) or not isinstance(document.get("packages"), dict):
        raise ValueError("package manifest requires targets and packages mappings")
    return document


def selected_packages(document: dict, priorities: list[str], names: list[str]) -> list[tuple[str, str, dict]]:
    selected = []
    wanted_priorities = set(priorities or document["packages"])
    wanted_names = set(names)
    for priority, packages in document["packages"].items():
        if priority not in wanted_priorities:
            continue
        if not isinstance(packages, dict):
            raise ValueError(f"priority {priority!r} must contain a package mapping")
        for name, configuration in packages.items():
            if wanted_names and name not in wanted_names:
                continue
            if not isinstance(configuration, dict) or "version" not in configuration:
                raise ValueError(f"package {name!r} requires a version")
            selected.append((priority, name, configuration))
    missing = wanted_names - {name for _, name, _ in selected}
    if missing:
        raise ValueError(f"unknown or priority-filtered packages: {', '.join(sorted(missing))}")
    return selected


def run_workflows(name: str, workflows: list[dict], target: dict) -> list[dict]:
    results = []
    for workflow in workflows:
        if workflow.get("status") == "context-only":
            results.append(workflow)
            continue
        source = (ROOT / workflow["package"]).resolve()
        if not source.is_relative_to(ROOT) or not (source / "package.toml").is_file():
            raise ValueError(f"invalid representative workflow package: {workflow['package']}")
        destination = WORK_ROOT / "representative-uses" / name / workflow["name"]
        if destination.exists():
            shutil.rmtree(destination)
        shutil.copytree(source, destination)
        try:
            completed = subprocess.run(
                [str(TERRANE), "check", str(destination)],
                cwd=ROOT,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                timeout=int(target["timeout-seconds"]),
                check=False,
            )
        except subprocess.TimeoutExpired:
            results.append(
                {
                    **workflow,
                    "status": "resource-limited-unproven",
                    "explanation": f"workflow exceeded {target['timeout-seconds']} seconds",
                }
            )
            continue
        if workflow.get("valid", True) is False:
            status = "invalid-test-use"
        elif completed.returncode == 0:
            status = "admitted-concrete-use"
        elif completed.returncode == 3:
            status = "compiler-declined"
        elif completed.returncode == 5:
            status = "engine-defect"
        else:
            status = "environment-failure"
        diagnostic = completed.stderr.strip().replace(
            str(destination), f"representative-use/{name}/{workflow['name']}"
        )
        results.append(
            {
                **workflow,
                "status": status,
                "exit-code": completed.returncode,
                "diagnostic": diagnostic,
            }
        )
    return results


def run_package(priority: str, name: str, package: dict, target: dict) -> dict:
    version = str(package["version"])
    command = [
        str(TERRANE),
        "projection-census",
        name,
        version,
        "--target",
        target["triple"],
        "--root",
        str(WORK_ROOT / "workspaces" / f"{name}-{version}"),
    ]
    if package.get("alias"):
        command.extend(["--alias", package["alias"]])
    if package.get("features"):
        command.extend(["--features", ",".join(package["features"])])
    if package.get("default-features") is False:
        command.append("--no-default-features")
    if package.get("target-condition"):
        command.extend(["--target-condition", package["target-condition"]])
    try:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=int(target["timeout-seconds"]),
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        return {
            "priority": priority,
            "package": name,
            "version": version,
            "outcome": "resource-limited-unproven",
            "explanation": f"assessment exceeded {target['timeout-seconds']} seconds",
            "stderr": error.stderr or "",
            "representative-workflows": package.get("representative-workflows", []),
        }
    if completed.returncode != 0:
        return {
            "priority": priority,
            "package": name,
            "version": version,
            "outcome": "environment-failure",
            "exit-code": completed.returncode,
            "explanation": completed.stderr.strip(),
            "representative-workflows": package.get("representative-workflows", []),
        }
    report = json.loads(completed.stdout)
    report.update(
        {
            "priority": priority,
            "outcome": "assessed",
            "representative-workflows": run_workflows(
                name, package.get("representative-workflows", []), target
            ),
        }
    )
    return report


def summarize(results: list[dict]) -> dict:
    outcomes = collections.Counter(result["outcome"] for result in results)
    classifications = collections.Counter()
    causes = collections.Counter()
    discovered = projected = declined = 0
    workflow_statuses = collections.Counter()
    for result in results:
        for workflow in result.get("representative-workflows", []):
            workflow_statuses[workflow.get("status", "not-run")] += 1
        assessment = result.get("assessment")
        if not assessment:
            continue
        discovered += assessment["discovered-declarations"]
        projected += assessment["projected-operations"]
        declined += assessment["declined-operations"]
        classifications.update(assessment["classifications"])
        for cause in assessment["structural-causes"]:
            causes[(cause["classification"], cause["reason"])] += cause["affected-operations"]
    ranked_causes = [
        {"classification": key[0], "reason": key[1], "affected-operations": count}
        for key, count in sorted(causes.items(), key=lambda item: (-item[1], item[0]))
    ]
    return {
        "package-outcomes": dict(sorted(outcomes.items())),
        "configuration-coverage": {"requested": len(results), "assessed": outcomes["assessed"]},
        "surface-denominators": {
            "discovered-declarations": discovered,
            "projected-operations": projected,
            "declined-operations": declined,
        },
        "declaration-classifications": dict(sorted(classifications.items())),
        "representative-workflow-statuses": dict(sorted(workflow_statuses.items())),
        "ranked-structural-causes": ranked_causes,
        "confidence": "static compiler admission; concrete generic and composition-sensitive uses require representative workflows",
    }


def concise_report(report: dict) -> dict:
    packages = []
    for result in report["packages"]:
        package = {
            "priority": result["priority"],
            "package": result.get("package", result.get("dependency", {}).get("package")),
            "version": result.get("version", result.get("dependency", {}).get("version")),
            "outcome": result["outcome"],
            "representative-workflows": result.get("representative-workflows", []),
        }
        if "assessment" in result:
            package["assessment"] = {
                key: value
                for key, value in result["assessment"].items()
                if key != "declaration-results"
            }
        if "explanation" in result:
            package["explanation"] = result["explanation"]
        packages.append(package)
    return {
        "schema": report["schema"],
        "manifest": report["manifest"],
        "target": report["target"],
        "summary": report["summary"],
        "packages": packages,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=pathlib.Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--target", default="linux-x86-64")
    parser.add_argument("--priority", action="append", default=[])
    parser.add_argument("--package", action="append", default=[])
    parser.add_argument("--report", type=pathlib.Path)
    parser.add_argument("--summary-report", type=pathlib.Path)
    arguments = parser.parse_args()
    try:
        document = load_manifest(arguments.manifest)
        target = document["targets"][arguments.target]
        packages = selected_packages(document, arguments.priority, arguments.package)
    except (OSError, KeyError, TypeError, ValueError, yaml.YAMLError) as error:
        parser.error(str(error))
    subprocess.run(
        ["cargo", "build", "--package", "terrane-cli"], cwd=ROOT, check=True
    )
    results = [run_package(priority, name, package, target) for priority, name, package in packages]
    report = {
        "schema": 1,
        "manifest": arguments.manifest.resolve().relative_to(ROOT).as_posix(),
        "target": {"name": arguments.target, "triple": target["triple"]},
        "summary": summarize(results),
        "packages": results,
    }
    report_path = arguments.report or WORK_ROOT / "reports" / f"{arguments.target}.json"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    if arguments.summary_report:
        arguments.summary_report.parent.mkdir(parents=True, exist_ok=True)
        arguments.summary_report.write_text(
            json.dumps(concise_report(report), indent=2, sort_keys=True) + "\n"
        )
    try:
        print(report_path.resolve().relative_to(ROOT))
    except ValueError:
        print(report_path.resolve())
    return 0 if all(result["outcome"] == "assessed" for result in results) else 1


if __name__ == "__main__":
    sys.exit(main())
