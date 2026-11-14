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

### Runtime performance and provider maintenance

Net runtime effect: most text operations got faster or stayed level compared with the former caseless/unicode-* and std implementation. The one regression is case mapping of strings that contain non-ASCII characters.

The initial ICU implementation regressed casing and whitespace trimming badly on ASCII and short inputs. Review repairs added fast paths that give results identical to ICU's root locale:
- Whole-string ASCII `upper`, `lower` and `case-fold` use the equivalent ASCII mappings.
- Trimming checks ASCII boundary characters directly, including vertical tab, and consults ICU only for non-ASCII boundaries. It does not scan the whole string to choose a trimming path.
- First-cased-character selection and mapping also bypass ICU for ASCII.

Non-ASCII contextual casing remains ICU-owned. `upper.words` keeps ICU word segmentation rather than approximating punctuation/digit boundaries. Unit tests check every ASCII casing, folding, trimming and first-cased result against ICU's own mappings and property sets, not against the ASCII routines.

The review's throwaway release-profile harness compared the `main` support crate with the repaired one: same host, same driver code. The workloads are a 900 KB pangram ×20 for ASCII, a ~900 KB Latin/Greek/emoji string ×20 for mixed, and `"  hello world  "` ×1M for short. These are microbenchmarks, not application throughput guarantees:

| Operation | Workload | Before upgrade (ms) | After repairs (ms) | Change |
| --- | --- | ---: | ---: | --- |
| `case-fold` | ASCII | 204 | 0.87 | ~235× faster |
| `normalise nfc` | mixed | 306 | 28 | ~11× faster |
| `length` (graphemes) | ASCII | 359 | 118 | ~3× faster |
| `upper.words` | ASCII ×5 | 114 | 63 | ~1.8× faster |
| `upper.first` | short | 27.0 | 20.0 | ~1.35× faster |
| `upper` | ASCII | 1.35 | 1.41 | unchanged |
| `lower` | ASCII | 0.89 | 0.94 | unchanged |
| `upper` | short | 19.0 | 17.2 | unchanged |
| `trim` | short | 16.8 | 17.3 | unchanged |
| `lower` | mixed | 134 | 240 | ~1.8× slower |
| `upper` | mixed | 140 | 255 | ~1.8× slower |
| `case-fold` | mixed | 162 | 228 | ~1.4× slower |

The normalization and grapheme-segmentation gains apply to all text, ASCII or not. The regression is narrow: it affects only `upper`, `lower` and `case-fold` on a string containing **at least one** non-ASCII character. The ASCII fast-path decision covers the whole string, so a single accented letter or emoji sends that whole string through ICU. ASCII identifiers, keys, protocol text and most UI strings are as fast or faster. Bulk case conversion of non-English or mixed-language text pays about 1.4–1.8×, while still benefiting from the faster normalization and segmentation.

`ICU4X_VERSION` is the coordinated exact provider pin beside the compiler Unicode profile. Application and conformance support now share manifest writers; compiler tests parse the actual support-crate manifests and compare every ICU dependency with that pin. ICU4X exposes no compiled-data Unicode version constant, so a provider upgrade still requires deriving its Unicode version from upstream data and reviewing behavioral witnesses. Passing Unicode-18 witnesses alone does not detect every future data upgrade.

### Cold-build and dependency costs

The review's isolated string-support harness recorded the following cold builds at `-j8`. These are the review's measurements, not a new baseline rerun; lock entries describe that isolated harness, not the compiler workspace:

| Provider graph | Lock entries | Dev cold (ms) | Release cold (ms) | Stripped all-text-ops binary (bytes) |
| --- | ---: | ---: | ---: | ---: |
| Earlier caseless/unicode-* | 6 | 574 | 902 | 561,624 |
| Initial ICU4X 2.4.0 | 42 | 3,741 | 4,319 | 588,432 |
| ICU4X with locale-core substitution | 40 | 3,743 | 4,174 | Not measured |

Generated programs currently depend on both string and collection support unconditionally. The roughly three-second additional cold compilation cost is an accepted cost of the coherent Unicode profile, not a demonstrated application runtime cost. Runtime effects are recorded separately above. `icu_locale_core::langid!` replaces `icu_locale` because only identifier construction is needed. Removing locale data reduces dependency surface; it is not claimed to speed up cold builds. Workspace timing changes also include toolchain and cache-identity changes and cannot be attributed to ICU alone.

