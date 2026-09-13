# Terrane source-intelligence protocol 1.0

This document is the compatibility reference for the compiler-owned source-intelligence projection in
`terrane_compiler::tooling`. It documents public JSON fields, not internal Rust layouts or lowering IR.

## Transport and envelope

`terrane tooling --stdio` reads and writes one UTF-8 JSON object per line. Standard output contains
protocol frames only. `terrane query --request <json-file>` accepts one request object and prints the
same response envelope once.

Every request has:

```json
{
  "schema_version": "1.0",
  "request_id": "client-chosen-id",
  "operation": "open-snapshot"
}
```

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
    "capabilities": []
  }
}
```

`manifest` and `lock`, when present, use the same `{uri,text}` shape. Sources are sorted by logical URI
before hashing; duplicate URIs and empty source sets are rejected. The result contains:

- `compiler_version` and `schema_version`;
- content-derived `snapshot_id`;
- sorted `sources` with `uri` and SHA-256 `content_hash`;
- `manifest_hash` and `lock_hash` availability;
- `target`, `profile`, and `capabilities` analysis identity;
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

The four node states are `complete`, `error`, `recovery`, and `unsupported`. Node IDs are deterministic
only inside the exact snapshot. Tokens and trivia carry `kind`, exact authored `text`, and `span`.
Diagnostic objects carry `severity`, stable `code`, `message`, nullable primary `span`, and nullable
`help`.

### `locate`, `definition`, and `references`

Each accepts `snapshot_id`, `uri`, and an `offset` at a UTF-8 boundary. `locate` returns the smallest
syntax object containing the position, augmented with nullable name and availability-tagged canonical
symbol identity, descriptor identity, value type, ownership, effects, capabilities, and declaration.
Locations contain `{uri,span}`. Definitions and references use canonical semantic identity, not text
matching.

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
are clamped to 1–1000. `complete: false` always carries an opaque continuation token. A continuation
is single-use and bound to the exact snapshot and selector; expiry returns a retryable error.

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

Fields: `snapshot_id`, `uri`, `offset`, and `new_name`. Rename finds all references by canonical symbol
identity and rejects invalid identifiers or a spelling that resolves to another symbol at a changed
site. It returns the ordinary edit-proposal shape.

### `apply-edits`

Fields: `proposal_id`. Disk application accepts `file://` sources only. It reads and validates every
content hash before the first write, writes same-directory temporary files, then replaces originals.
Stale content is never overwritten. Cross-file crash atomicity is not promised; host replacement
failure reports committed, uncommitted, and retained recovery paths.

LSP clients do not call disk apply. The language server converts proposals to versioned workspace
edits for the editor to apply.

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
