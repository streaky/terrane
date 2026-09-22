#!/usr/bin/env python3

from __future__ import annotations

import importlib.util
from pathlib import Path
import unittest


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


if __name__ == "__main__":
    unittest.main()
