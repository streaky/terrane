#!/usr/bin/env python3
"""Promote legacy fences nested in typed Markdown-bearing blocks.

The parent block keeps its identity and prose before the first fence. Each fence
becomes an independent illustrative example; intervening/following prose becomes
ordinary Markdown immediately after the parent block.
"""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path

from migrate_markdown_fences import FENCE, block_text, example_text, next_identifier

MARKDOWN_KEY = re.compile(r"^(?P<indent> *)markdown: \|[-+]?\s*$")
TYPE_BLOCK = re.compile(r"^(?P<indent> *)- type: (?P<kind>[a-z-]+)\s*$")


@dataclass(frozen=True)
class Occurrence:
    key_start: int
    content_end: int
    parent_end: int
    key_indent: int
    block_indent: int
    body: str
    fences: tuple[re.Match[str], ...]


def offsets_for(text: str) -> list[int]:
    offsets = []
    offset = 0
    for line in text.splitlines(keepends=True):
        offsets.append(offset)
        offset += len(line)
    return offsets


def occurrences(text: str) -> list[Occurrence]:
    lines = text.splitlines()
    offsets = offsets_for(text)
    found = []
    for key_line, line in enumerate(lines):
        key = MARKDOWN_KEY.match(line)
        if key is None:
            continue
        key_indent = len(key.group("indent"))
        parent_line = key_line - 1
        while parent_line >= 0:
            parent = TYPE_BLOCK.match(lines[parent_line])
            if parent is not None and len(parent.group("indent")) < key_indent:
                break
            parent_line -= 1
        if parent_line < 0:
            continue
        block_indent = len(TYPE_BLOCK.match(lines[parent_line]).group("indent"))
        content_start = key_line + 1
        content_end = content_start
        while content_end < len(lines):
            candidate = lines[content_end]
            if not candidate.strip():
                content_end += 1
                continue
            leading = len(candidate) - len(candidate.lstrip(" "))
            if leading <= key_indent:
                break
            content_end += 1
        nonempty = [
            len(candidate) - len(candidate.lstrip(" "))
            for candidate in lines[content_start:content_end]
            if candidate.strip()
        ]
        if not nonempty:
            continue
        content_indent = min(nonempty)
        body = "\n".join(
            candidate[content_indent:] if candidate.strip() else ""
            for candidate in lines[content_start:content_end]
        )
        fences = tuple(FENCE.finditer(body))
        if not fences:
            continue
        parent_end_line = parent_line + 1
        while parent_end_line < len(lines):
            candidate = lines[parent_end_line]
            if candidate.strip():
                leading = len(candidate) - len(candidate.lstrip(" "))
                if leading < block_indent or (
                    leading == block_indent and candidate.startswith(" " * block_indent + "- ")
                ):
                    break
            parent_end_line += 1
        found.append(
            Occurrence(
                offsets[key_line],
                offsets[content_end] if content_end < len(offsets) else len(text),
                offsets[parent_end_line] if parent_end_line < len(offsets) else len(text),
                key_indent,
                block_indent,
                body,
                fences,
            )
        )
    return found


def scalar_text(key_indent: int, prose: str) -> str:
    prefix = " " * key_indent
    content_prefix = " " * (key_indent + 2)
    body = "\n".join(
        content_prefix + line if line else "" for line in prose.strip("\n").splitlines()
    )
    return f"{prefix}markdown: |-\n{body}\n"


def migrate_file(path: Path, apply: bool) -> int:
    text = path.read_text(encoding="utf-8")
    items = occurrences(text)
    if not items:
        return 0
    replacements = []
    ordinal = 1
    total = 0
    for item in items:
        preamble = item.body[: item.fences[0].start()]
        if not preamble.strip():
            raise ValueError(f"{path}: typed Markdown block has no prose before its first fence")
        inserted = []
        cursor = 0
        for fence in item.fences:
            identifier, ordinal = next_identifier(text, ordinal)
            language = fence.group("language").strip()
            if not language:
                raise ValueError(f"{path}: unlabeled Markdown fence needs manual migration")
            inserted.append(example_text(item.block_indent, identifier, language, fence.group("source")))
            cursor = fence.end()
            following = item.body[cursor : item.fences[item.fences.index(fence) + 1].start()] if fence != item.fences[-1] else item.body[cursor:]
            if following.strip():
                inserted.append(block_text(item.block_indent, "markdown", following))
            total += 1
        replacements.append((item.key_start, item.content_end, scalar_text(item.key_indent, preamble)))
        replacements.append((item.parent_end, item.parent_end, "\n" + "\n".join(inserted)))
    updated = text
    for start, end, rendered in sorted(replacements, key=lambda change: change[0], reverse=True):
        updated = updated[:start] + rendered + updated[end:]
    if apply:
        path.write_text(updated, encoding="utf-8")
    return total


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    paths = sorted(root.glob("reference/records/**/*.yaml")) + sorted(root.glob("tutorial-book/records/**/*.yaml"))
    total = sum(migrate_file(path, args.apply) for path in paths)
    print(f"{'migrated' if args.apply else 'would migrate'} {total} typed-block fences")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