The non-ASCII casing regression is an accepted trade-off of the coherent ICU-owned Unicode 18 profile; these fast paths do not claim mixed-script parity with the former std implementation. Further optimization must preserve the pinned root-locale mappings and context rules, and should be driven by representative non-ASCII workloads.

Mapping safe ASCII runs inside mixed strings is a possible optimization, not implemented behavior or a committed plan. Any such change must retain context across run boundaries rather than case-mapping chunks independently.

## Cargo and lint policy

The workspace now explicitly denies Cargo's `unused_dependencies` lint through `[workspace.lints.cargo]`, alongside the existing Rust unsafe-code and Clippy all/pedantic policies. Cargo diagnostics are not rustc diagnostics: `RUSTFLAGS=-Dwarnings` alone is not a substitute for manifest lint policy. Check actual target/feature usage before removing a dependency. Partial target selections can withhold unused-dependency warnings, and this feature does not establish that dev-dependencies are minimal.

All-targets Clippy identified 16 `Option::map(...).unwrap_or_default()` sites across document helpers, compiler projection/lowering and debugger breakpoint resolution. They now use `map_or_default`, retaining lazy evaluation, ownership and default behavior. No blanket lint allowances were added, and the existing all/pedantic warning-denied gate passes.

Cargo 1.100 reorganizes **intermediate** artifacts. Its final profile directories and documentation outputs remain separate contracts. Terrane retains role-specific package names, generated workspaces, cache fingerprints and final artifact slots; Cargo's improved hashes are not a replacement for source-level cache identity or role isolation. No cross-workspace cache architecture or cache-retention policy is introduced by this upgrade.

The Cargo release also makes an available package lockfile the default for `cargo install`; `--locked` no longer changes installation resolution and is not a missing-lock guarantee. Publication-age cooldowns remain an optional dependency policy, not silently imposed by this cutover; locked versions are retained and Cargo install ignores the cooldown.

## Rust compatibility and improvements

### Public shared-owner identity

The first measured workspace run exposed callback and dereference regressions: the selected producer reports `Arc`, `Rc` and their weak handles beneath private `alloc::rcs` implementation modules. Shared path normalization now recovers their public owner paths both when classifying structured Rustdoc types and when spelling emitted Rust. It preserves generic arguments and associated-method suffixes, does not recognize arbitrary user types by short name, and does not weaken callback lifetime, mutability or Send/Sync requirements.

Projection policy revision 366 retires the cached declines from the earlier selection. Cache deletion is not the fix. Existing positive and negative carrier witnesses remain the contracts.

### Native never-type spelling

Rustdoc encodes the never primitive as `never`. Native Rust type rendering now maps that structured primitive to `!`, just as it maps `unit` to `()`, rather than emitting an invalid Rust identifier. A regression compiles the rendered type as an infallible `Result` error and executes the resulting program. Nameable aliases of the uninhabited native type retain a closed opaque identity, preserving defaults such as `MethodRouter<(), Infallible>` without erasing the error argument or converting it to source `none`. This does not introduce a Terrane bottom type or blanket admission of every diverging native API.

`Infallible` becoming an alias and never fallback changing on older editions remain native compatibility concerns. The workspace and generated crates already use edition 2024; that does not remove the need to compile older-edition dependencies, native aliases, callbacks and associated error contracts.

Native import demand now identifies a declaration family independently of its applied type arguments. Retained wrapper signatures can require a generic constructor even when a nested source call creates no named binding; value compatibility and generic argument checks remain strict.

