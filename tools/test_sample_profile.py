"""Focused contracts for sample-profile flag and snapshot handling."""
import hashlib
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

if __package__:
    from . import sample_profile as profiles
else:
    import sample_profile as profiles


class FlagTests(unittest.TestCase):
    def test_encoded_flags_take_precedence_and_profile_path_stays_one_argument(self):
        with tempfile.TemporaryDirectory() as temporary:
            cache = Path(temporary) / "cache with spaces"
            snapshot = Path(temporary) / "profile with spaces.prof"
            snapshot.write_bytes(b"profile")
            env = {"RUSTFLAGS": "-C opt-level=1", "CARGO_ENCODED_RUSTFLAGS": "--cfg\x1fexisting"}
            with patch.dict(os.environ, env, clear=True), \
                    patch.object(profiles, "rustc_profdata_tool", return_value="profdata"), \
                    patch.object(profiles, "validate_profile"), \
                    patch("subprocess.run", return_value=type("Result", (), {"returncode": 0})()) as run:
                profiles.use(snapshot, ["cargo", "build"], cache)
            child_env = run.call_args.kwargs["env"]
            flags = child_env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")
            self.assertEqual(flags[:2], ["--cfg", "existing"])
            self.assertNotIn("RUSTFLAGS", child_env)
            self.assertEqual(flags[-1], f"profile-sample-use={cache.resolve() / (hashlib.sha256(b'profile').hexdigest() + '.prof')}")

    def test_prepare_preserves_encoded_flags_not_rustflags(self):
        with patch.dict(os.environ, {"RUSTFLAGS": "--cfg from_rustflags",
                                     "CARGO_ENCODED_RUSTFLAGS": "--cfg\x1ffrom_encoded"}, clear=True), \
                patch("subprocess.run", return_value=type("Result", (), {"returncode": 0})()) as run:
            self.assertEqual(profiles.prepare(["cargo", "build"]), 0)
        flags = run.call_args.kwargs["env"]["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")
        self.assertEqual(flags[:2], ["--cfg", "from_encoded"])
        self.assertNotIn("from_rustflags", flags)
        self.assertIn("strip=none", flags)

    def test_path_text_does_not_count_as_a_codegen_profile_flag(self):
        env = {"RUSTFLAGS": "--remap-path-prefix=/tmp/profile-use=/src"}
        profiles.augment_flags(env, ["-C", "strip=none"])
        self.assertIn("--remap-path-prefix=/tmp/profile-use=/src",
                      env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f"))

    def test_conflicting_instrumentation_flags_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "Conflicting"):
            profiles.augment_flags({"CARGO_ENCODED_RUSTFLAGS": "-C\x1fprofile-generate"}, ["-C", "debuginfo=line-tables-only"])


class SnapshotTests(unittest.TestCase):
    def test_changed_bytes_at_same_input_path_select_different_snapshot(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            profile = root / "input.prof"
            cache = root / "cache"
            observed = []

            def build(command, env):
                observed.append(env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")[-1])
                return type("Result", (), {"returncode": 0})()

            with patch.object(profiles, "rustc_profdata_tool", return_value="profdata"), \
                    patch.object(profiles, "validate_profile"), patch("subprocess.run", side_effect=build):
                profile.write_bytes(b"first")
                profiles.use(profile, ["cargo", "build"], cache)
                profile.write_bytes(b"second")
                profiles.use(profile, ["cargo", "build"], cache)
            self.assertNotEqual(observed[0], observed[1])
            self.assertTrue((cache / (hashlib.sha256(b"first").hexdigest() + ".prof")).is_file())
            self.assertTrue((cache / (hashlib.sha256(b"second").hexdigest() + ".prof")).is_file())

    def test_corrupt_digest_named_snapshot_is_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            profile = root / "input.prof"
            profile.write_bytes(b"correct")
            digest = hashlib.sha256(b"correct").hexdigest()
            cache = root / "cache"
            cache.mkdir()
            (cache / f"{digest}.prof").write_bytes(b"corrupt")
            with patch.object(profiles, "rustc_profdata_tool", return_value="profdata"), \
                    patch.object(profiles, "validate_profile"):
                with self.assertRaisesRegex(ValueError, "Corrupt immutable"):
                    profiles.use(profile, ["cargo", "build"], cache)


    def test_profiles_require_positive_location_counts_not_just_function_totals(self):
        with tempfile.TemporaryDirectory() as temporary:
            profile = Path(temporary) / "samples.prof"
            profile.write_bytes(b"converted profile")
            for text in ("", "function:0:0\n", "function:1174:0\n 1: 0\n"):
                with self.subTest(text=text), \
                        patch.object(profiles, "run", return_value=type("Result", (), {"stdout": text})()):
                    with self.assertRaisesRegex(ValueError, "no positive source-location"):
                        profiles.validate_profile(profile, "profdata")
            with patch.object(profiles, "run", return_value=type("Result", (), {
                    "stdout": "function:7:0\n 1: 0\n 2.3: 7\n"})()):
                profiles.validate_profile(profile, "profdata")

    def test_rejected_conversion_preserves_previous_profile(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            binary = root / "program"
            binary.write_bytes(b"fixed executable")
            profile = root / "samples.prof"
            profile.write_bytes(b"previous profile")

            def convert(command, **kwargs):
                if command[0] == "converter":
                    Path(command[command.index("--out") + 1]).write_bytes(b"invalid profile")

            with patch.object(profiles, "tool_path", side_effect=lambda name, explicit=None: explicit or name), \
                    patch.object(profiles, "rustc_profdata_tool", return_value="profdata"), \
                    patch.object(profiles, "run", side_effect=convert), \
                    patch.object(profiles, "validate_profile", side_effect=ValueError("invalid profile")):
                with self.assertRaisesRegex(ValueError, "invalid profile"):
                    profiles.record(binary, profile, [], "autofdo", "converter", True, 999)
            self.assertEqual(profile.read_bytes(), b"previous profile")

    def test_failed_build_status_is_returned(self):
        with patch.dict(os.environ, {}, clear=True), \
                patch("subprocess.run", return_value=type("Result", (), {"returncode": 23})()):
            self.assertEqual(profiles.prepare(["failing-build"]), 23)


if __name__ == "__main__":
    unittest.main()
