#!/usr/bin/env python3
"""Generate and verify lookup artifacts for the Terrane manual."""

from __future__ import annotations

import argparse
import hashlib
import re
import subprocess
import sys
from pathlib import Path
from typing import Any, Iterable

import yaml
from format_manual_content import format_yaml_documents, reflow_markdown_scalars


CODE_RE = re.compile(r'\b([LSTW][0-9]{4})\b')
DECLARATION_RE = re.compile(
    r"^\s*(?:exported\s+)?(?:class|constant|descriptor|effect|enum|function|protocol)\s+"
)
FENCE_RE = re.compile(r"```terrane\n(.*?)\n```", re.DOTALL)
INLINE_CODE_RE = re.compile(r"`([^`\n]+)`")
REFERENCE_RE = re.compile(r"\[\[([^\]#]+)(?:#[^\]]+)?\]\]")



def load_yaml(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        value = yaml.safe_load(handle)
    if not isinstance(value, dict):
        raise ValueError(f"{path}: expected a YAML mapping")
    return value


def dump_yaml(value: Any) -> str:
    return yaml.safe_dump(value, sort_keys=False, allow_unicode=True, width=100)


def formatting_paths(root: Path) -> list[Path]:
    paths = set(root.glob("*/manual.yaml"))
    for manifest_path in sorted(paths):
        manifest = load_yaml(manifest_path)
        for group in manifest.get("navigation", []):
            for child in group.get("children", []):
                source = child.get("source")
                if source and str(source).endswith((".yaml", ".yml")):
                    record_path = manifest_path.parent / source
                    if not record_path.exists():
                        raise ValueError(f"{manifest_path}: missing record {source}")
                    paths.add(record_path)
    return sorted(paths)




def restore_files(snapshots: dict[Path, bytes]) -> None:
    for path, content in snapshots.items():
        if path.read_bytes() != content:
            path.write_bytes(content)


def manifest_records(root: Path) -> list[dict[str, Any]]:
    entries: list[dict[str, Any]] = []
    for manifest_path in sorted(root.glob("*/manual.yaml")):
        manifest = load_yaml(manifest_path)
        publication = manifest.get("publication", {})
        manual_id = publication.get("id", manifest_path.parent.name)
        manual_title = manifest.get("title")
        position = 0
        for group in manifest.get("navigation", []):
            group_name = group.get("group", "")
            for child in group.get("children", []):
                source = child.get("source")
                if not source or not str(source).endswith(('.yaml', '.yml')):
                    continue
                record_path = manifest_path.parent / source
                if not record_path.exists():
                    raise ValueError(f"{manifest_path}: missing record {source}")
                record = load_yaml(record_path)
                position += 1
                entries.append(
                    {
                        "manual": manual_id,
                        "manual-title": manual_title,
                        "group": group_name,
                        "position": position,
                        "source": record_path.relative_to(root).as_posix(),
                        "record": record,
                    }
                )
    return entries


def sections(record: dict[str, Any]) -> Iterable[dict[str, Any]]:
    documentation = record.get("documentation", {})
    yield from documentation.get("sections", []) or []


def blocks(section: dict[str, Any]) -> Iterable[dict[str, Any]]:
    yield from section.get("blocks", []) or []
    for nested in section.get("children", []) or []:
        yield from blocks(nested)


def markdown_values(block: dict[str, Any]) -> Iterable[str]:
    for key in ("markdown", "source"):
        value = block.get(key)
        if isinstance(value, str):
            yield value
    for key in ("rejected-example", "corrected-example"):
        value = block.get(key)
        if isinstance(value, dict) and isinstance(value.get("source"), str):
            yield value["source"]


def record_ids(entries: list[dict[str, Any]]) -> set[str]:
    return {str(entry["record"].get("id")) for entry in entries}


def validate_records(root: Path, entries: list[dict[str, Any]], compiler_root: Path | None) -> None:
    ids = record_ids(entries)
    duplicates = sorted(record_id for record_id in ids if sum(e["record"].get("id") == record_id for e in entries) > 1)
    if duplicates:
        raise ValueError(f"duplicate record IDs: {', '.join(duplicates)}")

    for entry in entries:
        record = entry["record"]
        record_id = record.get("id")
        if not record_id:
            raise ValueError(f"{entry['source']}: missing record ID")
        seen_sections: set[str] = set()
        for section in sections(record):
            section_id = section.get("id")
            if section_id:
                if section_id in seen_sections:
                    raise ValueError(f"{entry['source']}: duplicate section ID {section_id}")
                seen_sections.add(section_id)

        synopsis_sections = [
            section for section in sections(record) if "synopsis" in str(section.get("id", ""))
        ]
        if compiler_root is not None and synopsis_sections:
            implementation = record.get("implementation", {}).get("compiler", []) or []
            source_text = ""
            for relative in implementation:
                source_path = compiler_root / relative
                if not source_path.exists():
                    raise ValueError(f"{entry['source']}: missing compiler source {relative}")
                source_text += "\n" + source_path.read_text(encoding="utf-8")
            normalized = {line.strip().rstrip(";") for line in source_text.splitlines()}
            for section in synopsis_sections:
                for block in blocks(section):
                    for markdown in markdown_values(block):
                        for fence in FENCE_RE.findall(markdown):
                            for line in fence.splitlines():
                                candidate = line.strip().rstrip(";")
                                if DECLARATION_RE.match(candidate) and candidate not in normalized:
                                    raise ValueError(
                                        f"{entry['source']}: synopsis declaration not in compiler source: {candidate}"
                                    )


def record_synopsis(record: dict[str, Any], compiler_root: Path | None) -> list[str]:
    if compiler_root is None:
        return []
    declarations: list[str] = []
    for relative in record.get("implementation", {}).get("compiler", []) or []:
        path = compiler_root / relative
        if not path.exists() or path.suffix != ".trn":
            continue
        for line in path.read_text(encoding="utf-8").splitlines():
            candidate = line.strip().rstrip(";")
            if DECLARATION_RE.match(candidate):
                declarations.append(candidate)
    return declarations


def section_outline(section_values: Iterable[dict[str, Any]]) -> list[dict[str, Any]]:
    return [
        {
            "id": section.get("id"),
            "title": section.get("title"),
            "sections": section_outline(section.get("sections", []) or []),
        }
        for section in section_values
    ]


def catalog(entries: list[dict[str, Any]], compiler_root: Path | None) -> dict[str, Any]:
    records = []
    for entry in entries:
        record = entry["record"]
        documentation = record.get("documentation", {})
        provenance = record.get("provenance", {})
        records.append(
            {
                "id": record.get("id"),
                "title": documentation.get("title"),
                "summary": documentation.get("summary"),
                "kind": record.get("kind"),
                "manual": entry["manual"],
                "group": entry["group"],
                "position": entry["position"],
                "source": entry["source"],
                "lifecycle-status": record.get("lifecycle", {}).get("status"),
                "documentation-status": documentation.get("status"),
                "surface": record.get("surface"),
                "provenance": {
                    "specification": provenance.get("specification", []),
                    "conformance": provenance.get("conformance", []),
                    "compiler": provenance.get("compiler"),
                },
                "sections": section_outline(documentation.get("sections", []) or []),
                "declaration-synopsis": record_synopsis(record, compiler_root),
            }
        )
    return {"format": 2, "generated": True, "records": records}


def symbol_index(entries: list[dict[str, Any]], compiler_root: Path | None) -> dict[str, Any]:
    symbols: dict[tuple[str, str, str, str], dict[str, str]] = {}
    interesting_row_keys = ("declaration", "operation", "path", "prefix", "state")
    for entry in entries:
        record = entry["record"]
        record_id = str(record.get("id"))
        surface = record.get("surface")
        if isinstance(surface, str) and surface:
            key = (surface, record_id, "", surface)
            symbols[key] = {"name": surface, "record": record_id, "section": "", "declaration": surface}
        for section in sections(record):
            section_id = str(section.get("id"))
            for block in blocks(section):
                for row in block.get("rows", []) or []:
                    for row_key in interesting_row_keys:
                        value = row.get(row_key)
                        if isinstance(value, str) and value:
                            name = value.split(";", 1)[0].strip()
                            key = (name, record_id, section_id, value)
                            symbols[key] = {
                                "name": name,
                                "record": record_id,
                                "section": section_id,
                                "declaration": value,
                            }
                code = block.get("code")
                if isinstance(code, str):
                    key = (code, record_id, section_id, code)
                    symbols[key] = {"name": code, "record": record_id, "section": section_id, "declaration": code}
                for markdown in markdown_values(block):
                    for fence in FENCE_RE.findall(markdown):
                        for line in fence.splitlines():
                            declaration = line.strip().rstrip(";")
                            if DECLARATION_RE.match(declaration):
                                words = declaration.split()
                                name_index = 2 if words[0] == "exported" else 1
                                name = words[name_index].split(";")[0]
                                key = (name, record_id, section_id, declaration)
                                symbols[key] = {
                                    "name": name,
                                    "record": record_id,
                                    "section": section_id,
                                    "declaration": declaration,
                                }
                    for inline in INLINE_CODE_RE.findall(markdown):
                        if re.fullmatch(r"[/A-Za-z][A-Za-z0-9_./:-]*", inline):
                            key = (inline, record_id, section_id, inline)
                            symbols[key] = {
                                "name": inline,
                                "record": record_id,
                                "section": section_id,
                                "declaration": inline,
                            }
        for declaration in record_synopsis(record, compiler_root):
            words = declaration.split()
            name_index = 2 if words[0] == "exported" else 1
            name = words[name_index].split(";", 1)[0]
            key = (name, record_id, "compiler-synopsis", declaration)
            symbols[key] = {
                "name": name,
                "record": record_id,
                "section": "compiler-synopsis",
                "declaration": declaration,
            }
    return {
        "format": 1,
        "generated": True,
        "symbols": sorted(symbols.values(), key=lambda item: (item["name"].lower(), item["record"], item["section"])),
    }


def compiler_surface(compiler_root: Path) -> dict[str, Any]:
    modules = []
    core = compiler_root / "crates/terrane-compiler/src/core"
    for path in sorted(core.rglob("*.trn")):
        declarations = []
        for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            candidate = line.strip().rstrip(";")
            if DECLARATION_RE.match(candidate):
                declarations.append({"declaration": candidate, "line": line_number})
        modules.append({"source": path.relative_to(compiler_root).as_posix(), "declarations": declarations})
    return {"format": 1, "generated": True, "modules": modules}


def diagnostic_inventory(compiler_root: Path) -> dict[str, Any]:
    found: dict[str, dict[str, Any]] = {}
    source_root = compiler_root / "crates/terrane-compiler/src"
    for path in sorted(source_root.rglob("*.rs")):
        text = path.read_text(encoding="utf-8")
        for match in CODE_RE.finditer(text):
            code = match.group(1)
            line = text.count("\n", 0, match.start()) + 1
            entry = found.setdefault(code, {"code": code, "messages": [], "sources": []})
            source = {"path": path.relative_to(compiler_root).as_posix(), "line": line}
            if source not in entry["sources"]:
                entry["sources"].append(source)
            following = text[match.end() + 1 : match.end() + 501]
            message_match = re.search(r'"((?:[^"\\]|\\.){8,})"', following)
            if message_match:
                message = bytes(message_match.group(1), "utf-8").decode("unicode_escape")
                if message not in entry["messages"]:
                    entry["messages"].append(message)
    return {"format": 1, "generated": True, "diagnostics": [found[code] for code in sorted(found)]}


def with_generation_metadata(value: dict[str, Any], inputs: list[Path], root: Path) -> dict[str, Any]:
    digest = hashlib.sha256()
    for path in sorted(inputs):
        try:
            label = path.relative_to(root).as_posix()
        except ValueError:
            label = path.as_posix()
        digest.update(label.encode())
        digest.update(path.read_bytes())
    return {"generation": {"tool": "tools/generate_manual_catalog.py", "input-sha256": digest.hexdigest()}, **value}


def outputs(root: Path, compiler_root: Path | None) -> dict[Path, str]:
    entries = manifest_records(root)
    validate_records(root, entries, compiler_root)
    inputs = [root / entry["source"] for entry in entries] + sorted(root.glob("*/manual.yaml"))
    generated = root / "generated"
    result = {
        generated / "catalog.yaml": dump_yaml(with_generation_metadata(catalog(entries, compiler_root), inputs, root)),
        generated / "symbol-index.yaml": dump_yaml(with_generation_metadata(symbol_index(entries, compiler_root), inputs, root)),
    }
    if compiler_root is not None:
        compiler_inputs = sorted((compiler_root / "crates/terrane-compiler/src/core").rglob("*.trn"))
        compiler_inputs += sorted((compiler_root / "crates/terrane-compiler/src").rglob("*.rs"))
        result[generated / "compiler-surface.yaml"] = dump_yaml(
            with_generation_metadata(compiler_surface(compiler_root), inputs + compiler_inputs, root)
        )
        result[generated / "diagnostics.yaml"] = dump_yaml(
            with_generation_metadata(diagnostic_inventory(compiler_root), inputs + compiler_inputs, root)
        )
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--check",
        action="store_true",
        help="format YAML, then fail if generated output is stale",
    )
    parser.add_argument("--compiler-root", type=Path, default=Path(".."))
    args = parser.parse_args()

    root = Path(__file__).resolve().parents[1]
    compiler_root = args.compiler_root.resolve() if args.compiler_root else None
    paths = formatting_paths(root)
    snapshots = {path: path.read_bytes() for path in paths}
    try:
        format_yaml_documents(paths)
        reflow_markdown_scalars(paths)
        format_yaml_documents(paths)
        generated = outputs(root, compiler_root)
        stale = []
        for path, content in generated.items():
            if args.check:
                if not path.exists() or path.read_text(encoding="utf-8") != content:
                    stale.append(path.relative_to(root).as_posix())
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content, encoding="utf-8")
    except subprocess.CalledProcessError as error:
        restore_files(snapshots)
        restored = " and restored YAML inputs" if snapshots else ""
        print(
            f"manual formatting or generation failed{restored}: "
            f"command exited with status {error.returncode}",
            file=sys.stderr,
        )
        return 1
    except Exception as error:
        restore_files(snapshots)
        restored = " and restored YAML inputs" if snapshots else ""
        print(f"manual formatting or generation failed{restored}: {error}", file=sys.stderr)
        return 1
    if stale:
        print("stale generated manual artifacts: " + ", ".join(stale), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
