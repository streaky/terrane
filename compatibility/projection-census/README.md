# Compiler-backed projection gap census

This directory owns ecosystem-assessment inputs and concise baseline reports for Milestone 31. It is deliberately separate from compiler conformance fixtures and the native survey corpus: listed packages are evidence-gathering inputs, not dependencies of the compiler and not regression requirements.

`packages.yaml` pins every package version, feature set, target, timeout, priority, and known representative workflow. Add or revise entries there. A package without a representative workflow receives static discovery and compiler-admission evidence only; the report labels that limitation rather than inventing a concrete use.

Run the complete Linux x86-64 assessment from the repository root with
`python tools/run-projection-census.py --summary-report
compatibility/projection-census/baseline-linux-x86-64.json`.

Use `--priority high-priority` or `--package strsim` for a narrower investigation. `--package` can be repeated. Each disposable workspace receives `terrane-dependencies.lock`, the complete exact recursive Cargo graph used for owner identity and private generated edges. Full machine-readable reports, locks, Rustdoc output, and projection caches remain under ignored `target/projection-census/`; the concise `baseline-linux-x86-64.json` keeps package outcomes, denominators, classifications, ranked structural causes, and representative-workflow status for review.

The runner builds the core `terrane projection-census` command once and invokes it separately for each package. That command runs shared native discovery and the compiler's actual projection resolver against the same exact Cargo graph. When a public path belongs to an external crate, it directly surveys the exact matching package from the resolved closure and assesses the recovered signature through the original facade path. Enum variants are recovered from their surveyed parent declarations. `core` declarations are generated from the pinned toolchain's `rust-src`; install that component with `rustup component add rust-src --toolchain nightly-2026-04-29`. `std` paths retain exact identity and item kind from the pinned Rustdoc summary when a standalone owner definition is unavailable. None of these evidence paths makes a transitive or sysroot owner directly importable. Absent owners, ambiguous resolved versions, failed owner surveys, and still-absent declarations remain explicit attempted failures. Reports distinguish:

- `projectable`: the current compiler admitted the operation;
- `deferred-contextual`: discovery succeeded, but independent static projection is insufficient and a justified concrete use is needed;
- `unsupported-capability`: projection explicitly declined the operation, with the compiler's reason;
- `namespace-path`: a Rustdoc-confirmed crate or module reexport with no operation or type signature to assess;
- `missing-metadata`: no declaration or pinned-toolchain item summary remained available after attempting the resolved closure and sysroot evidence paths;
- `environment-failure`: graph resolution or tooling failed before an assessment completed; and
- `resource-limited-unproven`: the configured package deadline expired.

These are operation-level results. Import success is not package usability, and static projection cannot prove generic or composition-sensitive workflows. Representative workflow records must say exactly what was exercised; `context-only` material is retained as selection rationale, not counted as compiler admission.
