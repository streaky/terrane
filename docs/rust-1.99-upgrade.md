# Rust 1.99 upgrade and migration notes

Terrane now selects Rust **1.99.0**. This upgrade changes some projected dependency surfaces as well as the compiler pin.

Terrane is pre-release and has no external users yet, so this is a **clean, announced source-breaking cutover**, not a compatibility-shim release. The changes below require reviewing imports, catches, and member calls even when Cargo dependency versions are unchanged. Future upgrades with external consumers will need an explicit announcement and migration/compatibility decision; regenerating inventories is not itself that announcement.

## Selected toolchains and decoders

| Consumer | Selection | Contract |
| --- | --- | --- |
| Workspace and generated Rust | Rust 1.99.0 | Shared workspace pin, minimum version, and generated toolchain selection. |
| Projection and survey facts | nightly-2026-08-01; rustdoc-types 0.61.0; JSON format 61 | Exact format validation and identity-based cache invalidation. |
| Optional complete-surface rendering | public-api 0.52.2, internally rustdoc-types 0.59.0 | Lenient auxiliary renderer, invoked only after strict format-61 validation; not lossless schema or admission evidence. |

The former Rustdoc producer reports Rust 1.97 and cannot satisfy the generated manifest minimum. The July 1.99 nightly still rejects stabilized C-variadic definitions. The selected August producer accepts them and emits format 61. A newer producer introduced a WebSocket return-contract regression, so selecting the latest nightly would also have imported future sysroot behavior unrelated to this stable upgrade.

No published `public-api` release currently matches format 61. Its pinned auxiliary renderer works on the exercised format-61 surface but can omit newer metadata or reject unsupported shapes. Such errors remain errors; there is no fake or empty fallback. Projection and the survey's structured native facts use the strict 0.61 decoder. Synthetic `rustdoc-types::Crate` fixtures use its format constant; historical census/report metadata need not pretend to come from the current producer.

Projection admission policy now uses schema **260**, invalidating schema-259 artifacts even when manifest, package versions and toolchain selections are otherwise identical. A cache hit must not preserve the old defining-crate-dependent IO policy.

## Source-breaking surface changes

| Area | Change and migration |
| --- | --- |
| IO `Result<T, E>` failures | Explicit public reexports of IO `Error` and `ErrorKind` are admitted regardless of whether Rustdoc names `std` or `core` as the defining crate. Where the native error has an admitted, displayable descriptor in the bound graph, failures raise that typed identity instead of `dependency-error`. Migrate affected catches to the projected `Error`. |
| Error messages | Typed failures preserve the native error's display text. Generic fallback failures additionally prefix dependency/member context. Do not parse the old prefix to classify typed errors; add application context at the catch boundary. Source frames still identify the call. |
| Serde JSON facade | The incidental `ErrorKind` class is no longer exported by the JSON facade after canonical IO ownership normalization. If an integration needs that identity, explicitly reexport standard IO `ErrorKind` from its maintained native binding rather than importing a signature-discovered helper. The comparison against pre-upgrade revision `7bae5175` shows `Map.clone_from` remains projected; `Map.get_key_value` was not projected there either. Neither is a member removal introduced by this upgrade. |
| Iced and SQLx owner namespaces | Conversion-owner namespaces ending in `trait/core/convert/from-std/io/error/error` become `trait/core/convert/from-std/io/error`. This affects Iced's `window/icon/error` and the SQLx/SQLx-core `error` scopes. The native conversion remains available, but imports of the old generated owner namespace must change. |
| Other inventories | Tokio, Reqwest, HTTP, Iced and other projections gain members or change native-owner spellings. Rustdoc defining paths are not a stable public namespace contract. Review actual member deltas, not just added-member counts. |
| Same-version removals | Missing members are now retained in removal history even when the dependency version is unchanged. Reusing one reports `S2031` as a previous-versus-current **projection** change. Reintroducing the member clears that removal. Toolchain, source or metadata changes need not imply a Cargo version bump. |

The Godot inventory comparison against `7bae5175` shows additions, not removal of previously recorded members or namespaces. `Signal::builder` was not projected in that baseline, and the recorded Quaternion conversion helpers remain. These review examples therefore do not justify inventing shims or announcing nonexistent removals. Generated helper/trait-owner names are still not promised stable across future upgrades.

### Typed errors, fallback errors, and panics

An admitted, displayable native `E` is the throwable identity for `Result<T, E>`. A supported standard-library error without an admitted descriptor retains `dependency-error`; an unrepresentable non-standard error declines the call rather than silently becoming text. Signature-only IO references do not manufacture facade exports. A descriptor admitted through another bound dependency still represents the same canonical native error identity.

For Tokio IO calls now using the typed descriptor, import and catch that type:

```terrane
from /deps/runtime/net import TcpListener
from /deps/runtime/io import Error

async function bind-listener;
    try
        await TcpListener::bind; '127.0.0.1:9000'
    catch Error as failure
        print; failure.message
```

`runtime` is an application-selected Tokio dependency alias. A `dependency-error` catch is not a catch-all for native typed errors. Contained unwinding panics are separately `dependency-panic`; abort profiles do not translate them.

