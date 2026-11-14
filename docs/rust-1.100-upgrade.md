# Rust 1.100 upgrade and migration notes

Terrane now selects **Rust 1.100.0**. These notes record the implementation, compatibility decisions and verification. The [1.99 notes](rust-1.99-upgrade.md) remain the historical record of that earlier cutover.

## Selected toolchains and decoders

| Consumer | Selection | Contract |
| --- | --- | --- |
| Workspace and generated Rust | Rust 1.100.0; edition 2024 | Workspace minimum, compiler selection, generated support manifests and generated toolchain files agree. |
| Projection and native surveys | nightly-2026-10-02 | Accepts the minimum and exercised new stable library surface; participates in projection, native-proof and core-cache identities. |
| Strict Rustdoc decoder | rustdoc-types 0.61.0; JSON format 61 | Exact format validation remains mandatory. No schema-decoder migration was necessary. |
| Auxiliary complete-surface renderer | public-api 0.52.2, internally rustdoc-types 0.59.0 | Remains a separate lenient diagnostic renderer, not admission or lossless Rust syntax evidence. |
| Compiler-owned text profile | Unicode 18.0.0; exact ICU4X 2.4.0 providers | Coordinated text/collection support, generated metadata and provider versions; details below. |

The former August Rustdoc producer rejects a generated Rust-1.100 minimum. September 15 and September 23 producers report 1.100-nightly but reject allocator-aware `Vec<u32, System>` as unstable. October 2 accepts a crate using that type, `Result<u32, !>` and `DropGuard`, emits format 61, and passes the strict decoder. It reports 1.101-nightly, so native compatibility is verified against its actual sysroot rather than inferred from a matching release number. Neither minimum-version overrides nor injected feature gates are used.

The optional complete-surface renderer accepts the exercised format-61 document but prints the never primitive as `never`, not `!`. Its rendered text must not be compiled as a native signature or treated as a complete projection schema. No newer published `public-api` release was available during this upgrade. Strict structured facts remain authoritative; rendering errors remain errors rather than becoming empty fallback output.

Existing stable/producer/format/source identities invalidate old projection and proof caches. The producer selection is also updated in executable native-survey profiles and their runner. Historical benchmark reports and the 1.99 migration record are not rewritten to pretend they used the new toolchains.

## Unicode 18 migration

This upgrade replaces the split Unicode implementation rather than deferring it again. The former text profile used Unicode-16 folding, normalization and segmentation providers, while casing followed Rust std (Unicode 17 in Rust 1.99). The new implementation selects exact ICU4X 2.4.0 providers for the compiler-owned Unicode-18 operations, including root-locale casing and character-property decisions. Selecting a compatible system Rust must not silently select another text-table version.

The migration preserves the source API rather than removing folding, normalization or grapheme operations. Observable text results can nevertheless change when character assignments, mappings or boundary rules change. Review persisted case-folded/normalized keys, search indexes, derived identifiers and grapheme-based limits; rebuild derived data from original text where necessary. Byte and scalar indexing remain distinct from grapheme indexing.

Implementation and detailed behavioral evidence are described in the canonical Unicode contract and the verification section below. Provider changes, generated support publication and metadata are one coordinated cutover; the old caseless/unicode-normalization/unicode-segmentation direct dependencies and compatibility-only tinyvec pin are not retained as parallel text implementations.

Regression witnesses use actual changed data: `A` followed by U+1AE0 is now one extended grapheme, and U+1DF95 folds to `ss` and compatibility-normalizes to `ß`. Internal text operations consume borrowed grapheme slices, first-character casing writes directly into its final output, and word casing writes successive segments into one output buffer. Segmenter dependencies disable default `auto`/LSTM features and select only `compiled_data`, matching the default-boundary algorithm rather than pulling in unused complex-script segmentation machinery.

## Cargo and lint policy

The workspace now explicitly denies Cargo's `unused_dependencies` lint through `[workspace.lints.cargo]`, alongside the existing Rust unsafe-code and Clippy all/pedantic policies. Cargo diagnostics are not rustc diagnostics: `RUSTFLAGS=-Dwarnings` alone is not a substitute for manifest lint policy. Check actual target/feature usage before removing a dependency. Partial target selections can withhold unused-dependency warnings, and this feature does not establish that dev-dependencies are minimal.

