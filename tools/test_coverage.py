"""Behavioral checks for selection-relative coverage attribution."""
import json
from pathlib import Path
import tempfile
import unittest

from coverage import annotate_overlap, prepare_output


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


if __name__ == "__main__":
    unittest.main()
