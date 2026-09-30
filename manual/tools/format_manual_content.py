#!/usr/bin/env python3
"""Format manual YAML and reflow Markdown literal scalars safely."""

from __future__ import annotations

import re
import shutil
import subprocess
from pathlib import Path

PRETTIER_VERSION = "3.9.8"
PRINT_WIDTH = 80

MARKER = re.compile(r"^<!-- terrane-manual-block: (?P<index>\d+) -->$")
SCALAR_START = re.compile(r"^(?P<indent> *)markdown: [|>][+-]?\s*$")
CODE_FENCE = re.compile(r"(?m)^[ \t]*(?:`{3,}|~{3,})")


def prettier_path() -> str:
    executable = shutil.which("prettier")
    if executable is None:
        raise RuntimeError(
            "prettier is required; install prettier@"
            f"{PRETTIER_VERSION} and make it available on PATH"
        )
    version = subprocess.run(
        [executable, "--version"],
        check=True,
        capture_output=True,
        encoding="utf-8",
    ).stdout.strip()
    if version != PRETTIER_VERSION:
        raise RuntimeError(
            f"prettier {PRETTIER_VERSION} is required, found {version or 'an unknown version'}"
        )
    return executable

def format_yaml_documents(paths: list[Path]) -> None:
    executable = prettier_path()
    result = subprocess.run(
        [
            executable,
            "--write",
            "--parser",
            "yaml",
            "--print-width",
            str(PRINT_WIDTH),
            *[str(path) for path in paths],
        ],
        check=False,
        capture_output=True,
        encoding="utf-8",
    )
    if result.returncode:
        output = result.stderr.strip() or result.stdout.strip() or "no formatter output"
        raise RuntimeError(f"prettier YAML formatting failed: {output}")


def scalar_blocks(lines: list[str]) -> list[tuple[int, int, int]]:
    """Return (content-start, content-end, content-indent) for prose scalars."""
    blocks = []
    line_index = 0
    while line_index < len(lines):
        match = SCALAR_START.match(lines[line_index])
        if match is None:
            line_index += 1
            continue
        key_indent = len(match.group("indent"))
        content_start = line_index + 1
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
        if nonempty:
            blocks.append((content_start, content_end, min(nonempty)))
        line_index = content_end
    return blocks


def format_blocks(executable: str, blocks: list[str]) -> list[str]:
    if not blocks:
        return []
    payload = "\n\n".join(
        f"{block.rstrip()}\n\n<!-- terrane-manual-block: {index} -->"
        for index, block in enumerate(blocks)
    )
    output = subprocess.run(
        [
            executable,
            "--parser",
            "markdown",
            "--print-width",
            str(PRINT_WIDTH),
            "--prose-wrap",
            "always",
        ],
        input=payload,
        check=True,
        capture_output=True,
        encoding="utf-8",
    ).stdout
    formatted = []
    current = []
    expected = 0
    for line in output.splitlines():
        marker = MARKER.match(line)
        if marker is None:
            current.append(line)
            continue
        if int(marker.group("index")) != expected:
            raise RuntimeError("prettier did not preserve manual content block boundaries")
        formatted.append("\n".join(current).strip("\n"))
        current = []
        expected += 1
    if expected != len(blocks):
        raise RuntimeError("prettier removed a manual content block boundary")
    return formatted


def reflow_markdown_scalars(paths: list[Path]) -> None:
    executable = prettier_path()
    for path in paths:
        lines = path.read_text(encoding="utf-8").splitlines()
        block_ranges = scalar_blocks(lines)
        if not block_ranges:
            continue
        reflowable = []
        for start, end, indent in block_ranges:
            block = "\n".join(
                line[indent:] if line.strip() else "" for line in lines[start:end]
            )
            if not CODE_FENCE.search(block):
                reflowable.append(((start, end, indent), block))
        formatted_blocks = format_blocks(executable, [block for _, block in reflowable])
        for ((start, end, indent), _), block in reversed(
            list(zip(reflowable, formatted_blocks))
        ):
            prefix = " " * indent
            replacement = [prefix + line if line else "" for line in block.splitlines()]
            lines[start:end] = replacement
        path.write_text("\n".join(lines) + "\n", encoding="utf-8")