All-targets Clippy identified 16 `Option::map(...).unwrap_or_default()` sites across document helpers, compiler projection/lowering and debugger breakpoint resolution. They now use `map_or_default`, retaining lazy evaluation, ownership and default behavior. No blanket lint allowances were added, and the existing all/pedantic warning-denied gate passes.

Cargo 1.100 reorganizes **intermediate** artifacts. Its final profile directories and documentation outputs remain separate contracts. Terrane retains role-specific package names, generated workspaces, cache fingerprints and final artifact slots; Cargo's improved hashes are not a replacement for source-level cache identity or role isolation. No cross-workspace cache architecture or cache-retention policy is introduced by this upgrade.

The Cargo release also makes an available package lockfile the default for `cargo install`; `--locked` no longer changes installation resolution and is not a missing-lock guarantee. Publication-age cooldowns remain an optional dependency policy, not silently imposed by this cutover; locked versions are retained and Cargo install ignores the cooldown.

## Rust compatibility and improvements

### Native never-type spelling

Rustdoc encodes the never primitive as `never`. Native Rust type rendering now maps that structured primitive to `!`, just as it maps `unit` to `()`, rather than emitting an invalid Rust identifier. A regression compiles the rendered type as an infallible `Result` error and executes the resulting program. This is native representation normalization, **not** the introduction of a Terrane bottom type or blanket admission of every diverging native API.

`Infallible` becoming an alias and never fallback changing on older editions remain native compatibility concerns. The workspace and generated crates already use edition 2024; that does not remove the need to compile older-edition dependencies, native aliases, callbacks and associated error contracts.

### DropGuard as a failure-path backup

Atomic projection-cache publication now uses `DropGuard` to retain temporary-file cleanup responsibility through failures and unwinding. Cleanup is armed only after successful exclusive creation, closes the owned file before removal, and remains armed through rename failure. A name collision cannot cause cleanup to delete someone else's file. Successful publication dismisses cleanup. Existing error context and old-target preservation remain intact.

A behavioral regression covers successful multi-part replacement and failed publication onto an existing directory: the existing sentinel is preserved and no owned temporary file remains. It does not use a test-only panic-injection seam.

This does **not** replace object destruction or the async destructor executor. Source objects already lower to lineage/construction-aware Rust `Drop`; awaited destruction has runtime admission, synchronous completion and shutdown draining, with an off-runtime/current-thread fallback. A synchronous guard cannot await or turn borrowed-object destruction into detached scheduling. Existing explicit cleanup APIs that report cleanup failures are retained rather than replaced by a `Drop` action that cannot return those errors.

### Smaller APIs and allocation assessment

Box/Rc/Arc mapping APIs were evaluated against current ownership sites. Existing boxed trait-object adaptation constructs the required owned carrier rather than transforming an already boxed value. Shared-storage handles intentionally clone an Arc so the selected handle outlives its mutex guard; mapping the owned payload is not an equivalent operation. No speculative pointer-map rewrite is made.

Prefix/suffix parsers retain `strip_prefix`/`strip_suffix` where match success matters; replacing them with the new trim helpers would discard that distinction. Tiny TLS restoration and error-reporting cleanup guards remain where the bespoke type is clearer or carries a contract that `DropGuard` cannot replace.

Stable allocator-aware `Vec` and `Box` were exercised with `System`. Lexer/parser buffers and Rustdoc/census processing were examined as possible allocation boundaries, but their values can escape the nominal phase into returned snapshots or reports, and no profiling evidence established an arena win. No allocator region syntax, blanket container migration, unsafe allocator implementation or unmeasured performance claim is introduced. Shared-pointer allocator support is not implied by the Vec/Box stabilization; `Arc::new_in` remains unstable in the versioned 1.100 API.

Other new unsafe/ZST APIs do not relax Terrane's unsafe permissions or native ABI admission. Stable sample-profile optimization is not enabled by default and does not replace the profiler. Platform-specific ABI changes still require evidence on their respective targets; Linux execution does not establish Windows, ARM or WASI behavior.

