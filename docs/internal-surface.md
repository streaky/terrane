# Terrane compiler-internal surface

This document maps implementation contracts that support the public Terrane object surface but are not language APIs. For the public surface implemented today, see [`surface-today.md`](surface-today.md). For language semantics, the language specification remains authoritative.

## Boundary

A compiler-backed object is not automatically internal. Public, reusable types and operations live in an organised `/core/*` namespace whether their current implementation is Terrane or Rust. Only irreducible host ABI calls, opaque ABI carriers, and lowering identities belong to this internal surface.

Terrane has no importable `/internal` namespace and no `/core/platform-*` compatibility namespaces. Compiler internals therefore cannot be reached by spelling a second namespace path.

## Package-private host bindings

Each bundled `/core/*` package is compiled from Terrane source. Before semantic analysis, the compiler seeds that package's own namespace with only the private host bindings needed by its implementation. Their local names begin with `host-`; their lowering identities use `intrinsic:<group>::<operation>`.

The authoritative inventory is `bootstrap_namespaces` in `crates/terrane-compiler/src/semantics.rs`. It groups bindings by their owning package:

|Owning package|Host groups|Purpose|
|---|---|---|
|`/core/streams`|`streams`|standard stream acquisition and byte-stream operations|
|`/core/filesystem`|`system`, `streams`|filesystem authority, path operations, and resource-backed file I/O|
|`/core/process`|`system`|arguments, environment, host name, and process exit|
|`/core/documents`|`data`|opaque document values, construction, traversal, and validation|
|`/core/documents/json`|`data`|JSON parsing and canonical writing|
|`/core/documents/yaml`|`data`|YAML parsing and safe writing support|
|`/core/urls`|`data`|URL parsing and component access|
|`/core/random`|`capabilities`|random sources, secret storage, hashes, and HMAC|
|`/core/codecs`|`capabilities`|hex and Base64 codecs|
|`/core/compression`|`capabilities`|bounded compression and decompression|
|`/core/random/uuid`|`capabilities`|UUID parsing and generation|
|`/core/networking`|`capabilities`|addresses, sockets, DNS, cancellation, and network resources|
|`/core/networking/tls`|`capabilities`|TLS client operations over transferred network resources|
|`/core/concurrency`|`concurrency`, `capabilities`|channels, synchronization, atomics, thread-local state, and cancellation|

These symbols have `Private` visibility. Bundled Terrane source in the exact owning namespace can resolve them as ordinary same-namespace names. Authored source, sibling core packages, and child packages cannot import them. The public classes, functions, and result objects in the bundled source are the only supported API.

## Protected bridges between core packages

A small number of implementation fields and helper functions must cross from a parent package into one of its organised children—for example, `/core/documents/json` operates on the document representation owned by `/core/documents`, and `/core/networking/tls` transfers resources owned by `/core/networking`.

Those declarations use Terrane `protected` visibility. They are available only in their declaring namespace and descendant namespaces. They are excluded from authored imports and from namespace-wide public imports. This is a core-package composition mechanism, not a user API.

## Rust support ABI

Lowering recognizes a private binding through `Symbol::compiler_identity()`, not through its public namespace spelling. The compiler emits only the Rust support modules required by the bundled core units included in the semantic package. Generated support records and functions may need Rust `pub` visibility because generated modules call across Rust module boundaries; that Rust visibility does not make them Terrane namespace objects.

Opaque host carriers include resource handles, capability handles, platform result records, and document representations. Their Rust layouts are compiler implementation details. Public Terrane wrappers expose typed values, structured result objects, ownership rules, and source-oriented diagnostics instead.

## Maintenance rules

- Add reusable user-facing behavior to the appropriate public `/core/*` package, not to this internal inventory.
- Give a new host operation to exactly one owning package unless a deliberate protected parent/child bridge is required.
- Keep host bindings `Private`; keep bridge declarations `Protected`; do not publish raw ABI carriers.
- Update `surface-today.md` only for public Terrane objects. Update this document when the internal grouping or visibility model changes.
- Regenerate lowered Rust artifacts through the compiler after changing a bundled core package or host binding.
