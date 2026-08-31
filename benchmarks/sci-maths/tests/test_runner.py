import copy
import importlib.util
import os
from pathlib import Path
import sys
import tempfile
import time
import unittest
from unittest import mock


RUNNER_PATH = Path(__file__).resolve().parents[1] / "run.py"
SPEC = importlib.util.spec_from_file_location("sci_maths_runner", RUNNER_PATH)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = runner
SPEC.loader.exec_module(runner)


def lane(
    lane_id: str,
    command: tuple[str, ...],
    implementation: str = "implementation.py",
) -> runner.Lane:
    return runner.Lane(
        lane_id=lane_id,
        name=lane_id,
        config_path=Path("fixture.toml"),
        implementation=implementation,
        setup=(),
        prepare=(),
        lower=(),
        lower_output=None,
        run=command,
        prepare_output="none",
        cache_paths=(),
        metadata={},
        environment_commands=(),
    )


class RunnerContracts(unittest.TestCase):
    def test_float_contract_accepts_either_absolute_or_relative_tolerance(self) -> None:
        self.assertTrue(runner.result_matches("float", 100.2, 100.0, 0.25, 0.0))
        self.assertTrue(runner.result_matches("float", 10_001.0, 10_000.0, 0.0, 0.0001))
        self.assertFalse(runner.result_matches("float", 100.3, 100.0, 0.25, 0.001))

    def test_templates_allow_literal_braces_and_reject_unknown_placeholders(self) -> None:
        expanded = runner.expand(("$implementation", "lambda {value: 1}"), {"implementation": "main.py"})
        self.assertEqual(expanded, ["main.py", "lambda {value: 1}"])
        with self.assertRaisesRegex(runner.BenchmarkError, "unknown command placeholder"):
            runner.expand(("$missing",), {})

    def test_suite_validation_reports_missing_and_malformed_contracts(self) -> None:
        valid = {
            "format": 2,
            "name": "fixture",
            "measurement": {
                "warmups": 0,
                "runs": 1,
                "setup-timeout-seconds": 5,
                "runtime-timeout-seconds": 2.5,
            },
            "lanes": [{"path": "lanes/fixture.toml"}],
            "groups": [
                {
                    "id": "baseline",
                    "name": "Baseline",
                    "lanes": ["fixture"],
                    "problems": [{"path": "problems/fixture"}],
                }
            ],
        }
        runner.validate_suite(valid)

        missing_timeout = copy.deepcopy(valid)
        del missing_timeout["measurement"]["runtime-timeout-seconds"]
        with self.assertRaisesRegex(
            runner.BenchmarkError, "measurement.runtime-timeout-seconds"
        ):
            runner.validate_suite(missing_timeout)

        malformed_path = copy.deepcopy(valid)
        malformed_path["lanes"] = [{"path": "../outside.toml"}]
        with self.assertRaisesRegex(runner.BenchmarkError, "path is outside"):
            runner.validate_suite(malformed_path)


    def test_fixture_lane_exercises_setup_prepare_run_and_environment(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            problem_path = Path(directory)
            implementation = problem_path / "implementation.py"
            implementation.write_text("import sys\nprint(sys.argv[1])\n")
            config_path = problem_path / "lane.toml"
            config_path.write_text(
                "\n".join(
                    [
                        'id = "fixture"',
                        'name = "Fixture lane"',
                        'implementation = "implementation.py"',
                        f'setup = ["{sys.executable}", "--version"]',
                        f'prepare = ["{sys.executable}", "-c", "import sys; print(sys.argv[1])", "$implementation"]',
                        'prepare-output = "executable-path"',
                        f'run = ["{sys.executable}", "$prepared"]',
                        f'environment = [["{sys.executable}", "--version"]]',
                    ]
                )
            )
            fixture_lane = runner.load_lane(config_path)
            problem = {
                "id": "fixture",
                "path": problem_path,
                "result": "integer",
                "profiles": {"correctness": {"size": 41, "expected": 41}},
            }
            setup = runner.setup_lane(fixture_lane, 5.0)
            prepared, preparation = runner.prepare_implementation(problem, fixture_lane, 5.0)
            actual, execution = runner.execute(
                problem, fixture_lane, prepared, "correctness", 5.0
            )
            environment = runner.capture_lane_environment(fixture_lane, 5.0)
            self.assertIsNotNone(setup)
            self.assertIsNotNone(preparation)
            self.assertEqual(Path(prepared), implementation)
            self.assertEqual(actual, 41)
            self.assertEqual(execution.returncode, 0)
            self.assertIn("Python", environment[0]["stdout"])

    def test_process_result_retains_stderr_and_records_peak_memory_when_available(self) -> None:
        result = runner.run_process(
            [sys.executable, "-c", "import sys; print('warning: fixture', file=sys.stderr); print(7)"],
            cwd=Path.cwd(),
            timeout=5.0,
        )
        self.assertEqual(result.stdout.strip(), "7")
        self.assertIn("warning: fixture", result.stderr)
        if runner.memory_measurement_available():
            self.assertIsNotNone(result.peak_memory_bytes)
            assert result.peak_memory_bytes is not None
            self.assertGreater(result.peak_memory_bytes, 0)
        else:
            self.assertIsNone(result.peak_memory_bytes)
        record = runner.process_record(result)
        assert record is not None
        self.assertEqual(record["warning_lines"], ["warning: fixture"])

    def test_processes_force_available_sccache_when_inherited_wrapper_is_disabled(
        self,
    ) -> None:
        with (
            mock.patch.dict(os.environ, {"RUSTC_WRAPPER": ""}),
            mock.patch.object(runner, "available_sccache", return_value="/opt/cache/sccache"),
        ):
            result = runner.run_process(
                [
                    sys.executable,
                    "-c",
                    "import os; print(os.environ.get('RUSTC_WRAPPER', 'missing'))",
                ],
                cwd=Path.cwd(),
                timeout=5.0,
            )
        self.assertEqual(result.stdout.strip(), "/opt/cache/sccache")

    def test_process_environment_does_not_require_unavailable_sccache(self) -> None:
        with (
            mock.patch.dict(os.environ, {}, clear=True),
            mock.patch.object(runner, "available_sccache", return_value=None),
        ):
            environment = runner.process_environment()
        self.assertNotIn("RUSTC_WRAPPER", environment)

    def test_sccache_server_starts_before_measured_process_groups(self) -> None:
        probe = runner.subprocess.CompletedProcess([], 1, "", "not running")
        started = runner.subprocess.CompletedProcess([], 0, "", "")
        with (
            mock.patch.object(runner, "available_sccache", return_value="/opt/cache/sccache"),
            mock.patch.object(runner.subprocess, "run", side_effect=[probe, started]) as run,
        ):
            runner.prepare_sccache_server()
        self.assertEqual(run.call_args_list[0].args[0], ["/opt/cache/sccache", "--show-stats"])
        self.assertEqual(run.call_args_list[1].args[0], ["/opt/cache/sccache", "--start-server"])

    def test_cgroup_cleanup_error_does_not_mask_command_failure(self) -> None:
        class FailingCleanupGroup:
            path = Path("/sys/fs/cgroup/terrane-sci-fixture")

            def join_from_child(self) -> None:
                pass

            def peak_bytes(self) -> int:
                return 1024

            def populated(self) -> bool:
                return False

            def kill(self) -> None:
                pass

            def remove(self) -> None:
                raise OSError("fixture cleanup failure")

        with mock.patch.object(
            runner, "create_memory_cgroup", return_value=FailingCleanupGroup()
        ):
            result = runner.run_process(
                [sys.executable, "-c", "import sys; sys.stderr.write('primary\\n'); sys.exit(7)"],
                cwd=Path.cwd(),
                timeout=5.0,
            )
        with self.assertRaisesRegex(runner.BenchmarkError, r"command failed \(7\)") as raised:
            runner.require_success(result, [sys.executable])
        self.assertIn("primary", str(raised.exception))
        self.assertIn("fixture cleanup failure", str(raised.exception))


    def test_cgroup_memory_measurement_distinguishes_low_and_high_allocations(self) -> None:
        if not runner.memory_measurement_available():
            self.skipTest("delegated cgroup-v2 memory accounting is unavailable")
        low = runner.run_process(
            [sys.executable, "-c", "print(0)"],
            cwd=Path.cwd(),
            timeout=5.0,
        )
        high = runner.run_process(
            [
                sys.executable,
                "-c",
                "data = bytearray(32 * 1024 * 1024); "
                "data[::4096] = b'x' * len(data[::4096]); "
                "print(len(data))",
            ],
            cwd=Path.cwd(),
            timeout=5.0,
        )
        assert low.peak_memory_bytes is not None
        assert high.peak_memory_bytes is not None
        self.assertGreater(high.peak_memory_bytes, low.peak_memory_bytes + 24 * 1024 * 1024)

    def test_timeout_kills_the_whole_process_group(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            group_file = Path(directory) / "group"
            script = (
                "import os, pathlib, subprocess, sys, time; "
                "subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(60)']); "
                "pathlib.Path(sys.argv[1]).write_text(str(os.getpgrp())); "
                "time.sleep(60)"
            )
            with self.assertRaisesRegex(runner.BenchmarkError, "timed out"):
                runner.run_process(
                    [sys.executable, "-c", script, str(group_file)],
                    cwd=Path.cwd(),
                    timeout=0.2,
                )
            process_group = int(group_file.read_text())
            deadline = time.monotonic() + 2.0
            while True:
                try:
                    os.killpg(process_group, 0)
                except ProcessLookupError:
                    break
                if time.monotonic() >= deadline:
                    self.fail("timed-out process group still exists")
                time.sleep(0.01)

    def test_execute_passes_manifest_size_as_the_only_workload_argument(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            problem_path = Path(directory)
            implementation = problem_path / "implementation.py"
            implementation.write_text("import sys\nprint(sys.argv[1] if len(sys.argv) == 2 else -1)\n")
            problem = {
                "id": "fixture",
                "path": problem_path,
                "result": "integer",
                "profiles": {
                    "performance": {"size": 37, "expected": 37},
                },
            }
            actual, _ = runner.execute(
                problem,
                lane("fixture", (sys.executable, "$implementation")),
                "",
                "performance",
                5.0,
            )
            self.assertEqual(actual, 37)

    def test_benchmark_interleaves_cases_within_each_run_index(self) -> None:
        lanes = [lane("first", (), str(RUNNER_PATH)), lane("second", (), str(RUNNER_PATH))]
        problems = [
            {
                "id": problem_id,
                "title": problem_id,
                "dataset": "fixture data",
                "path": Path.cwd(),
                "group": "fixture-group",
                "group_name": "Fixture group",
                "lane_ids": lane_ids,
                "result": "integer",
                "profiles": {
                    "correctness": {"size": 1, "expected": 1},
                    "performance": {"size": 1, "expected": 1},
                },
            }
            for problem_id, lane_ids in (
                ("alpha", ("first", "second")),
                ("beta", ("second",)),
            )
        ]
        calls: list[tuple[str, str, str]] = []
        original_execute = runner.execute

        def fake_execute(problem, selected_lane, prepared, profile_name, timeout):
            calls.append((profile_name, problem["id"], selected_lane.lane_id))
            return 1, runner.ProcessResult(0, "1\n", "", 0.01, 1024)

        runner.execute = fake_execute
        try:
            runner.benchmark(
                {
                    "name": "fixture",
                    "groups": [{"id": "fixture-group", "name": "Fixture group"}],
                },
                problems,
                lanes,
                setup_timeout=5.0,
                runtime_timeout=5.0,
                runs=2,
                warmups=1,
                cold_builds=False,
            )
        finally:
            runner.execute = original_execute

        expected_order = [
            ("performance", "alpha", "first"),
            ("performance", "alpha", "second"),
            ("performance", "beta", "second"),
        ]
        performance_calls = [call for call in calls if call[0] == "performance"]
        self.assertEqual(performance_calls, expected_order * 3)


if __name__ == "__main__":
    unittest.main()
