# Developing Terrane

This guide is for contributors to the compiler and its tooling. For writing Terrane programs, start with the [repository README](../README.md) and the [Terrane manual](https://github.com/streaky/terrane-manual).

## Build the compiler

The checkout selects its Rust toolchain in [`rust-toolchain.toml`](../rust-toolchain.toml). See the [current toolchain migration notes](rust-1.100-upgrade.md) for the Rust and Unicode compatibility details, and the [previous migration](rust-1.99-upgrade.md) for its projection and error-handling changes.

```sh
cargo build --locked --bin terrane
cargo clippy --workspace --all-targets -- -D warnings
```

Cargo displays a progress bar by default in this checkout. Set `CARGO_TERM_PROGRESS_WHEN=never` to disable it. Terrane's compiler-owned Cargo commands use `sccache` only with `TERRANE_SCCACHE=1`.

`terrane --version` (`terrane -V`) reports build identity and both Rust toolchains. Development builds include their Git branch and abbreviated commit when available. Cargo's `--release` selects an optimized Rust build; it does not designate a distribution build. Distributors can use `cargo build --locked --release --bin terrane --features release-build` to label the binary `release` and omit Git details. Both builds take the version from the workspace manifest. Rebuild checkout-local binaries when they warn that compiler inputs have changed; installed/copied binaries do not inspect a working tree.

## Verify a change

Run focused checks first. To select conformance cases by name fragment, use `TERRANE_CONFORMANCE_FILTER` (a single fragment or a comma-separated list):

```sh
TERRANE_CONFORMANCE_FILTER=time-exact-values \
  cargo test -p terrane-compiler --test conformance every_manifest_drives_a_conformance_case
```

- Keep regression programs minimal and self-contained under tracked fixtures. Debug package-shaped fixtures from disposable copies, not their tracked source directories. Workspace tests do not require local applications under `projects/`.
- Assert result classifications and failure details through structured reports rather than wording from human-readable summaries. Combine related CLI assertions when they can share one native build without losing a distinct failure case.
- Preserve native type and runtime boundaries with small local Rust witnesses. For example, `projected-generated-api-reexport` exercises a build-script-generated hidden owner and separate facade without pulling Godot into normal conformance. Real-crate compatibility evidence belongs in dedicated [native surveys](../tests/native-survey/) or application checks, not in claims about what those local fixtures prove. Small real Rustdoc inputs support focused cache-key tests without replacing real-sysroot coverage.
- Host-only Rustdoc fixtures must discover the host triple: selecting a hard-coded target also sets Cargo's `--target` and can accidentally require cross-compilation. Fixture registries may be symlinked into rejection cases; preserve shared API boundaries and exercise the dependent case family when reducing one.
- Keep profiling workloads long enough to collect meaningful samples and inspect fresh-source reports before changing a workload for comparison. The language-server package at `tests/fixtures/lsp-outside-namespace` covers documents outside package namespace roots.

## Refresh the test scoreboard

Run the timing collector from a clean, committed tree. Do not run competing Cargo processes during collection.

```sh
python docs/measure-test-times.py -- --workspace
```

The collector returns Cargo's exit status. It uses bounded parallelism; reduce `TERRANE_SCORECARD_JOBS` or `TERRANE_CONFORMANCE_JOBS` on constrained hosts. Review and commit both the YAML and HTML scoreboard together. Full test names, including Rust module prefixes, must be at most 80 characters. Name validation happens after collection and before publishing either scoreboard file (even with `--no-html`), not during test discovery.

To regenerate or verify only the HTML view:

```sh
python docs/generate-test-scoreboard.py
python docs/generate-test-scoreboard.py --check
```

## Measure Rust implementation coverage

Coverage is a separate diagnostic run and does not update timing scoreboards. It measures handwritten Rust under `crates/*/src`, including support crates executed by generated conformance programs and native test runners. It is not Terrane source coverage. Use the checkout's selected Rust toolchain and matching LLVM tools:

```sh
rustup component add llvm-tools-preview
python tools/coverage.py
```

Open `target/coverage/index.html` in a browser for annotated source, execution counts, and next-uncovered navigation. Reports are self-contained. `--output <directory>` sets the exact report directory. Each run replaces the previous report, profiles, and executable coverage objects there but retains the reusable `build/` cache.

For individual test attribution, start with a focused selection:

```sh
python tools/coverage.py --per-test -- -p terrane-string-support --lib
python tools/coverage.py --filter tracked_native_testing_package_covers_all_tiers -- -p terrane-cli --test package_cli
TERRANE_CONFORMANCE_FILTER=time-exact-values python tools/coverage.py -- -p terrane-compiler --test conformance
```

Arguments after `--` select Cargo packages or targets; `--filter` selects Rust test names. Without a Cargo selection the runner uses `--workspace`. Per-test mode runs each non-ignored Rust test in its own process, showing linked reports, duration, executed-function footprint, unique functions within the selection, and overlap. Function identity uses canonical source ranges. Overlap can guide review, but it does not prove test redundancy. Conformance attribution stops at the outer Rust harness test, not individual Terrane cases. Run attribution regressions with `python -m unittest tools.test_coverage`.

`run.json` records revision, dirty-tree status, filters, environment overrides, toolchain, and failures. Failed tests still produce reports and a nonzero exit status; crashes without profiles are **unavailable**, not zero coverage. Build/report errors other than absent profiles stop the run. Outer build-script and test-discovery profiles are retained separately from execution totals; nested generated Cargo actions can contribute profiles. LLVM mapping warnings are linked from the report; counts can include dependency or fixture mappings and do not quantify missed compiler functions. Do not treat warned reports as clean baselines.

Stable doctests, integration-test source, generated Rust, dependencies, and `build.rs` are outside the reported source scope; colocated unit tests remain included. Embedded fragments executed only as generated Rust are not remapped. Files without mappings are listed as **unmeasured**; percentages apply to mapped executable code and are not branch coverage. Aborted subprocesses or children that clear the coverage environment may not contribute. The debugger host is measured, but native debuggees retain the caller's Rust flags so instrumentation cannot change stepping sequence points.

Do not run coverage beside competing Cargo or suite runs. There is no outer Rust-test deadline; supervise the command if one is needed. Native test-runner deadlines are unchanged.

## Implementation references

- [Compiler plan](compiler-plan.md#7-active-milestone-roadmap), [implemented object surface](surface-today.md), and [proposed version-one surface](surface-v1.md). The [legacy language/architecture draft](language-spec-and-compiler-architecture-draft.md) is a historical inventory, not authority over the manual or executable conformance.
- [Rust projection design](rust-deps.md), [projection census and compatibility evidence](../compatibility/projection-census/), and [source-intelligence protocol](tooling-schema.md).
- [Security contracts](security/README.md) record review boundaries, not a security certification. Run `terrane run tools/security-contracts -- check` from the repository root to validate registered regions or use `terrane run tools/security-contracts -- impact --base main` to report known contracts affected by changes.
