# Terrane

Terrane is an experimental programming language for high-level native software. It keeps everyday
source focused on values, behavior, and contracts while leaving low-level representation choices to
the compiler.

Everything is an object in Terrane's semantics, but that does not require everything to become a
heap object at runtime. An integer can lower to a machine scalar, a statically resolved call can
become a direct Rust call, and an independent value assignment can use copy-on-write storage. The
compiler emits deterministic, readable Rust as an inspectable receipt, then uses Cargo and rustc to
produce native code.

> **Project status:** Terrane is under active development and is built from source; there is not yet
> a finalized release distribution. The compiler implements most of the planned first-version
> language and standard-library surface, but the language, CLI, and generated-code contracts are not
> yet stable. See the [language scoreboard](docs/language-scoreboard.html) for the clearest current
> feature-by-feature view.

## What works today

The working compiler goes from UTF-8 Terrane source to checked, deterministic Rust and native
executables. Implemented, executable behavior includes:

- indentation-sensitive syntax, comments, scalar and block literals, canonical formatting, and
  recovering source diagnostics;
- packages, slash-separated namespaces, imports, lexical scope, globals, manifests, and compile-time
  build selection;
- adaptive exact integers, fixed-width integers, `float32` and `float64`, coercion and bounded
  arithmetic families, and the foundational floating-point mathematics surface;
- typed functions, named and default arguments, closures, bound methods, control flow, structured
  errors, and exact throwable inference for directly known callables;
- strings, Unicode scalar and grapheme views, immutable bytes, pinned Unicode behavior, encodings,
  lists, maps, sets, tuples, ranges, copy-on-write values, and structural iteration;
- classes, nominal interfaces, traits, descriptors and reflection, construction and destruction,
  explicit ownership transfer, references, borrow-oriented lowering, and cycle diagnostics;
- async functions, structured task scopes, cancellation and deadlines, byte and text streams,
  typed channels, synchronization cells, and projected Rust futures, sequences, and sinks;
- capability-gated filesystem and process facilities, document values, JSON, safe YAML, URLs,
  randomness, codecs, digests, compression, UUIDs, TCP, UDP, DNS, TLS, and structured logging;
- projected Rust dependencies, including closed associated interfaces and recursive supertrait
  conformance, generated Cargo projects, source-projected backend diagnostics, executable scripts,
  target profiles, system and C ABI packages, and VS Code language-server support.

These are supported claims only where executable conformance evidence exists. The specification also
describes settled destination behavior that has not all shipped yet, and `demos/` deliberately
contains design pressure tests that may use unsupported combinations.

### Remaining first-version work

The principal unfinished areas are:

- destination-directed specialization for closed generic results projected from Rust dependencies;
- precise throwable bounds on function types;
- a Terrane-native unit, integration, and end-to-end testing framework;
- the remaining release hardening and cross-platform release gate; and
- bundled, vendorable, relocatable projection artifacts for fully offline dependency builds.

Some surrounding infrastructure already exists for these items, so the
[compiler plan](docs/compiler-plan.md) is the authority for their exact remaining scope.
Source-declared generics, general pattern matching, `no_std`, embedded and kernel targets, hot-code
replacement, and several other exploratory ideas are not first-version promises.

## Design priorities

- **Semantic objects, efficient representations.** Source behavior is stable even when its concrete
  native representation is specialized or erased.
- **Readable everyday code.** Common syntax favors clear words and visible control flow over
  punctuation-heavy ceremony.
- **Progressive strictness.** Programs can begin concisely and state sharper type, ownership,
  visibility, capability, and build contracts where they matter.
- **Inspectable lowering.** Generated Rust is a supported debugging and auditing surface, not hidden
  compiler exhaust.
- **Native reach without a second execution world.** Rust supplies compilation, linking, platform
  support, interoperability, and the systems ecosystem; Terrane supplies its own source semantics.
- **Explicit power.** Shared identity, ownership transfer, unsafe operations, dependency effects, and
  platform capabilities remain visible choices.

## Why Rust underneath?

Terrane is a source language, not a new virtual machine. Rust already provides a mature native
compiler ecosystem, optimization, memory-safety machinery, linking, C interoperability, WebAssembly,
platform support, and access to a large library ecosystem. Building those layers again would shift
work away from the language semantics Terrane is intended to explore.

Lowering to Rust also lets the compiler remove abstractions that are no longer observable. Terrane
semantics do not map one-for-one onto Rust, but once the compiler has proved a concrete
representation safe, generated code can use direct scalars, calls, enums, borrows, and specialized
storage rather than a universal boxed runtime value. The resulting Rust stays readable so that the
choice can be inspected, profiled, and debugged.

## Toolchains and Rust dependencies

Each Terrane version selects one stable Rust release. The workspace `rust-toolchain.toml`, Cargo
`rust-version`, compiler build toolchain, and generated crates use the same version. Rustup installs
the selected toolchain when needed. A package may explicitly choose `rust-toolchain = "system"` to
use a compatible ambient compiler instead.

Compiler-owned Cargo commands use `sccache` only when `TERRANE_SCCACHE=1` opts in.

