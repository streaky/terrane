"""Behavioral checks for selection-relative coverage attribution."""
import json
from html.parser import HTMLParser
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

if __package__:
    from .rust_coverage import ROOT, annotate_overlap, current_warning_files, llvm_tools, prepare_output, report
else:
    from rust_coverage import ROOT, annotate_overlap, current_warning_files, llvm_tools, prepare_output, report


class AttributionTests(unittest.TestCase):
    def attribute(self, footprints):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            records = []
            for index, functions in enumerate(footprints):
                path = directory / str(index)
                path.mkdir()
                (path / "functions.json").write_text(json.dumps(functions))
                records.append({"name": f"test-{index}", "path": str(index)})
            annotate_overlap(directory, records)
            return records

    def test_overlap_and_uniqueness_are_relative_to_all_selected_tests(self):
        records = self.attribute([
            ["shared-setup", "parser", "unique-a"],
            ["shared-setup", "parser", "lowering"],
            ["shared-setup", "lowering", "unique-c"],
        ])
        self.assertEqual([record["functions"] for record in records], [3, 3, 3])
        self.assertEqual([record["unique_functions"] for record in records], [1, 0, 1])
        self.assertEqual(records[0]["overlap"], "2 shared with test-1")
        self.assertEqual(records[2]["overlap"], "2 shared with test-1")

    def test_repeated_instantiations_do_not_inflate_function_footprints(self):
        records = self.attribute([["same-body", "same-body"], ["same-body"], []])
        self.assertEqual([record["functions"] for record in records], [1, 1, 0])
        self.assertEqual([record["unique_functions"] for record in records], [0, 0, 0])
        self.assertEqual(records[0]["overlap"], "1 shared with test-1")
        self.assertEqual(records[2]["overlap"], "No shared functions")


class OutputDirectoryTests(unittest.TestCase):
    def test_repeated_runs_clear_old_coverage_but_preserve_cache_and_unrelated_files(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary) / "coverage"
            prepare_output(directory)
            (directory / "aggregate/profiles").mkdir(parents=True)
            (directory / "aggregate/profiles/old.profraw").write_bytes(b"old counters")
            (directory / "tests").mkdir()
            (directory / "tests/old-test.json").write_text("{}")
            (directory / "index.html").write_text("old report")
            (directory / "build").mkdir()
            (directory / "build/cached-artifact").write_bytes(b"cached build")
            (directory / "unrelated.txt").write_text("keep")
            prepare_output(directory)
            self.assertFalse((directory / "aggregate").exists())
            self.assertFalse((directory / "tests").exists())
            self.assertFalse((directory / "index.html").exists())
            self.assertEqual((directory / "build/cached-artifact").read_bytes(), b"cached build")
            self.assertEqual((directory / "unrelated.txt").read_text(), "keep")

    def test_unowned_output_collision_is_not_deleted(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "tests").mkdir()
            source = directory / "tests/important.rs"
            source.write_text("user source")
            with self.assertRaisesRegex(ValueError, "unrelated contents"):
                prepare_output(directory)
            self.assertEqual(source.read_text(), "user source")


class ReportAvailabilityTests(unittest.TestCase):
    def test_warning_inventory_excludes_retained_builds_and_unrelated_runs(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            paths = [
                "aggregate/warnings.txt", "tests/current/warnings.txt",
                "build/nested/warnings.txt", "old-run/aggregate/warnings.txt",
                "unrelated/warnings.txt",
            ]
            for name in paths:
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("LLVM warning")
            self.assertEqual(
                current_warning_files(root),
                ["aggregate/warnings.txt", "tests/current/warnings.txt"],
            )

    def test_missing_profiles_are_unavailable_instead_of_zero_coverage(self):
        with tempfile.TemporaryDirectory() as temporary:
            self.assertIsNone(report(Path(temporary), [], []))

    def test_unavailable_test_does_not_change_available_test_uniqueness(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "covered").mkdir()
            (root / "covered/functions.json").write_text('["only-covered-here"]')
            records = [
                {"name": "covered", "path": "covered", "coverage_available": True},
                {"name": "crashed", "path": "crashed", "coverage_available": False, "functions": None},
            ]
            annotate_overlap(root, records)
            self.assertEqual(records[0]["unique_functions"], 1)
            self.assertIsNone(records[1]["functions"])
            self.assertNotIn("unique_functions", records[1])


class AbnormalExitTests(unittest.TestCase):
    def test_aborted_tests_retain_mixed_and_all_unavailable_reports(self):
        try:
            llvm_tools()
        except (ValueError, FileNotFoundError) as error:
            self.skipTest(str(error))
        with tempfile.TemporaryDirectory(prefix="terrane-coverage-abort-") as temporary:
            fixture = Path(temporary)
            manifest = fixture / "Cargo.toml"
            manifest.write_text(
                '[package]\nname = "coverage-abort-regression"\nversion = "0.0.0"\n'
                'edition = "2024"\n[workspace]\n[lib]\npath = "lib.rs"\n'
                '[dependencies]\nterrane-int-support = { path = '
                + json.dumps(str(ROOT / "crates/terrane-int-support")) + ' }\n'
            )
            (fixture / "lib.rs").write_text(
                '#[test]\nfn a_crash() { std::process::abort(); }\n'
                '#[test]\nfn b_healthy() {\n'
                ' assert_eq!(terrane_int_support::Int::from(42_i128).to_string(), "42");\n}\n'
            )
            output = fixture / "coverage"

            def run(*selection):
                result = subprocess.run(
                    [sys.executable, str(ROOT / "tools/coverage.py"), "--output", str(output),
                     "--per-test", *selection, "--", "--manifest-path", str(manifest), "--lib"],
                    cwd=ROOT, capture_output=True, text=True, timeout=300,
                )
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
                return json.loads((output / "run.json").read_text())

            mixed = run()
            crashed, healthy = mixed["tests"]
            self.assertEqual(crashed["returncode"], -6)
            self.assertFalse(crashed["coverage_available"])
            self.assertIsNone(crashed["functions"])
            self.assertEqual(healthy["status"], "passed")
            self.assertTrue(healthy["coverage_available"])
            self.assertEqual(healthy["unique_functions"], healthy["functions"])
            self.assertTrue(mixed["aggregate_available"])

            class Links(HTMLParser):
                def handle_starttag(self, tag, attributes):
                    if tag == "a":
                        for key, value in attributes:
                            if key == "href":
                                self.hrefs.append(value)

                def __init__(self):
                    super().__init__()
                    self.hrefs = []

            links = Links()
            links.feed((output / "index.html").read_text())
            for link in links.hrefs:
                self.assertTrue((output / link).is_file(), link)
            self.assertNotIn(crashed["path"] + "/html/index.html", links.hrefs)
            self.assertIn(crashed["log"], links.hrefs)

            all_unavailable = run("--filter", "a_crash")
            self.assertFalse(all_unavailable["aggregate_available"])
            self.assertEqual(all_unavailable["tests"][0]["returncode"], -6)
            self.assertIsNone(all_unavailable["tests"][0]["functions"])
            links = Links()
            links.feed((output / "index.html").read_text())
            self.assertNotIn("aggregate/html/index.html", links.hrefs)
            for link in links.hrefs:
                self.assertTrue((output / link).is_file(), link)


if __name__ == "__main__":
    unittest.main()