## Verification

Focused verification:

- Rust identities: `rustc 1.100.0 (0ef1bc386 2026-11-09)` and `cargo 1.100.0 (3acc79c0c 2026-11-05)`.
- `cargo test -p terrane-rust-analysis --all-targets`: **31 passed**, including strict core-cache recovery, native bound/call/implementation proofs, macro discovery, concurrent proof batches and the resolved SQLx accessor witness.
- `cargo test -p terrane-string-support -p terrane-collection-support`: **27 passed**, including actual Unicode-18-only data changes, unchanged text controls, collection ownership and clone-count witnesses. Repeated after disabling unused segmenter defaults.
- `cargo test -p terrane-compiler --lib rust_interop::projection::`: **81 passed**, including native never-type compilation and atomic cache-publication cleanup.
- `cargo test -p terrane-compiler --test runtime_destructors`: **5 passed**.
- Scoped source conformance compiled and executed the Unicode witness, all selected awaited-destructor controls, core results and associated native owners. Generated Rust and its source-site sidecar were regenerated and reviewed; the Unicode source iterator yields the same complete cluster as text length reports.
- Current native consumer witnesses passed for SQLx query chains, WebSocket protocols, native alias identity, Serde JSON results, PDF/document bytes and generated API re-exports.
- `cargo clippy --workspace --all-targets -- -D warnings` passed, as did the manual catalog generation/check and the three scoreboard-collector unit tests.
- The built CLI reports Rust 1.100.0 and the October producer. A canonical-Rust CLI workload compiled and printed grapheme count `1`, the complete U+1AE0 cluster, `strasse`, root-contextual `ος`, and `Hello World`.
- An isolated new-API Rustdoc document passed strict format-61 decoding and auxiliary rendering. Inspection identified the renderer's `never` spelling limitation; it is documented rather than misreported as lossless rendering.
- All six executable native-survey profiles load successfully with the updated producer selection and their exact fixture paths. The `markdown-associated-owner` profile also completed end to end, including its source conformance witness and survey report; both reported producer identities were `nightly-2026-10-02`.
- Earlier isolated 1.100 native probes, before the approved cache reset, compiled with `-Dwarnings`: allocator-aware Vec/Box sum 10; `Result<u32, !>` through an `Infallible` signature returns 42; guard dismissal/drop/unwind/pending-future scenarios produce exactly three cleanup calls. Cargo final custom-profile binary and Linux dynamic-library paths, JSON artifact filenames, new build-script OUT_DIR layout and final Rustdoc JSON placement were also exercised. These are narrow API/layout evidence, not a replacement for Terrane integration tests.

Full-workspace validation uses `python docs/measure-test-times.py -- --workspace` from a clean source commit, without competing Cargo jobs. The authoritative tested commit, per-test results and timings are recorded in `docs/test-times.yaml`; focused checks above are not a substitute for that measurement.

## Sources

- [Rust 1.100 announcement](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/)
- [Detailed Rust 1.100 release and compatibility notes](https://github.com/rust-lang/rust/releases/tag/1.100.0)
- [Cargo 1.100 changes](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-1100-2026-11-12)
- [Cargo 1.100 build-cache layout](https://doc.rust-lang.org/1.100.0/cargo/reference/build-cache.html)
- [Cargo 1.100 installation and lockfiles](https://doc.rust-lang.org/1.100.0/cargo/commands/cargo-install.html#dealing-with-the-lockfile)
- [Rust 1.100 DropGuard](https://doc.rust-lang.org/1.100.0/core/mem/struct.DropGuard.html)
- [Rust 1.100 Arc allocator API status](https://doc.rust-lang.org/1.100.0/std/sync/struct.Arc.html#method.new_in)
- [Clippy 1.100 changes](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-1100)
- [Canonical Terrane toolchain profile](../manual/reference/records/internals/build-toolchain-profiles.yaml)
- [Canonical Terrane Unicode contract](../manual/reference/records/language/text/unicode.yaml)
