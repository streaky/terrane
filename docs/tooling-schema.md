# Terrane source-intelligence protocol 1.0

This document is the compatibility reference for the compiler-owned source-intelligence projection in
`terrane_compiler::tooling`. It documents public JSON fields, not internal Rust layouts or lowering IR.

## Transport and envelope

`terrane tooling --stdio` reads and writes one UTF-8 JSON object per line. Standard output contains
protocol frames only. `terrane query --request <json-file>` accepts one request or an array and emits
one response envelope per request. Within an array or one JSON-lines session, `$last`, `$last-build`,
and `$last-proposal` refer to the latest successful snapshot, generated build, and edit proposal.

Every request has:

```json
{
  "schema_version": "1.0",
  "request_id": "client-chosen-id",
  "operation": "open-snapshot"
}
```

Unknown request fields are rejected. This keeps misspelled or newer fields from silently changing a
request's meaning.

Every response has `compiler_version`, `schema_version`, `request_id`, and nullable `snapshot_id`,
`source_uri`, and `source_hash` context fields. A successful response has `result`; a failed response
has `error`:

```json
{
  "code": "expired-snapshot",
  "message": "snapshot is closed, evicted, or unknown; open a fresh snapshot",
  "retry_fresh_query": true
}
```

Schema `1.x` evolves additively. A different major version is rejected as `unsupported-schema` rather
than guessed. Compiler spans are zero-based, half-open UTF-8 byte ranges:

```json
{"start": 12, "end": 18}
```

## Snapshots

### `open-snapshot`

```json
{
  "schema_version": "1.0",
  "request_id": "open",
  "operation": "open-snapshot",
  "sources": [{"uri": "file:///work/main.trn", "text": "function main;\n"}],
  "manifest": null,
  "lock": null,
  "options": {
    "semantic": false,
    "generated": false,
    "target": "host",
    "profile": "development",
    "capabilities": [],
    "generated_entrypoint": "src/main.rs"
  }
}
```

`manifest` and `lock`, when present, use the same `{uri,text}` shape. A manifest supplies the package's
actual namespace, prelude, profile, executor, and dependency configuration; a lock input must match
the on-disk locked input consumed by analysis. Only the host target is currently supported. Sources
are sorted by logical URI before hashing; duplicate URIs and empty source sets are rejected. The
result contains:

- `compiler_version` and `schema_version`;
- content-derived `snapshot_id`;
- sorted `sources` with `uri` and SHA-256 `content_hash`;
- `manifest_hash` and `lock_hash` availability;
- `target`, effective `profile`, `capabilities`, and `generated_entrypoint`;
- dependency-projection cache/content identities and projected artifact names;
- `build_id` availability for exact generated output.

Snapshots are immutable. The service retains at most 32 snapshots and 64 MiB of source text by
default, evicting oldest snapshots first. Reopening identical inputs and options reuses the same ID.

### `close-snapshot`

```json
{"schema_version":"1.0","request_id":"close","operation":"close-snapshot","snapshot_id":"sha256:..."}
```

Closing also expires continuations and unapplied proposals associated with the snapshot.

## Availability

Facts never use an empty value to disguise missing analysis. A known value is encoded as an object;
unit states are strings:

```json
{"known":"/pkg/name::symbol"}
"unresolved"
"invalid"
"not-yet-analyzed"
"unsupported"
```

`invalid` means attempted analysis was blocked by invalid source. `not-yet-analyzed` means the
snapshot did not request that phase. `unsupported` means the public schema deliberately has no fact
for the object or target.

## Syntax and semantic queries

### `syntax`

Fields: `snapshot_id`, `uri`, and nullable `node_id`. Omitting `node_id` returns the root. The result
contains the source identity, ordered diagnostics, exact token/trivia arrays, and a recursive node:

```json
{
  "id": 0,
  "kind": "CompilationUnit",
  "span": {"start": 0, "end": 15},
  "state": "complete",
  "children": [{"field": "item", "node": {"id": 1, "kind": "FunctionDeclaration"}}]
}
```

The five node states are `complete`, `error`, `recovery`, `contains-recovery`, and `unsupported`.
`recovery` identifies a directly recovered node; `contains-recovery` identifies an otherwise valid
ancestor. Node IDs are deterministic only inside the exact snapshot. Tokens and trivia carry `kind`,
exact authored `text`, and `span`. Diagnostic objects carry `severity`, stable `code`, `message`,
nullable primary `span`, and nullable `help`.

### `locate`, `definition`, `references`, and `implementations`

Each accepts `snapshot_id`, `uri`, and an `offset` at a UTF-8 boundary and is available through both
JSON-lines stdio and one-shot query transports. `locate` returns the smallest syntax object containing
the position, augmented with nullable name and availability-tagged canonical symbol identity,
descriptor identity, value type, ownership, exact effects, capability requirements, declaration,
invocation mode, member facts, and inheritance. Ownership is `unsupported` until the semantic model
exposes an authoritative ownership state. Locations contain `{uri,span}`. Definitions and references
resolve by syntactic role and canonical semantic identity, not matching text. A reference query for
an interface method returns sites resolved directly to that interface identity; calls through a
concrete receiver belong to the resolved concrete override and remain discoverable from the
interface through `implementations`. `implementations` returns implementing descriptors for an
interface or implementing methods for an interface method. A query from a concrete method use first
follows its resolved method's owning interface contract, so its result contains every method
implementing that contract, including the resolved concrete method.