Projects with `[rust-dependencies]` use projected dependency metadata rather than treating arbitrary
Rust APIs as if they automatically satisfy Terrane's contracts. On Linux, local projection uses
`bubblewrap` (`bwrap`) for contained Cargo and rustdoc inspection. Terrane can also resolve an exact,
trusted HTTPS projection artifact configured through `TERRANE_PROJECTION_ARTIFACT_URL`; verified
artifacts are cached and their provenance is recorded in `terrane-projection.lock`. Bundled and
vendored offline artifact distribution remains unfinished.

## Developing the compiler

Run focused Cargo checks while working. For complete workspace verification and per-test/conformance
timing, use the repository collector from the root:

```sh
python docs/measure-test-times.py -- --workspace
```

Do not run another Cargo build or test process concurrently with the collector. It returns Cargo's
exit status, updates `docs/test-scoreboard.yaml`, and regenerates
`docs/test-scoreboard.html`; review and commit those two generated files together when the timing
history is intentionally refreshed.

To regenerate or verify only the test scoreboard view:

```sh
python docs/generate-test-scoreboard.py
python docs/generate-test-scoreboard.py --check
```

The conformance corpus under `tests/conformance/` is the executable authority for implemented
language behavior. Accepted cases exercise checking, lowering, generated-Rust compilation, and—when
behavior matters—execution. Rejected cases pin source diagnostics and malformed boundaries.

Manual CLI debugging of a package-shaped conformance fixture can rewrite its tracked
`terrane-projection.lock`, including changing recorded projection provenance on a cache hit. Run
such experiments from a disposable copy of the entire case directory (including `.cargo` and
`fixture-registry`), or restore the lock immediately afterwards. Ordinary corpus runs are safe:
the conformance harness stages package cases before invoking the compiler.

## Documentation map

- [Concise language reference](docs/language-spec-concise.md) — the best first stop for syntax and
  language/compiler contracts.
- [Full language specification and compiler architecture](docs/language-spec-and-compiler-architecture-draft.md)
  — authoritative semantics and architecture when exact detail or conflicts matter.
- [Compiler plan](docs/compiler-plan.md) — remaining milestone scope, sequencing, exit criteria, and
  completed implementation evidence.
- [Language scoreboard](docs/language-scoreboard.html) — generated, feature-level status and links to
  evidence and reference coverage.
- [Implemented object surface](docs/surface-today.md) — descriptive map of what the compiler exposes
  today.
- [Proposed version-one object surface](docs/surface-v1.md) — destination design, not implementation
  status.
- [Rust dependency projection design](docs/rust-deps.md) — projection, containment, cache, diagnostics,
  and artifact contracts.
- [`manual/`](manual/) — the tutorial and reference manual when that separate repository is checked
  out beside the compiler.
- [`benchmarks/sci-maths/`](benchmarks/sci-maths/) — cross-language correctness, runtime, and
  peak-memory benchmark corpus with recorded environment and toolchain evidence.

Editor support lives in [`editors/`](editors/). The VS Code extension launches
`terrane-language-server`, which reuses the compiler frontend for semantic highlighting and source
diagnostics.

## Quick start

Terrane currently requires Rust and Cargo. From the repository root, build the CLI:

```sh
cargo build --release -p terrane-cli
./target/release/terrane --version
```

Create `hello.trn`:

```terrane
namespace hello

function main;
  print; 'Hello from Terrane!'
```

Then check, run, build, or inspect it:

```sh
./target/release/terrane check hello.trn
./target/release/terrane run hello.trn
./target/release/terrane build hello.trn
./target/release/terrane rust hello.trn
```

`check`, `rust`, `build`, and `run` all use the same source, resolution, semantic, lowering, and
Cargo pipeline. They differ only in how far they take the result:

| Command | Result |
| --- | --- |
| `terrane check <path>` | Validates Terrane and compiles the generated Rust without running it. |
| `terrane rust <path>` | Prints the deterministic generated Rust. |
| `terrane rust -o app.rs <path>` | Writes authored lowering to `app.rs` and support code to `app.support.rs`. |
| `terrane build <path>` | Builds a native executable and prints its path. |
| `terrane run <path>` | Builds and runs the program, forwarding arguments after `--`. |
| `terrane <file.trn> [args]` | Runs a source file directly, which is useful for executable scripts. |
| `terrane toolchains` | Reports Rust toolchain pins previously requested by Terrane. |

Use `--release` with `build` or `run` for an optimized executable. Use
`--require-canonical-rust` with a compiler command when generated Rust must already match Terrane's
bundled formatter.

A standalone source file can also be an executable script:

```terrane
#!/usr/bin/env terrane
function main;
  print; >hello
```

```sh
chmod +x hello.trn
./hello.trn one two three
```

A shebang-bearing direct source file may omit `namespace`; manifest-discovered source files may not.
Compilation still happens before execution—this is the ordinary native pipeline, not an interpreter.

## License

Terrane is dual-licensed under the [Apache License 2.0](LICENSE-APACHE) and the
[MIT license](LICENSE-MIT), at your option.
