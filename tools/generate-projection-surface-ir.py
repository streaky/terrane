#!/usr/bin/env python3
"""Generate complete, human-readable application surface IRs from the projection census."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import subprocess
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[1]
CENSUS_REPORT = ROOT / "target/projection-census/reports/linux-x86-64.json"
CACHE_ROOT = ROOT / "target/projection-surface-ir/surveys"
OUTPUT_PATH = ROOT / "compatibility/projection-census/application-surfaces.yaml"
SURVEY_BINARY = ROOT / "target/debug/terrane-rust-survey"
TARGET = "x86_64-unknown-linux-gnu"
IR_SCHEMA = 1
SURVEY_CACHE_SCHEMA = 1
PROFILES = {
    "high-mid": ["high-priority", "mid-priority"],
    "high-mid-low": ["high-priority", "mid-priority", "low-priority"],
}



def compact_diagnostic(reason: str) -> dict:
    encoded = reason.encode()
    lines = reason.splitlines()
    return {
        "summary": "\n".join(lines[:20])[:4000],
        "sha256": hashlib.sha256(encoded).hexdigest(),
        "truncated": len(lines) > 20 or len(reason) > 4000,
    }


class NoAliasDumper(yaml.SafeDumper):
    def ignore_aliases(self, data: object) -> bool:
        return True


def workspace_manifest(root: dict) -> pathlib.Path:
    dependency = root["dependency"]
    version = dependency["version"].lstrip("=")
    return (
        ROOT
        / "target/projection-census/workspaces"
        / f"{dependency['package']}-{version}"
        / ".trn/dependencies/Cargo.toml"
    )


def cache_path(identity: str, manifest: pathlib.Path) -> pathlib.Path:
    lock = manifest.with_name("Cargo.lock")
    digest = hashlib.sha256()
    digest.update(str(SURVEY_CACHE_SCHEMA).encode())
    digest.update(identity.encode())
    digest.update(lock.read_bytes())
    safe = "".join(character if character.isalnum() else "-" for character in identity)
    return CACHE_ROOT / f"{safe}-{digest.hexdigest()[:16]}.json"


def isolated_surface_manifest(
    identity: str, package: dict, contexts: list[dict]
) -> pathlib.Path | None:
    if not (package.get("source") or "").startswith("registry+"):
        return None
    digest = hashlib.sha256(identity.encode()).hexdigest()[:16]
    workspace = ROOT / "target/projection-surface-ir/fallback" / digest
    source = workspace / "src"
    source.mkdir(parents=True, exist_ok=True)
    (source / "lib.rs").write_text("")
    features = sorted(
        {
            feature
            for context in contexts
            for feature in context["features"]
            if feature != "default"
        }
    )
    rendered_features = ", ".join(json.dumps(feature) for feature in features)
    manifest = workspace / "Cargo.toml"
    manifest.write_text(
        "[package]\n"
        f'name = "terrane-surface-{digest}"\n'
        'version = "0.0.0"\n'
        'edition = "2024"\n\n'
        "[dependencies.surveyed]\n"
        f'package = {json.dumps(package["name"])}\n'
        f'version = {json.dumps("=" + package["version"])}\n'
        "default-features = false\n"
        f"features = [{rendered_features}]\n"
    )
    try:
        subprocess.run(
            ["cargo", "generate-lockfile", "--manifest-path", str(manifest)],
            cwd=ROOT,
            check=True,
            capture_output=True,
        )
    except subprocess.CalledProcessError:
        return None
    return manifest


def survey_package(identity: str, manifest: pathlib.Path, timeout: int) -> dict:
    cache = cache_path(identity, manifest)
    if cache.is_file():
        return json.loads(cache.read_text())
    command = [
        str(SURVEY_BINARY),
        str(manifest),
        "--package",
        identity,
        "--target",
        TARGET,
        "--complete-surface",
        "true",
    ]
    try:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=timeout,
            check=False,
        )
    except subprocess.TimeoutExpired:
        result = {
            "status": "resource-limited-unproven",
            "reason": f"complete public API survey exceeded {timeout} seconds",
        }
    else:
        if completed.returncode == 0:
            result = {"status": "surveyed", "report": json.loads(completed.stdout)}
        else:
            result = {
                "status": "unavailable",
                "exit-code": completed.returncode,
                "reason": completed.stderr.strip(),
            }
    cache.parent.mkdir(parents=True, exist_ok=True)
    cache.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    return result


def selected_roots(census: dict, priorities: list[str]) -> list[dict]:
    allowed = set(priorities)
    return [root for root in census["packages"] if root["priority"] in allowed]


def merged_graph(roots: list[dict]) -> tuple[dict[str, dict], dict[str, list[dict]]]:
    packages: dict[str, dict] = {}
    contexts: dict[str, list[dict]] = {}
    for root in roots:
        root_name = root["dependency"]["package"]
        manifest = workspace_manifest(root)
        for package in root["native"]["resolved_packages"]:
            identity = package["identity"]
            context = {
                "root": root_name,
                "manifest": manifest.relative_to(ROOT).as_posix(),
                "features": package["features"],
                "dependencies": package["dependencies"],
            }
            contexts.setdefault(identity, []).append(context)
            existing = packages.setdefault(
                identity,
                {
                    "identity": identity,
                    "name": package["name"],
                    "version": package["version"],
                    "source": package["source"],
                    "manifest": package["manifest"],
                    "contexts": [],
                },
            )
            if existing["source"] != package["source"]:
                existing.setdefault("conflicting-sources", []).append(package["source"])
    for identity, package in packages.items():
        package["contexts"] = sorted(
            contexts[identity], key=lambda context: context["root"]
        )
    return packages, contexts


def compact_native_surface(report: dict) -> dict:
    return {
        "selected-package": report["selected_package"],
        "rustdoc": {
            "format": report["rustdoc_format"],
            "toolchain": report["rustdoc_toolchain"],
            "visibility-policy": report["rustdoc_visibility_policy"],
        },
        "public-paths": report["public_paths"],
        "declarations": report["declarations"],
        "discovery-failures": report["discovery_failures"],
        "complete-public-api": [
            item["rendered"] for item in report.get("complete_public_api", [])
        ],
    }


def root_projection(root: dict) -> dict:
    return {
        "priority": root["priority"],
        "declared-dependency": root["dependency"],
        "representative-workflows": root.get("representative-workflows", []),
        "assessment": root["assessment"],
        "projection": root["projection"],
        "closure-survey-failures": root["native-closure-failures"],
        "sysroot-survey-failures": root["native-sysroot-failures"],
    }


def generate_profile(census: dict, profile: str, timeout: int) -> dict:
    priorities = PROFILES[profile]
    roots = selected_roots(census, priorities)
    packages, contexts = merged_graph(roots)
    surfaces = []
    for index, identity in enumerate(sorted(packages), start=1):
        package_contexts = sorted(contexts[identity], key=lambda item: item["root"])
        attempts = []
        raw_reasons = []
        result = None
        context = package_contexts[0]
        for candidate in package_contexts:
            candidate_result = survey_package(
                identity, ROOT / candidate["manifest"], timeout
            )
            attempt = {"root": candidate["root"], "status": candidate_result["status"]}
            if candidate_result["status"] != "surveyed":
                raw_reasons.append(candidate_result["reason"])
                attempt["diagnostic"] = compact_diagnostic(candidate_result["reason"])
            attempts.append(attempt)
            if candidate_result["status"] == "surveyed":
                context = candidate
                result = candidate_result
                break
        fallback = None
        if result is None:
            fallback = isolated_surface_manifest(
                identity, packages[identity], package_contexts
            )
            if fallback is not None:
                fallback_result = survey_package(identity, fallback, timeout)
                attempt = {
                    "root": "isolated-locked-package",
                    "status": fallback_result["status"],
                }
                if fallback_result["status"] != "surveyed":
                    raw_reasons.append(fallback_result["reason"])
                    attempt["diagnostic"] = compact_diagnostic(
                        fallback_result["reason"]
                    )
                attempts.append(attempt)
                if fallback_result["status"] == "surveyed":
                    result = fallback_result
        if result is None:
            result = fallback_result if fallback is not None else candidate_result
            if raw_reasons and all(
                "no library target" in reason
                or "does not have a library target" in reason
                for reason in raw_reasons
            ):
                result = {
                    "status": "no-library-target",
                    "reason": "resolved package exposes no Rust library target",
                }
        surface = {
            "package": identity,
            "survey-context-root": context["root"],
            "status": result["status"],
            "attempts": attempts,
        }
        if result["status"] == "surveyed":
            surface["native-surface"] = compact_native_surface(result["report"])
        else:
            surface["diagnostic"] = compact_diagnostic(result["reason"])
        surfaces.append(surface)
        print(
            f"[{profile}] {index}/{len(packages)} {identity}: {result['status']}",
            file=sys.stderr,
        )
    roots_ir = {
        root["dependency"]["package"]: root_projection(root)
        for root in sorted(roots, key=lambda item: item["dependency"]["package"])
    }
    unavailable = [surface for surface in surfaces if surface["status"] != "surveyed"]
    public_items = sum(
        len(surface["native-surface"]["complete-public-api"])
        for surface in surfaces
        if surface["status"] == "surveyed"
    )
    return {
        "schema": IR_SCHEMA,
        "kind": "terrane-application-projection-surface",
        "profile": {
            "name": profile,
            "priorities": priorities,
            "target": census["target"],
            "root-package-count": len(roots),
            "resolved-package-count": len(packages),
            "surveyed-package-count": len(packages) - len(unavailable),
            "unavailable-package-count": len(unavailable),
            "rendered-public-item-count": public_items,
            "completeness-policy": "superset-first: retain every item rendered by public-api from pinned Rustdoc JSON, every declaration record, every projection result, and every failed survey",
        },
        "root-projections": roots_ir,
        "resolved-graph": {"packages": [packages[key] for key in sorted(packages)]},
        "native-surfaces": surfaces,
    }

def combine_profiles(documents: list[dict]) -> dict:
    largest = max(documents, key=lambda document: len(document["native-surfaces"]))
    profiles = {}
    for document in documents:
        name = document["profile"]["name"]
        profiles[name] = {
            "summary": document["profile"],
            "root-projections": document["root-projections"],
            "resolved-graph": document["resolved-graph"],
            "package-surfaces": [
                surface["package"] for surface in document["native-surfaces"]
            ],
        }
    return {
        "schema": IR_SCHEMA,
        "kind": "terrane-application-projection-surface-set",
        "profiles": profiles,
        "package-surfaces": largest["native-surfaces"],
    }


def validate_document(document: dict) -> None:
    surfaces = [surface["package"] for surface in document["package-surfaces"]]
    if surfaces != sorted(set(surfaces)):
        raise ValueError("package surfaces must be unique and sorted")
    surface_set = set(surfaces)
    for name, profile in document["profiles"].items():
        packages = profile["resolved-graph"]["packages"]
        identities = [package["identity"] for package in packages]
        if len(identities) != len(set(identities)):
            raise ValueError(f"{name} contains duplicate package identities")
        identity_set = set(identities)
        if set(profile["package-surfaces"]) != identity_set:
            raise ValueError(f"{name} surface references do not match its graph")
        if not identity_set <= surface_set:
            raise ValueError(f"{name} references a missing shared package surface")
        for package in packages:
            for context in package["contexts"]:
                missing = set(context["dependencies"]) - identity_set
                if missing:
                    raise ValueError(
                        f"{name}:{package['identity']} references packages outside its graph: {sorted(missing)}"
                    )
        for root in profile["root-projections"].values():
            package = root["declared-dependency"]["package"]
            version = root["declared-dependency"]["version"].lstrip("=")
            if f"{package}@{version}" not in identity_set:
                raise ValueError(f"{name} root {package}@{version} is absent from its graph")


def render_document(document: dict) -> str:
    validate_document(document)
    return yaml.dump(
        document,
        Dumper=NoAliasDumper,
        allow_unicode=True,
        default_flow_style=False,
        sort_keys=False,
        width=120,
    )




def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--census-report", type=pathlib.Path, default=CENSUS_REPORT)
    parser.add_argument("--output", type=pathlib.Path, default=OUTPUT_PATH)
    parser.add_argument("--profile", choices=sorted(PROFILES), action="append")
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--package-timeout", type=int, default=1800)
    arguments = parser.parse_args()
    census = json.loads(arguments.census_report.read_text())
    subprocess.run(
        ["cargo", "build", "--package", "terrane-rust-analysis", "--bin", "terrane-rust-survey"],
        cwd=ROOT,
        check=True,
    )
    profiles = arguments.profile or list(PROFILES)
    documents = [
        generate_profile(census, profile, arguments.package_timeout)
        for profile in profiles
    ]
    rendered = render_document(combine_profiles(documents))
    arguments.output.parent.mkdir(parents=True, exist_ok=True)
    if arguments.check:
        if not arguments.output.is_file() or arguments.output.read_text() != rendered:
            raise SystemExit(f"{arguments.output.relative_to(ROOT)} is stale")
    else:
        arguments.output.write_text(rendered)
    print(arguments.output.resolve().relative_to(ROOT))
    return 0


if __name__ == "__main__":
    sys.exit(main())
