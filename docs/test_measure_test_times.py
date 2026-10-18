#!/usr/bin/env python3

from __future__ import annotations

import argparse
import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


SCRIPT = Path(__file__).with_name("measure-test-times.py")
SPEC = importlib.util.spec_from_file_location("measure_test_times", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
MEASURE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MEASURE)


def scoreboard_with_deleted_test() -> dict:
    scoreboard = MEASURE.base_scoreboard()
    scoreboard["tests"] = [
        {
            "id": "active",
            "target": "unit",
            "name": "active",
            "kind": "libtest",
            "seconds": 2.0,
            "status": "passed",
            "history": [],
        },
        {
            "id": "deleted",
            "target": "unit",
            "name": "deleted",
            "kind": "libtest",
            "seconds": 3.0,
            "status": "passed",
            "history": [],
        },
    ]
    return scoreboard


def active_measurement() -> list[dict]:
    return [
        {
            "id": "active",
            "target": "unit",
            "name": "active",
            "kind": "libtest",
            "seconds": 1.0,
            "status": "passed",
        }
    ]


class CompleteWorkspaceRunTests(unittest.TestCase):
    def test_only_unfiltered_complete_workspace_run_prunes_absent_tests(self) -> None:
        command = ["cargo", "test", "--workspace", "--", "--test-threads=8"]
        updated = MEASURE.update_scoreboard(
            scoreboard_with_deleted_test(), active_measurement(), command, 1.0, 0, {}
        )
        self.assertEqual([test["id"] for test in updated["tests"]], ["active"])

    def test_incomplete_workspace_runs_preserve_absent_tests(self) -> None:
        cases = [
            (["cargo", "test", "--workspace", "--exclude", "crate", "--"], 0, {}),
            (["cargo", "test", "--workspace", "--exclude=crate", "--"], 0, {}),
            (["cargo", "test", "--workspace", "--package=crate", "--"], 0, {}),
            (["cargo", "test", "--workspace", "--package", "crate", "--"], 0, {}),
            (["cargo", "test", "--workspace", "-pcrate", "--"], 0, {}),
            (["cargo", "test", "--workspace", "-p", "crate", "--"], 0, {}),
            (["cargo", "test", "--workspace", "--"], 1, {}),
            (
                ["cargo", "test", "--workspace", "--"],
                0,
                {"TERRANE_CONFORMANCE_FILTER": "projected-namespace-overlay"},
            ),
        ]
        for command, exit_code, environment in cases:
            with self.subTest(command=command, exit_code=exit_code, environment=environment):
                updated = MEASURE.update_scoreboard(
                    scoreboard_with_deleted_test(),
                    active_measurement(),
                    command,
                    1.0,
                    exit_code,
                    environment,
                )
                self.assertEqual(
                    [test["id"] for test in updated["tests"]], ["active", "deleted"]
                )


class NameLimitPublicationTests(unittest.TestCase):
    def test_name_limit_preserves_scoreboard_on_rejection(self) -> None:
        for length in (80, 81):
            with self.subTest(length=length), tempfile.TemporaryDirectory() as directory:
                output = Path(directory) / "scoreboard.yaml"
                original = MEASURE.yaml.safe_dump(MEASURE.base_scoreboard())
                output.write_text(original, encoding="utf-8")
                measured = active_measurement()
                measured[0]["name"] = "x" * length
                measured[0]["id"] = "unit::" + measured[0]["name"]
                options = argparse.Namespace(output=output, cargo_args=[], no_html=True)
                with (
                    patch.object(MEASURE, "parse_args", return_value=options),
                    patch.object(
                        MEASURE,
                        "run_tests",
                        return_value=(0, 1.0, measured, ["cargo", "test", "--workspace"]),
                    ),
                    patch.object(MEASURE, "command_text", return_value="fixture"),
                ):
                    result = MEASURE.main()
                if length == 80:
                    self.assertEqual(result, 0)
                    published = MEASURE.yaml.safe_load(output.read_text(encoding="utf-8"))
                    self.assertEqual(published["tests"][0]["name"], "x" * 80)
                else:
                    self.assertEqual(result, 2)
                    self.assertEqual(output.read_text(encoding="utf-8"), original)


if __name__ == "__main__":
    unittest.main()
