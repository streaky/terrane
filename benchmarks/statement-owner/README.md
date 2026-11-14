# Statement-local shared-owner benchmark

## Finding

The canonical-IR owner-loan change improves these eligible shared-field-write loops. This compares actual compilers from `9f857a0a` (before) and `ec074bd6` (after), not hand-edited generated Rust or the discarded allocator prototype.

| Workload | Before median | After median | Elapsed-time change | Change across cohort medians |
|---|---:|---:|---:|---:|
| Local alias, borrowed statement owner | 237.752 ms | 226.518 ms | -4.73% | -1.85% to -4.79% |
| Shared parameter, borrowed statement owner | 235.850 ms | 229.995 ms | -2.48% | -2.44% to -2.55% |
| Effectful RHS, unchanged clone fallback | 235.949 ms | 236.161 ms | +0.09% | -0.05% to +0.19% |

There are 63 measured executions per compiler/workload: three cohorts of 21 interleaved runs, each preceded by three warmups per case. Two cohorts ran before/after; the final cohort ran after/before. All correctness and performance-result checks passed. Performance executions perform 2,000,000 iterations and print `7999995`; the small correctness profile performs 10 and prints `34`.

The evidence supports keeping this optimization: both eligible shapes improve, while the fallback has no meaningful measured change. Local-alias improvement varied substantially in the first normalized cohort, so the pooled median is not a universal 4.73% promise. These are deliberately write-heavy microbenchmarks, not application-wide results or confidence intervals.

## What changed

Each program owns one long-lived shared counter and repeatedly writes a value cycling through 1–7, reads it and accumulates a checksum.

- **Local:** an explicit source alias remains. Only the compiler-created statement owner changes from `alias.clone()` to `&alias`.
- **Parameter:** the helper's statement owner changes from `owner.clone()` to `&owner`. The source-level call and its argument handling remain unchanged.
- **Fallback:** the RHS calls a helper, so the new proof does not apply. Generated Rust and executable bytes are identical before/after.

The inspected generated programs have only those single-line changes; their support modules are byte-identical. Both read/write locks, RHS ordering and source aliases remain in place. The optimization removes temporary strong-reference count work; this experiment does not claim fewer allocations, less retained memory, closure unboxing or lock elision.

## Method and controls

- Stable Rust **1.100.0**, optimization level 3, fat LTO, one codegen unit, warnings denied. No nightly feature or `RUSTC_BOOTSTRAP`.
- Both compilers emit unchanged Rust through `terrane rust --require-canonical-rust`. Preparation copies the emission verbatim into the same per-problem crate identity and source path, using the same support dependencies and native Cargo target directory for both versions.
- Runs were pinned to the same allowed CPU. These are elapsed-time measurements of single-threaded CPU-bound work, not hardware-cycle measurements. Cgroup memory measurement was unavailable; no memory result is reported.
- The launcher uses `exec` with a fixed `argv[0]`. Executable path names have equal lengths. Byte-identical executables are hard-linked, so the fallback also shares its inode and executable pages.
- An initial unnormalized cohort is **excluded**: its byte-identical fallback moved by about 7%, invalidating attribution. Normalizing launch conditions restored control stability. The experiment did not establish which individual launch difference caused that initial bias.
- Counterbalancing changes the suite's `[[lanes]]` declaration order. The group's `lanes` list selects membership; reversing only that list does not reverse execution.

[results.json](results.json) preserves individual timing samples, inclusion/exclusion decisions, actual case orders, observed results, environment, compiler revisions and executable hashes, source fingerprints, and emitted-Rust hashes/diffs. The misleadingly named local scratch file `reversed-timings.json` was actually another before/after cohort; its portable record is correctly named `normalized-repeat`. The genuinely reversed cohort is `normalized-counterbalanced`.

The previous hand-edited Arc/pool experiment's percentages are not substituted for this real compiler comparison. Allocator-aware Arc remains deferred.

## Reproduce

Build and freeze both compiler revisions independently. For example, from the repository root:

```sh
mkdir -p target/statement-owner-reproduce
git worktree add --detach target/statement-owner-reproduce/before 9f857a0a
git worktree add --detach target/statement-owner-reproduce/after ec074bd6
cargo +1.100.0 build -p terrane-cli \
  --manifest-path target/statement-owner-reproduce/before/Cargo.toml \
  --target-dir target/statement-owner-reproduce/build-before
cargo +1.100.0 build -p terrane-cli \
  --manifest-path target/statement-owner-reproduce/after/Cargo.toml \
  --target-dir target/statement-owner-reproduce/build-after
export TERRANE_BENCH_BEFORE="$PWD/target/statement-owner-reproduce/build-before/debug/terrane"
export TERRANE_BENCH_AFTER="$PWD/target/statement-owner-reproduce/build-after/debug/terrane"
python benchmarks/statement-owner/run.py check
```

Then run the interleaved benchmark pinned to one permitted CPU:

```sh
python - <<'PY'
import os
import subprocess
os.sched_setaffinity(0, {min(os.sched_getaffinity(0))})
subprocess.run([
    'python', 'benchmarks/statement-owner/run.py', 'benchmark',
    '--runs', '21', '--warmups', '3',
    '--output', 'benchmarks/statement-owner/.trn/timings.json',
], check=True)
PY
```

Swap the two `[[lanes]]` declarations in `suite.toml` for a counterbalanced run and use another output filename. Preserve the unchanged fallback control when extending the workloads. Generated Rust, native crates/binaries and scratch reports remain ignored under `.trn/`.
