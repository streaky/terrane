# Rust 1.99 upgrade evaluation

## Selected inputs and migration

Terrane now selects Rust **1.99.0** for its workspace and generated applications/support crates. Native projection and surveys select **nightly-2026-10-02**, paired with **rustdoc-types 0.61.0 / JSON format 61**. The previous producer, nightly-2026-04-29, reports Rust 1.97 nightly and cannot satisfy the new generated manifest minimum. The already installed nightly-2026-07-06 reports Rust 1.99 nightly but still rejects stable C-variadic definitions; the selected October producer accepts them. Its emitted JSON format was measured before selecting the decoder. Strict schema checks remain enabled; toolchain/format cache identities invalidate older projection artifacts and proof results.

Schema migration adds stability metadata to synthetic Rustdoc items and default-implementation instability metadata to synthetic functions, and updates associated-item patterns. Synthetic crate fixtures use the decoder's format constant instead of a stale literal. Historical benchmark and scoreboard evidence retains its original toolchain versions.

## Release assessment

| Area | Terrane impact and action |
| --- | --- |
| C/C-unwind variadic definitions and `VaList` | Useful for maintained native ABI adapters. Not a replacement for Terrane's typed, list-backed variadic parameters. Complete and partial native projection now explicitly decline C-variadic functions instead of losing the variable tail. Fixed-arity extern C functions remain admitted. ABI promotions, argument count/types and unsafe caller obligations need an explicit design before a direct Terrane C-varargs surface can be admitted. |
| Raw-pointer layout APIs | `Layout::for_value_raw`, `size_of_val_raw`, and `align_of_val_raw` allow layout queries without fabricating a reference. No existing Terrane use needs migration; no unsafe layout machinery added merely to use them. Potentially useful for a future native ABI adapter with proven pointer metadata. |
| `Box::leak` ownership guidance | No `Box::leak` usage found in workspace crate sources. Future temporary ownership transfers should use `into_raw`/`into_non_null`, not leak-and-reclaim. |
| `Pin::new_unchecked` safety changes | No use found in workspace crate sources. Generated/runtime futures use safe `Box::pin` and `pin!`; no unchecked-pinning migration required. |
| `UnsafeCell`, allocation growth and native union guidance | No `UnsafeCell` or native Rust union declarations found in workspace crate sources. Terrane finite union carriers are Rust enums, not unsafe tagged unions. No custom allocator migration indicated. This is a focused first-party review, not an audit of every transitive dependency. |
| `transmute_copy` now panics without unwinding | No use found in workspace crate sources; no panic-boundary adjustment required. |
| `raw_borrows_via_references` lint | The audited signal ABI code already uses explicit raw borrows for stack pointers. The new lint is allow-by-default; no blanket warning suppression introduced. |
| Box non-null and Vec part conversions | Useful when genuine allocation ownership crosses an ABI boundary. No current matched ownership-transfer site requires replacement; ordinary safe containers remain preferable. |
| Owned lossy UTF-8 APIs | Can reuse an owned valid UTF-8 buffer. Current CLI process-output decoding commonly borrows bytes, iterates lines, or trims text; no demonstrated allocation win warrants a speculative rewrite. |
| Boxed-array iteration, `VecDeque::retain_back`, fused `StepBy` | Available to generated Rust and maintained adapters. No required first-party rewrite identified. |
| File timestamp APIs | Potentially useful to future filesystem/package operations, but do not justify adding timestamp behavior or using timestamps for semantic cache identity. |
| Exhausted `RangeInclusive` behavior | The release improves inclusive iteration but does not preserve exhausted `start`/`end` or slice-index behavior. No new dependency on that unspecified behavior is introduced; full conformance exercises existing range lowering. |
| Cargo built-in `debug` profile | Initially equivalent to `dev`. Existing explicit workspace profiles and named generated debugger/profiler profiles remain unchanged; artifact directory `debug` is not a custom Cargo profile definition. |
| Cargo edition-2024 inherited dependency features | No member currently overrides an inherited dependency's `default-features`. Root `ureq`/`rustls` settings already disable defaults explicitly, so no requested feature change. |
| Cargo CI incremental default | Root dev/test profiles explicitly enable incremental compilation. No broad CI/cache-policy change is needed for this upgrade. |
| Cargo lint names and startup improvements | Existing lint names use underscores. Cargo performance improvements are inherited without Terrane code changes; no unmeasured speedup claimed. |
| Clippy 1.99 | Migrated applicable empty assertions to equality assertions with useful failure values, adopted integer `bit_width()`, and removed newly diagnosed needless closure borrows. Non-comparable values retain their existing boolean assertions. No new blanket allowances. |
| Other compatibility changes | No first-party legacy integral-module constants, generic unmangled exports, unchecked pinning, or transmute-copy sites found. The unmangled CLI dynamic-library fixture exports a nongeneric function. New panic, macro, inference, documentation and codegen behavior is checked through the full workspace rather than speculative compiler changes. |
| LLVM 23 / static PIE / Rustdoc performance | Inherited compiler changes. Linux executable/debugger/profiler and projection coverage are the compatibility gate; other operating systems remain unverified. Rustdoc performance claims do not directly describe the separately pinned producer's measured Terrane workload. |

## Verification evidence

- `cargo check --workspace --all-targets`: passed after JSON-schema migration.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed after new-lint migration.
- `cargo fmt --all -- --check`: passed before the full-suite prebuild.
- `cargo test --workspace --no-run`: passed, compiling all workspace test binaries.
- Focused native-admission regression: passed; C-variadic signature declines, has no misleading partial callable, and the equivalent fixed-arity C signature preserves its native integer contracts and unsafe status.
- Disposable native package smoke: a Rust 1.99-minimum dependency containing a stable C-variadic definition was projected with the new producer; its fixed-arity extern C function compiled and ran through Terrane, printing `42`. Generated `rust-toolchain.toml` selected `1.99.0`. Calling its variadic function reported source-oriented `S2029` with the explicit C-variadic decline. Importing an unused unavailable member alone is not an invocation error.
- The initial scorecard invocation was interrupted by the command runner's 300-second deadline after the debugger binary passed all 33 tests and while package CLI tests were still running. It is not full-suite evidence. The replacement invocation disables the outer command deadline and retains the unfiltered workspace selection.
- Full workspace outcome is recorded in the latest `runs[]` entry of `docs/test-scoreboard.yaml`, including revision, rustc, counts and exit status; its published HTML is refreshed alongside it.

## Sources

- [Rust 1.99 announcement](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/)
- [Detailed Rust release and compatibility notes](https://github.com/rust-lang/rust/releases/tag/1.99.0)
- [Clippy 1.99 changes](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-199)
- [Cargo 1.99 changes](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-199-2026-10-01)
