# Terrane

Terrane is an experimental programming language for high-level native software. Its source focuses on values, behavior, and contracts; the compiler emits readable Rust, then uses Cargo and rustc to produce native code. There is no separate interpreter or virtual machine.

Everything is an object in Terrane's semantics, but not everything becomes a heap object. Integers can use machine scalars, calls can lower directly to Rust, and independent values can share copy-on-write storage until mutation. Native representation is the compiler's responsibility, not an extra object model the programmer must manage.

> **Status:** actively developed and built from source. The language, CLI, and generated-code contracts are not yet stable, and there is no finalized release distribution. See the [language scoreboard](docs/language-scoreboard.html) for implemented features and their evidence.

## What works today

- **Language:** indentation-sensitive syntax, namespaces and imports, lexical scope, typed and inferred bindings, functions with named/default/variadic arguments, closures, and bound methods.
- **Objects and types:** classes, nominal interfaces, reusable traits, authored generics, closed enums with payloads and exhaustive matching, optional/union types, descriptors, and reflection.
- **Values and ownership:** adaptive exact integers, fixed-width numbers, floating-point math, Unicode strings, bytes, typed collections, structural iteration, copy-on-write value semantics, references, explicit moves, and deterministic destruction.
- **Errors and concurrency:** structured throwable errors and callable `throws` contracts, async functions, structured task scopes, cancellation, deadlines, channels, heterogeneous async selection, native clocks/timers, and process-signal subscriptions.
- **Standard library:** capability-gated filesystem and process operations, byte/text streams, document values, JSON, safe YAML, URLs, codecs, digests, compression, randomness, UUIDs, TCP, UDP, DNS, TLS, and structured logging.
- **Native integration:** Terrane library packages, projected Rust dependencies, contextual generic specialization, concrete associated interfaces, native enums and typed dependency errors, checked ownership/borrowing crossings, and explicit authored Rust or C ABI boundaries.
- **Tooling:** source formatting and diagnostics, native unit/integration/end-to-end testing, compiler-backed source intelligence and VS Code support, LLDB-backed source debugging, and source-attributed CPU, allocation, and process-memory profiling.

These capabilities have executable coverage in [`tests/conformance/`](tests/conformance/) and the CLI integration suites. They do not mean that every Rust API or every combination of features is supported.

## Quick start

Install Rust and Cargo through rustup, plus a native linker for your host. The checkout selects its Rust release through [`rust-toolchain.toml`](rust-toolchain.toml). From the repository root:

```sh
cargo build -p terrane-cli -p terrane-language-server
./target/debug/terrane --version
```

Create `hello.trn`:

```terrane
namespace hello

function main;
  print; 'Hello from Terrane!'
```

Check it, run it, build a native executable, or inspect the generated Rust:

```sh
./target/debug/terrane check hello.trn
./target/debug/terrane run hello.trn
./target/debug/terrane build hello.trn
./target/debug/terrane rust -o hello.rs hello.trn
```

The run prints `Hello from Terrane!`. `build` prints the executable path; `rust -o` writes program-owned lowering and a support sidecar. Use `--release` with `build` or `run` for an optimized program. Building the compiler itself with Cargo's `--release` is a separate choice.

To install the CLI on your Cargo executable path, use `cargo install --path crates/terrane-cli`. In the commands below, `terrane` means that installed executable, or `./target/debug/terrane` when working from this checkout.

### Start a package

For a multi-file application, put the source above in `hello/src/main.trn` and create `hello/package.toml`:

```toml
package = "hello"

[namespaces]
hello = "src"
```

Run `terrane check hello`, `terrane run hello`, or `terrane build hello`. Commands accept a package directory, its manifest, or a standalone source file.

Generated state lives in `.trn/`; add `**/.trn/` to your repository's `.gitignore`. For projected dependencies, also ignore `terrane-projection.generated.trn`, but commit `terrane-dependencies.lock` and `terrane-projection.lock`.

A standalone source file can be run as `terrane hello.trn`. Adding `#!/usr/bin/env terrane` and making it executable enables direct script invocation; only shebang-bearing standalone files may omit `namespace`. Scripts still compile through the native pipeline.

## Working with Rust dependencies

Declare an exact crate version, reviewed features, and required effects in `[rust-dependencies.<alias>]` in `package.toml`. Import its admitted declarations from `/deps/<alias>/...`, then use ordinary package commands to resolve and project the dependency.

