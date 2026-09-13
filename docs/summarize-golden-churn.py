#!/usr/bin/env python3
"""Collapse repeated generated-Rust golden hunks across a Git range."""

from __future__ import annotations

import argparse
import collections
import json
import signal
import subprocess
import sys


Hunk = tuple[str, ...]


def golden_diff(git_range: str) -> str:
    command = [
        "git",
        "diff",
        "--no-ext-diff",
        "--no-color",
        "--unified=3",
        git_range,
        "--",
        "**/lower.rs",
    ]
    result = subprocess.run(command, check=False, text=True, capture_output=True)
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    return result.stdout


def normalized_changed_lines(hunk: Hunk, prefix: str) -> list[str]:
    return [
        line[1:].strip().removesuffix(",")
        for line in hunk
        if line.startswith(prefix)
    ]


def without_one_scope(lines: list[str]) -> list[list[str]]:
    return [
        lines[:opening] + lines[opening + 1 : closing] + lines[closing + 1 :]
        for opening, line in enumerate(lines)
        if line == "{"
        for closing in range(opening + 1, len(lines))
        if lines[closing] == "}"
    ]


def scope_wrapper_hunk(hunk: Hunk) -> Hunk | None:
    """Recognize one balanced wrapper without overlooking reordered contents."""
    removed = normalized_changed_lines(hunk, "-")
    added = normalized_changed_lines(hunk, "+")
    if removed and any(candidate == removed for candidate in without_one_scope(added)):
        return ("+{", "+}")
    if added and any(candidate == added for candidate in without_one_scope(removed)):
        return ("-{", "-}")
    return None


def grouped_hunks(diff: str) -> collections.OrderedDict[Hunk, set[str]]:
    file_hunks: collections.OrderedDict[str, list[Hunk]] = collections.OrderedDict()
    path: str | None = None
    hunk: list[str] | None = None

    def finish_hunk() -> None:
        nonlocal hunk
        if path is not None and hunk:
            file_hunks.setdefault(path, []).append(tuple(hunk))
        hunk = None

    for line in diff.splitlines():
        if line.startswith("diff --git "):
            finish_hunk()
            path = None
        elif line.startswith("--- a/") and path is None:
            path = line.removeprefix("--- a/")
        elif line.startswith("+++ b/"):
            path = line.removeprefix("+++ b/")
        elif line.startswith("@@ "):
            finish_hunk()
            hunk = []
        elif hunk is not None:
            hunk.append(line)
    finish_hunk()

    grouped: collections.OrderedDict[Hunk, set[str]] = collections.OrderedDict()
    for file_path, hunks in file_hunks.items():
        for literal_hunk in hunks:
            changed_lines = tuple(
                line for line in literal_hunk if line.startswith(("+", "-"))
            )
            signature = scope_wrapper_hunk(changed_lines) or changed_lines
            grouped.setdefault(signature, set()).add(file_path)
    return grouped


def summarize(grouped: collections.OrderedDict[Hunk, set[str]]) -> str:
    changed_files = set().union(*grouped.values()) if grouped else set()
    lines = [
        "summary:",
        f"  unique-hunks: {len(grouped)}",
        f"  changed-files: {len(changed_files)}",
    ]
    if not grouped:
        lines.append("hunks: []")
        return "\n".join(lines) + "\n"

    lines.append("hunks:")
    for hunk, paths in grouped.items():
        lines.append(f"  - file-count: {len(paths)}")
        lines.append("    files:")
        lines.extend(f"      - {json.dumps(path)}" for path in sorted(paths))
        lines.append("    diff: |-")
        lines.extend(f"      {line}" for line in hunk)
    return "\n".join(lines) + "\n"


def main() -> None:
    signal.signal(signal.SIGPIPE, signal.SIG_DFL)
    parser = argparse.ArgumentParser(
        description="Print each unique lower.rs diff hunk once and count affected files."
    )
    parser.add_argument("git_range", help="range accepted by git diff, for example HEAD~1..HEAD")
    arguments = parser.parse_args()
    sys.stdout.write(summarize(grouped_hunks(golden_diff(arguments.git_range))))


if __name__ == "__main__":
    main()