The cached-result witness's `DefaultOnly<Infallible>` local annotation now spells the imported `DefaultOnly`, whose native generic default is exactly `Infallible`; its generated constructor wrapper still returns `DefaultOnly<core::convert::Infallible>`. The nested `make_lane::<_, _>` leaves the state binder to Rust's call context: `Builder<()>::attach` requires `Lane<(), Infallible>`, so it selects the same `S = ()` as the earlier explicit turbofish. This does not introduce runtime dispatch, allocation or a different specialization. The contextual form is intentionally retained rather than pinning incidental Rust spelling. A non-default `Builder<bool>` control exercises the same nested call without relying on the unit default. A detached call without an explicit result destination remains subject to `T0117`; that is a separate source-inference boundary, not evidence that Rust cannot infer the nested argument.

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
- `cargo test -p terrane-compiler --lib rust_interop::`: **85 passed** after the shared-owner, named never-alias and import-demand repairs, including native never-type compilation, alias/identity checks and atomic cache-publication cleanup.
- `cargo test -p terrane-compiler --test runtime_destructors`: **5 passed**.
- Scoped source conformance compiled and executed the Unicode witness, all selected awaited-destructor controls, core results and associated native owners. Generated Rust and its source-site sidecar were regenerated and reviewed; the Unicode source iterator yields the same complete cluster as text length reports.
- The first full run's callback and dereference regressions passed scoped positive/negative conformance after shared-owner normalization. The remaining callable-recipe case also passed after preserving the named never default and matching generic declaration import demand.
- Current native consumer witnesses passed for SQLx query chains, WebSocket protocols, native alias identity, Serde JSON results, PDF/document bytes and generated API re-exports.
- `cargo clippy --workspace --all-targets -- -D warnings` passed, as did the manual catalog generation/check and the three scoreboard-collector unit tests.
- The built CLI reports Rust 1.100.0 and the October producer. A canonical-Rust CLI workload compiled and printed grapheme count `1`, the complete U+1AE0 cluster, `strasse`, root-contextual `ος`, and `Hello World`.
- An isolated new-API Rustdoc document passed strict format-61 decoding and auxiliary rendering. Inspection identified the renderer's `never` spelling limitation; it is documented rather than misreported as lossless rendering.
- All six executable native-survey profiles load successfully with the updated producer selection and their exact fixture paths. The `markdown-associated-owner` profile also completed end to end, including its source conformance witness and survey report; both reported producer identities were `nightly-2026-10-02`.
- Earlier isolated 1.100 native probes, before the approved cache reset, compiled with `-Dwarnings`: allocator-aware Vec/Box sum 10; `Result<u32, !>` through an `Infallible` signature returns 42; guard dismissal/drop/unwind/pending-future scenarios produce exactly three cleanup calls. Cargo final custom-profile binary and Linux dynamic-library paths, JSON artifact filenames, new build-script OUT_DIR layout and final Rustdoc JSON placement were also exercised. These are narrow API/layout evidence, not a replacement for Terrane integration tests.

Full-workspace validation of the review repairs completed successfully with `python docs/measure-test-times.py -- --workspace` from clean source commit `a78bc31a`, without competing Cargo jobs: **2,929 passed, 0 failed, 0 ignored**, with Cargo wall time **955.71 seconds**. The authoritative tested commit, per-test results and timings are recorded in `docs/test-scoreboard.yaml` and its published HTML scoreboard. This run covers the provider/fast-path repairs and the follow-up independent Unicode test oracles.

The initial upgrade commit `f3de3026` has a recorded failing run; `17722fc5` repairs it. History is retained rather than rewritten, so bisecting this cutover can still encounter that intermediate failure. The original scoreboard-refresh commit has no verification body; the focused evidence below remains the durable record rather than retroactively amending its message.

### Review-repair verification

- String/collection support: **30 passed**; string unit tests were repeated successfully after final test cleanup.
- Compiler manifest contracts: **3 passed**; interop unit tests: **86 passed**. The renamed shared-owner boundary test also passed independently.
- The generated-manifest CLI contract passed with locale-core and the coordinated provider pin.
- Focused conformance passed for `text-families`, `projected-native-cached-result-template` and `projected-callable-blanket-recipe`. The cached-result program prints `7`, `11`, `7`; inspected Rust retains `DefaultOnly`'s exact native default and infers the nested `make_lane` state for both unit and boolean builders. Existing callback adaptation remains unchanged, with no added clone or allocation in those native calls.
- `cargo fmt --all --check`, all-targets warning-denied workspace Clippy, and manual catalog generation/check passed.
- The author's post-repair release probe confirmed the ASCII and short-string recoveries and was deleted afterwards. The before/after table above comes from the review's independent `main`-versus-repaired harness, which was also deleted.
- Follow-up test review replaced ASCII self-comparisons with independent ICU root-locale mappings and property sets across all 128 ASCII characters, covering casing, folding, first-cased selection and all whitespace-trimming directions. Unicode-18-specific witnesses are separated from general context/selection controls.

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