Projection is driven by source demand. Review `terrane-projection.generated.trn` for Terrane signatures and unavailable operations, `terrane-dependencies.lock` for the resolved Cargo graph, and `terrane-projection.lock` for admitted/declined API history. Unavailable demanded operations report explicit gaps rather than silently substituting an adapter.

The compiler preserves exact native types and obligations through calls, generic selections, callbacks, moves, and borrows. It can specialize operations from ordinary arguments, receiver types, callable contracts, explicit selections, and result destinations. This does not imply unrestricted projection of arbitrary Rust traits, lifetimes, macros, or host registration APIs. Executable integration evidence lives in the compiler's conformance and CLI integration suites.

Rust builds use the compiler-selected stable release; projection additionally uses a pinned rustdoc toolchain. Linux local dependency inspection requires `bubblewrap` (`bwrap`) and permission to create the containment sandbox. Trusted exact HTTPS projection artifacts can be configured with `TERRANE_PROJECTION_ARTIFACT_URL`; bundled, vendored, fully offline distribution is still unfinished. Compiler-owned Cargo commands use `sccache` only with the `TERRANE_SCCACHE=1` opt-in.

For an irreducible host boundary, a package may use an explicit authored Rust module. This is distinct from an automatically generated integration layer; ordinary projected calls should remain the first choice. See the [manual's dependency guide](https://github.com/streaky/terrane-manual/blob/main/reference/records/packages/rust-dependencies.yaml) for declarations, effects, ownership, failure contracts, and limitations.

## Commands and tools

| Command | Purpose |
| --- | --- |
| `terrane check <path>` | Validate Terrane and backend-check the generated Rust. |
| `terrane run <path> [-- arguments]` | Build and execute a native program. |
| `terrane build <path>` | Build the package's native artifact and print its path. |
| `terrane rust [-o app.rs] <path>` | Inspect generated Rust, or write split program/support files. |
| `terrane fmt [--check] <path>` | Format source, or check formatting without writing. |
| `terrane test [options] <package>` | Discover and run isolated native Terrane tests. |
| `terrane debug <path>` | Launch the LLDB-backed Terrane source debugger. |
| `terrane profile record --cpu <path>` | Record optimized CPU samples with Linux `perf`. |
| `terrane profile record --allocations <path>` | Record allocation, retention, and peak-live evidence with Heaptrack. |
| `terrane profile record --memory-timeline <path>` | Record a bounded Linux process-memory timeline. |
| `terrane profile show <file.trnprof>` | Present captured evidence with source attribution. |
| `terrane package hash <directory>` | Calculate a deterministic Terrane library source hash. |
| `terrane package install <relative-directory>` | Add a local Terrane library to the current package manifest. |
| `terrane projection-census <package> <version> --target <triple> --root <directory>` | Compare a Rust crate's public surface with actual compiler projection admission. |
| `terrane tooling --stdio` | Serve versioned JSON Lines source-intelligence requests. |
| `terrane query --request <json-file>` | Execute one source-intelligence request. |
| `terrane debug-adapter --stdio` | Serve the debugger translation layer to DAP clients. |
| `terrane toolchains` | Report Rust toolchains previously requested by Terrane. |

### Testing

`terrane test` discovers parameterless top-level `test-*` functions returning `none` under `tests/unit`, `tests/integration`, and `tests/end-to-end`. `/core/testing` supplies assertions, expected-error checks, deterministic context, and structured failures. Each selected test runs in an isolated process with bounded output and a deadline.

Use `--list` to inspect discovery, `--tier` to select tiers, and `--filter`, `--exact`, `--glob`, or `--regex` to select cases. `--jobs`, `--timeout`, `--fail-fast`, and `--show-output` control execution; `--report <file>` writes a structured JSON report. Test roots and capability profiles are configurable in the manifest. Frontend errors are checked before filtering; only selected tiers undergo native compilation.

See [`tests/native-testing/`](tests/native-testing/) for a tracked package exercising all three tiers, and the [testing reference](https://github.com/streaky/terrane-manual/blob/main/reference/records/packages/testing.yaml) for the full contract.

### Debugging, profiling, and editor support

The source debugger provides breakpoints, stepping, mapped frames, scope-aware locals, bounded value inspection, secret-field redaction, and explicit generated/native views. It requires the LLDB backend and currently has exact source fidelity for supported Linux x86-64 builds. Optimized source debugging is not supported; `debug --release` is rejected.

Profiling uses optimized native artifacts, not debugger builds. CPU sampling requires Linux x86-64 and host `perf` permission; allocation capture requires Heaptrack and zstd; process-memory timelines use Linux procfs. `--memory-timeline` may accompany CPU or allocation capture. Allocation reports support comparisons and allocated/retained byte thresholds. Retained-at-exit allocations are not automatically leaks, and RSS is not allocation-site evidence.

Debug/profile artifacts carry exact-build identity. Authored source snapshots and profile workload arguments are included only with explicit opt-ins; paths, symbols, and other diagnostic evidence can still be sensitive. See the manual's [debugging](https://github.com/streaky/terrane-manual/blob/main/reference/records/tooling/debugging.yaml) and [profiling](https://github.com/streaky/terrane-manual/blob/main/reference/records/tooling/profiling.yaml) references for platform, fidelity, privacy, and collector boundaries.

The [VS Code extension](editors/vscode/) uses `terrane-language-server` and compiler-owned snapshots for diagnostics, semantic highlighting, navigation, completion, and formatting. The shared source-intelligence protocol is documented in [`docs/tooling-schema.md`](docs/tooling-schema.md).

## What's still unfinished?

The major remaining work is:

- **Ecosystem-scale native integration:** broader contextual binding synthesis, declarative integration profiles, consuming native iteration/streams, and generated host registration. Existing bounded projection support is not a universal Rust binding generator.
- **Release hardening:** stable distribution and cache behavior, supported-platform release gates, actionable backend-failure reports, fuzzing, and clean-checkout reproducibility.
- **Offline projection distribution:** release-owned bundled artifacts, relocatable/vendored search paths, and an explicit network-disabled workflow.

See the [active compiler roadmap](docs/compiler-plan.md#7-active-milestone-roadmap) for exact remaining requirements. Source generics, enum matching, callable throwable bounds, destination-directed projected results, native testing, and allocation profiling are already implemented—not pending first-version work.

`no_std`, embedded/kernel targets, hot-code replacement, and unrestricted general pattern matching remain outside the current supported contract. Current compiler limitations and safe workarounds are recorded separately in the [manual](https://github.com/streaky/terrane-manual/blob/main/reference/records/tooling/current-limitations.yaml).

## Documentation

The [Terrane manual](https://github.com/streaky/terrane-manual) is maintained in a separate repository. It contains **The Terrane Book** (guided application development) and the **Terrane Reference** (current language, standard-library, package, tooling, and compiler contracts). An optional checkout at `manual/` is ignored by this compiler repository.

- [Concise language reference](docs/language-spec-concise.md) — compact syntax and contract lookup.
- [Language scoreboard](docs/language-scoreboard.html) — implementation status and evidence.
- [Compiler plan](docs/compiler-plan.md) — active work and completed milestone records.
- [Implemented object surface](docs/surface-today.md) — the current compiler-owned surface.
- [Proposed version-one surface](docs/surface-v1.md) — destination design, not implementation status.
- [Rust dependency design](docs/rust-deps.md) — projection architecture and design background.
- [Legacy specification/architecture draft](docs/language-spec-and-compiler-architecture-draft.md) — historical inventory, **not authority over the current manual or executable conformance**.
- [Scientific math benchmarks](benchmarks/sci-maths/) — cross-language correctness, runtime, and peak-memory evidence.

## Developing the compiler

Use focused checks while working, such as `cargo clippy --workspace --all-targets -- -D warnings`. For a specific conformance case, set `TERRANE_CONFORMANCE_FILTER=<case-name-or-comma-separated-fragments>` and run `cargo test -p terrane-compiler --test conformance every_manifest_drives_a_conformance_case`. Debug package-shaped fixtures from disposable copies, not their tracked source directories.

For complete workspace verification, run the timing collector from a clean tree at a commit:

```sh
python docs/measure-test-times.py -- --workspace
```

The collector returns Cargo's exit status and updates the tracked YAML/HTML test scoreboards. Review and commit both generated files together. It uses bounded parallelism; reduce `TERRANE_SCORECARD_JOBS` or `TERRANE_CONFORMANCE_JOBS` on constrained hosts, and do not run competing Cargo processes during collection.

To regenerate or verify only the scoreboard view:

```sh
python docs/generate-test-scoreboard.py
python docs/generate-test-scoreboard.py --check
```

Repository-local binaries record their build revision and compiler-input fingerprint. Rebuild if they warn that the checkout changed; installed/copied binaries do not inspect a working tree.

## License

Terrane is dual-licensed under the [Apache License 2.0](LICENSE-APACHE) and the [MIT license](LICENSE-MIT), at your option.