### `find`

```json
{
  "operation": "find",
  "snapshot_id": "sha256:...",
  "selector": {
    "kind": "Name",
    "child_field": null,
    "containing": null,
    "text": "value",
    "token_kind": "Identifier",
    "symbol_identity": null,
    "descriptor_identity": null,
    "include_recovery": false
  },
  "page_size": 100,
  "continuation": null
}
```

All selector fields are optional constraints. Results are sorted by URI, start, then end. Page sizes
are clamped to 1–1000. `complete: false` always carries an opaque continuation token whose remaining
match set is retained rather than recomputed. A continuation is single-use, bound to the exact
snapshot and selector, and one of at most 128 active continuations; expiry returns a retryable error.

With `include_recovery: false`, `find` excludes only nodes whose own state is `error`, `recovery`, or
`unsupported`. An otherwise valid `contains-recovery` ancestor remains eligible; clients can inspect
its state without treating the entire surrounding construct as a directly recovered match.

### `generated-rust`

Fields: `snapshot_id`, `uri`, `node_id`, and exact `build_id`. The result is an availability-tagged list
of `{build_id,path,span}` associations taken from final rendered Rust. Parse-only snapshots return
`not-yet-analyzed`; a different build is rejected as `mismatched-build`.

## Edits

### `propose-edits`

Fields: `snapshot_id` and `replacements`, where each replacement is `{uri,span,text}`. The engine
rejects invalid UTF-8 boundaries and overlapping replacements, reparses every changed source, and
performs semantic reanalysis when the original snapshot had semantic analysis. The proposal result
contains:

- `proposal_id` and `snapshot_id`;
- every affected `{uri,content_hash}` precondition;
- sorted non-overlapping `replacements`;
- `preview_diagnostics`;
- `semantic_reanalysis`.

### `propose-rename`

Fields: `snapshot_id`, `uri`, `offset`, and `new_name`. Rename finds references by canonical symbol
identity. Interface method contracts form one rename family with every implementing override, so a
request from either the interface or a concrete implementation updates the entire contract. Rename
rejects invalid identifiers (`invalid-name`), capture (`rename-capture`), and any candidate with parse
or semantic diagnostics (`invalid-rename`); the latter error includes the first preview diagnostic to
explain which contract would be broken. Rejected rename proposals are not retained. A clean rename
returns the ordinary edit-proposal shape only after successful semantic reanalysis.

### `apply-edits`

Fields: `proposal_id`. Proposals carrying preview diagnostics fail with `invalid-proposal` and are
never applicable. Disk application
accepts `file://` sources only. It reads and validates every content hash before the first write,
writes same-directory temporary files, then replaces originals. Stale content is never overwritten.
Cross-file crash atomicity is not promised; a `partial-apply` error carries a structured
`apply_report` with committed and uncommitted URIs and retained recovery paths.

LSP clients do not call disk apply. The language server converts proposals to versioned workspace
edits for the editor to apply. Rename engine errors surface as JSON-RPC invalid-params errors
(`-32602`) whose message retains the tooling error code and explanation.

## Language-server surface

The language server advertises the standard diagnostics, completion, hover, definition, references,
implementation, rename, document-symbol, formatting, code-action, signature-help, and semantic-token
capabilities. Its source code action formats the current document. Ordinary `didOpen` and `didChange`
analysis snapshots do not compile generated Rust. The custom `terrane/generatedRust` request accepts
a text-document position, lazily opens a short-lived generated snapshot from the exact current package
and editor overlays, returns the same availability-tagged exact-build generated locations as
`generated-rust`, and closes that generated snapshot before responding.

## Formatting

`format` accepts `snapshot_id` and `uri`, returning `uri`, `changed`, complete resulting `text`, and
zero or one replacement. It removes safe trailing whitespace and canonicalises parser-proven
assignment and infix gaps. It does not split operator-bearing identifiers, alter newline style,
rewrite comments or multiline strings, or modify malformed source that cannot safely be formatted.

The CLI exposes the operation as `terrane fmt [--check] <file-or-manifest>`. Check mode prints each
drifting path, returns exit status 1, and performs no write.

## Cancellation and lifecycle errors

`cancel` has a target `request_id`. A canceled queued request receives `canceled`. The initial stdio
transport processes frames serially; cancellation is therefore observable for a request ID queued
after its cancellation frame, not as thread preemption of a response already being encoded.

Clients should reopen and retry after `expired-snapshot`, `expired-continuation`, or
`stale-continuation` when `retry_fresh_query` is true. They must not reinterpret `canceled`, a partial
page, or unavailable facts as a successful empty query.
