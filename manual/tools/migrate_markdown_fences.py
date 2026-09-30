#!/usr/bin/env python3
"""Convert legacy Markdown fences into independent illustrative example blocks.

The migration deliberately does not infer source/output or multi-file relationships.
It preserves prose order, fence language labels, and fenced bodies exactly.
"""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path

MARKDOWN_BLOCK = re.compile(
    r"^(?P<indent> *)- type: markdown\s*\n"
    r"(?P=indent)  markdown: \|[-+]?\s*\n",
    re.MULTILINE,
)
FENCE = re.compile(
    r"(?ms)^(?P<fence>`{3,}|~{3,})(?P<language>[^\n]*)\n"
    r"(?P<source>.*?)^(?P=fence)[ \t]*$"
)


@dataclass(frozen=True)
class MarkdownBlock:
    start: int
    end: int
    indent: int
    body: str
    fences: tuple[re.Match[str], ...]


def prose_body(lines: list[str], content_indent: int) -> str:
    return "\n".join(
        line[content_indent:] if line.strip() else "" for line in lines
    )


def markdown_blocks(text: str) -> list[MarkdownBlock]:
    lines = text.splitlines()
    offsets = []
    offset = 0
    for line in text.splitlines(keepends=True):
        offsets.append(offset)
        offset += len(line)
    blocks = []
    for match in MARKDOWN_BLOCK.finditer(text):
        start_line = text[: match.start()].count("\n")
        markdown_line = start_line + 1
        key_indent = len(lines[markdown_line]) - len(lines[markdown_line].lstrip(" "))
        content_start = markdown_line + 1
        content_end = content_start
        while content_end < len(lines):
            line = lines[content_end]
            if not line.strip():
                content_end += 1
                continue
            leading = len(line) - len(line.lstrip(" "))
            if leading <= key_indent:
                break
            content_end += 1
        nonempty = [
            len(line) - len(line.lstrip(" "))
            for line in lines[content_start:content_end]
            if line.strip()
        ]
        if not nonempty:
            continue
        body = prose_body(lines[content_start:content_end], min(nonempty))
        fences = tuple(FENCE.finditer(body))
        if fences:
            start = offsets[start_line]
            end = offsets[content_end] if content_end < len(offsets) else len(text)
            block_indent = len(match.group("indent"))
            blocks.append(MarkdownBlock(start, end, block_indent, body, fences))
    return blocks


def block_text(indent: int, kind: str, body: str) -> str:
    prefix = " " * indent
    content_prefix = " " * (indent + 4)
    content = "\n".join(
        content_prefix + line if line else "" for line in body.strip("\n").splitlines()
    )
    return f"{prefix}- type: markdown\n{prefix}  markdown: |-\n{content}\n"


def example_text(indent: int, identifier: str, language: str, source: str) -> str:
    prefix = " " * indent
    content_prefix = " " * (indent + 4)
    source = "\n".join(
        content_prefix + line if line else "" for line in source.splitlines()
    )
    return "\n".join(
        [
            f"{prefix}- type: example",
            f"{prefix}  id: {identifier}",
            f"{prefix}  title: null",
            f"{prefix}  language: {language}",
            f"{prefix}  mode: illustrative",
            f"{prefix}  target: null",
            f"{prefix}  timeout-ms: null",
            f"{prefix}  source: |2",
            source,
            f"{prefix}  response: null",
            f"{prefix}  diagnostic: null",
            f"{prefix}  markdown: null",
            "",
        ]
    )


def next_identifier(text: str, ordinal: int) -> tuple[str, int]:
    while True:
        identifier = f"legacy-fence-{ordinal:03d}"
        ordinal += 1
        if f"id: {identifier}" not in text:
            return identifier, ordinal


def replacement(block: MarkdownBlock, existing: str, ordinal: int) -> tuple[str, int, int]:
    parts = []
    cursor = 0
    migrated = 0
    for fence in block.fences:
        prose = block.body[cursor:fence.start()]
        if prose.strip():
            parts.append(block_text(block.indent, "markdown", prose))
        identifier, ordinal = next_identifier(existing, ordinal)
        language = fence.group("language").strip()
        if not language:
            raise ValueError("an unlabeled Markdown fence needs manual migration")
        parts.append(example_text(block.indent, identifier, language, fence.group("source")))
        cursor = fence.end()
        migrated += 1
    prose = block.body[cursor:]
    if prose.strip():
        parts.append(block_text(block.indent, "markdown", prose))
    return "\n".join(parts).rstrip() + "\n", ordinal, migrated


def migrate_file(path: Path, apply: bool) -> tuple[int, int]:
    text = path.read_text(encoding="utf-8")
    blocks = markdown_blocks(text)
    if not blocks:
        return 0, 0
    replacements = []
    ordinal = 1
    fences = 0
    for block in blocks:
        rendered, ordinal, migrated = replacement(block, text, ordinal)
        replacements.append((block.start, block.end, rendered))
        fences += migrated
    updated = text
    for start, end, rendered in reversed(replacements):
        updated = updated[:start] + rendered + updated[end:]
    if apply:
        path.write_text(updated, encoding="utf-8")
    return len(blocks), fences


def record_paths(root: Path) -> list[Path]:
    return sorted(root.glob("reference/records/**/*.yaml")) + sorted(
        root.glob("tutorial-book/records/**/*.yaml")
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="write the migration")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    scalars = fences = 0
    for path in record_paths(root):
        migrated_scalars, migrated_fences = migrate_file(path, args.apply)
        scalars += migrated_scalars
        fences += migrated_fences
    action = "migrated" if args.apply else "would migrate"
    print(f"{action} {fences} fences from {scalars} Markdown scalars")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
