"""Focused contracts for sample-profile flag and snapshot handling."""
import hashlib
import contextlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
from types import SimpleNamespace

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

    def test_failed_build_status_is_returned(self):
        with patch.dict(os.environ, {"RUSTFLAGS": ""}, clear=True), \
                patch("subprocess.run", return_value=type("Result", (), {"returncode": 23})()):
            self.assertEqual(profiles.prepare(["failing-build"]), 23)


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

            with patch.dict(os.environ, {"RUSTFLAGS": ""}, clear=True), \
                    patch.object(profiles, "rustc_profdata_tool", return_value="profdata"), \
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


class CargoFlagTests(unittest.TestCase):
    @unittest.skipUnless(shutil.which("cargo") and shutil.which("rustc"), "Rust tools unavailable")
    def test_config_flags_require_an_explicit_baseline(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / ".cargo").mkdir()
            (root / "src").mkdir()
            (root / "Cargo.toml").write_text(
                '[workspace]\n[package]\nname="sample-flags-probe"\nversion="0.0.0"\nedition="2024"\n'
            )
            (root / ".cargo/config.toml").write_text(
                '[build]\nrustflags=["--cfg", "configured", "--check-cfg", "cfg(configured)"]\n'
            )
            (root / "src/main.rs").write_text(
                'fn main() { println!("{}", cfg!(configured)); }\n'
            )
            env = os.environ.copy()
            for name in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS"):
                env.pop(name, None)
            env["CARGO_HOME"] = str(root / "cargo-home")
            env["CARGO_TARGET_DIR"] = str(root / "target")
            command = ["cargo", "run", "--offline", "--quiet"]
            ordinary = subprocess.run(command, cwd=root, env=env, check=True, capture_output=True)
            self.assertEqual(ordinary.stdout, b"true\n")
            wrapped = [sys.executable, str(profiles.ROOT / "tools/sample-profile.py"),
                       "prepare", "--", *command]
            refused = subprocess.run(wrapped, cwd=root, env=env, capture_output=True)
            self.assertEqual(refused.returncode, 1)
            self.assertIn(b"effective baseline", refused.stderr)
            env["RUSTFLAGS"] = "--cfg configured --check-cfg cfg(configured)"
            prepared = subprocess.run(wrapped, cwd=root, env=env, check=True, capture_output=True)
            self.assertEqual(prepared.stdout, ordinary.stdout)


class RecordTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.binary = self.root / "program"
        self.binary.write_bytes(b"fixed executable")
        self.output = self.root / "samples.prof"
        self.sidecar = self.root / "samples.prof.json"
        self.raw = self.root / "samples.prof.perf.data"
        self.status = 0
        self.write_capture = True
        self.profile_bytes = b"function:7:0\n 1: 7\n"
        self.enterContext(patch.object(
            profiles, "tool_path", side_effect=lambda name, explicit=None: explicit or name
        ))
        self.enterContext(patch.object(profiles, "rustc_profdata_tool", return_value="profdata"))
        self.enterContext(patch.object(profiles, "tool_version", return_value="test tool version"))
        self.enterContext(patch.object(profiles.subprocess, "run", side_effect=self.simulate_tools))

    def simulate_tools(self, command, **kwargs):
        command = [str(arg) for arg in command]
        if command[0] == "perf":
            if self.write_capture:
                Path(command[command.index("-o") + 1]).write_bytes(b"fresh capture")
            return SimpleNamespace(returncode=self.status)
        if command[0] == "converter":
            Path(command[command.index("--out") + 1]).write_bytes(self.profile_bytes)
            return SimpleNamespace(returncode=0)
        if command[0] == "profdata":
            return SimpleNamespace(returncode=0, stdout=Path(command[-1]).read_text())
        raise AssertionError(f"Unexpected command: {command}")

    def record(self, *, allow=False):
        profiles.record(self.binary, self.output, ["private-workload-argument"], "autofdo",
                        "converter", True, 999, allow)

    def old_artifacts(self):
        self.output.write_bytes(b"previous profile")
        self.sidecar.write_bytes(b"previous provenance")
        self.raw.write_bytes(b"previous capture")

    def test_success_publishes_matching_private_provenance(self):
        self.record()
        metadata = json.loads(self.sidecar.read_text())
        self.assertEqual(self.output.read_bytes(), self.profile_bytes)
        self.assertEqual(metadata["profile_sha256"], hashlib.sha256(self.profile_bytes).hexdigest())
        self.assertEqual(metadata["binary_sha256"], hashlib.sha256(self.binary.read_bytes()).hexdigest())
        self.assertEqual(metadata["raw_perf_sha256"], hashlib.sha256(self.raw.read_bytes()).hexdigest())
        self.assertEqual(metadata["capture"]["exit_status"], 0)
        self.assertEqual(metadata["tools"]["profile-validation"]["version"], "test tool version")
        self.assertNotIn("private-workload-argument", self.sidecar.read_text())

    def test_nonzero_training_requires_explicit_permission(self):
        self.old_artifacts()
        self.status = 3
        with self.assertRaises(subprocess.CalledProcessError) as error:
            self.record()
        self.assertEqual(error.exception.returncode, 3)
        self.assertEqual(self.output.read_bytes(), b"previous profile")
        self.assertEqual(self.sidecar.read_bytes(), b"previous provenance")
        self.assertEqual(self.raw.read_bytes(), b"previous capture")

    def test_cli_permission_records_nonzero_status(self):
        for code in (3, 137):
            with self.subTest(perf_status=code):
                self.status = code
                status = profiles.cli([
                    "record", "--binary", str(self.binary), "--output", str(self.output),
                    "--converter", "autofdo", "--converter-path", "converter", "--no-lbr",
                    "--allow-workload-failure", "--", "private-workload-argument",
                ])
                self.assertEqual(status, 0)
                self.assertEqual(json.loads(self.sidecar.read_text())["capture"]["exit_status"], code)
                self.assertNotIn("private-workload-argument", self.sidecar.read_text())

    def test_permission_cannot_reuse_a_stale_capture(self):
        self.old_artifacts()
        self.status = 1
        self.write_capture = False
        with self.assertRaisesRegex(ValueError, "fresh nonempty capture"):
            self.record(allow=True)
        self.assertEqual(self.output.read_bytes(), b"previous profile")
        self.assertEqual(self.sidecar.read_bytes(), b"previous provenance")

    def test_perf_signal_termination_is_rejected(self):
        self.status = -15
        with self.assertRaises(subprocess.CalledProcessError) as error:
            self.record(allow=True)
        self.assertEqual(error.exception.returncode, -15)
        self.assertFalse(self.output.exists())

    def test_rejected_conversion_preserves_previous_artifacts(self):
        self.old_artifacts()
        self.profile_bytes = b"function:1174:0\n 1: 0\n"
        with self.assertRaisesRegex(ValueError, "no positive source-location"):
            self.record()
        self.assertEqual(self.output.read_bytes(), b"previous profile")
        self.assertEqual(self.sidecar.read_bytes(), b"previous provenance")
        self.assertEqual(self.raw.read_bytes(), b"previous capture")

    def test_metadata_staging_failure_preserves_previous_artifacts(self):
        self.old_artifacts()
        with patch.object(Path, "write_text", side_effect=OSError("metadata storage full")):
            with self.assertRaisesRegex(OSError, "metadata storage full"):
                self.record()
        self.assertEqual(self.output.read_bytes(), b"previous profile")
        self.assertEqual(self.sidecar.read_bytes(), b"previous provenance")

    def test_publication_failure_cannot_leave_stale_provenance(self):
        self.old_artifacts()
        replace = Path.replace

        def fail_sidecar(path, target):
            if path.name == "provenance.json":
                raise OSError("sidecar publication interrupted")
            return replace(path, target)

        with patch.object(Path, "replace", new=fail_sidecar):
            with self.assertRaisesRegex(OSError, "sidecar publication interrupted"):
                self.record()
        self.assertEqual(self.output.read_bytes(), self.profile_bytes)
        self.assertFalse(self.sidecar.exists())

    def test_no_lbr_rejects_the_llvm_converter(self):
        with contextlib.redirect_stderr(io.StringIO()) as stderr:
            status = profiles.cli([
                "record", "--binary", str(self.binary), "--output", str(self.output), "--no-lbr",
            ])
        self.assertEqual(status, 1)
        self.assertIn("--no-lbr requires --converter autofdo", stderr.getvalue())
        self.assertFalse(self.output.exists())


if __name__ == "__main__":
    unittest.main()
