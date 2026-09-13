#!/usr/bin/env python3
"""Collapse repeated generated-Rust golden hunks across a Git range."""

from __future__ import annotations

import argparse
import collections
import json
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


def grouped_hunks(diff: str) -> collections.OrderedDict[Hunk, set[str]]:
    grouped: collections.OrderedDict[Hunk, set[str]] = collections.OrderedDict()
    path: str | None = None
    hunk: list[str] | None = None

    def finish_hunk() -> None:
        nonlocal hunk
        if path is not None and hunk:
            grouped.setdefault(tuple(hunk), set()).add(path)
        hunk = None

    for line in diff.splitlines():
        if line.startswith("diff --git "):
            finish_hunk()
            path = None
        elif line.startswith("+++ b/"):
            path = line.removeprefix("+++ b/")
        elif line.startswith("@@ "):
            finish_hunk()
            hunk = []
        elif hunk is not None:
            hunk.append(line)
    finish_hunk()
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
    parser = argparse.ArgumentParser(
        description="Print each unique lower.rs diff hunk once and count affected files."
    )
    parser.add_argument("git_range", help="range accepted by git diff, for example HEAD~1..HEAD")
    arguments = parser.parse_args()
    sys.stdout.write(summarize(grouped_hunks(golden_diff(arguments.git_range))))


if __name__ == "__main__":
    main()