A caught value exposes the common throwable interface, not the opaque native IO error's methods. On an ordinary projected native IO value, `Error.kind` is available in the selected inventory (it is an addition in the Tokio IO case, not a removed getter). **`io_error_kind` is not an interchangeable replacement for `kind`.** The tracked explicit-IO regression invokes `kind` and verifies its result denotes `AddrInUse` through a native witness.

Terrane's own typed listener catch was migrated without changing its expected `false` / `occupied` output. The authored package/project scan found no other IO `dependency-error` catches or imports of the removed JSON `ErrorKind` or old conversion-owner namespaces requiring migration. Existing HTTP-server fallback catches belong to other native error families and were retained; all 26 of its unit tests passed against the updated compiler.

## Rust, Cargo and Clippy evaluation

| Area | Outcome |
| --- | --- |
| C/C-unwind variadics and `VaList` | Useful for maintained native ABI adapters, not a replacement for Terrane's typed list-backed variadics. Complete and partial projection explicitly decline variable-tail functions instead of erasing the tail. Fixed-arity extern C siblings remain callable. Direct C-varargs admission would need a separate ABI-promotion and unsafe-caller contract. |
| Atomic update rename | Migrated the embedded native test clock from deprecated `fetch_update` to `try_update`, preserving memory orderings and checked-add behavior. Warning-denied generated-code compilation caught this; workspace Clippy alone does not compile embedded runtime fragments. |
| Safety requirements | The focused first-party audit found no leak-and-reclaim `Box::leak`, unchecked-pinning, `UnsafeCell`, native union or `transmute_copy` sites requiring migration. Finite Terrane unions are Rust enums. This is not an audit of every transitive dependency; unsafe permissions were not loosened. |
| Stabilized ownership, UTF-8 and layout APIs | Exercised owned lossy UTF-8 allocation reuse and replacement behavior, and raw slice layout/size/alignment queries. New Box/Vec ownership APIs are available to maintained adapters, but no existing ownership-transfer site justified a speculative rewrite. |
| Other stabilized APIs | Boxed-array iteration, reverse deque retention, file timestamp APIs and fused iteration provide future options; no required first-party rewrite was established. Cache identity remains semantic, not timestamp-based. |
| Rust compatibility and codegen | Existing range lowering, native compilation, debugger and profiler paths are covered by the full suite. No reliance on unspecified exhausted-range bounds was introduced. LLVM/static-PIE effects are verified on Linux only; no cross-platform or unmeasured performance claim is made. |
| Cargo | Existing explicit profiles, inherited default-feature settings and underscored lint names need no migration. No unrelated cache or CI-policy changes were added. |
| Clippy | Migrated applicable empty assertions, adopted `bit_width()`, and removed needless closure borrows without blanket allowances. Non-comparable assertions retain their existing behavior. |

Strict schema checks and toolchain/format cache identities remain in place. Reviewed projection locks no longer store the Rustdoc wire-format number: serialization metadata is checked by the decoder, while semantic locks compare actual dependency identities and projected members. Inventories were regenerated through the conformance harness, with member changes reviewed separately from metadata-only changes.

## Verification

The final unfiltered workspace outcome, revision, rustc identity and individual timings are recorded in the latest entry of [`test-scoreboard.yaml`](test-scoreboard.yaml) and the published test scoreboard. Golden updates are disabled for that verification run. Failed exploratory runs remain scorecard provenance rather than release instructions.

Final unfiltered result on implementation revision **`d2a6a4db`**, using **Rust 1.99.0**: **1,656 passed, 0 failed, 0 ignored; exit code 0**. Cargo elapsed time was approximately **17m 48s**. This includes the complete conformance corpus, generated warning-denied Rust, CLI/debugger/profiler integrations, language-server tests, runtime support and doc-tests. Workspace formatting and Clippy with `-D warnings` also passed before this run.

Tracked regressions exercise:

- `c-variadic-callsite-unavailable`: an actual foreign C declaration rejects at the unsafe call site with `S2029`, not an unsafe-permission or unused-import diagnostic.
- `c-variadic-fixed-arity-sibling`: a fixed-arity extern C function alongside the same variable-tail declaration compiles and prints `41`.
- `rust-dependency-explicit-io-error`: the IO getter identifies `AddrInUse`; a typed catch prints the unprefixed native message.
- `rust-dependency-signature-io-fallback`: an isolated signature-only IO error still reaches `dependency-error` with its dependency/member message prefix.
- Existing SQLx, listener and network cases preserve SQLx's own error identity, typed occupied-listener handling and supplemental network nominal discovery.

Additional real smoke evidence covers same-version removal and reintroduction (`S2031`, then restored execution); schema-259 admission-cache invalidation to schema 260 with unchanged dependency inventory; a complete-surface survey of `terrane-int-support` with format 61, including its rendered `parse_radix` signature; HTTP-server's 26 unit tests; compilation of the announced async IO catch example; and virtual-clock advancement to `7`, overflow rejection without mutation, then advancement to `12`.

The original native smoke's C-variadic definition returns `i32`; the tracked foreign declaration deliberately returns `void` and is never linked or called. The callable tracked sibling returns `i32`. These are separate capability and rejection witnesses, not one interchangeable ABI signature.

## Sources

- [Rust 1.99 announcement](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/)
- [Detailed Rust release and compatibility notes](https://github.com/rust-lang/rust/releases/tag/1.99.0)
- [Clippy 1.99 changes](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-199)
- [Cargo 1.99 changes](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-199-2026-10-01)
