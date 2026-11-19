# Security contracts

This directory owns security-sensitive maintenance contracts, versioned with Terrane's implementation. YAML files use the shared manual format's dedicated `security-contract` record kind. They are not copies of the standalone manual and do not require its checkout. A later manual build can explicitly include these records under Internals → Security contracts; this change does not implement manual assembly or choose manual versioning.

A contract identifies a guarantee, its threat boundary, assumptions, limitations, application guidance, implementation, evidence, and review considerations. Registration means **security-sensitive**, not audited, certified, or proven secure. The pilot is [directory-handle confined opens](filesystem-beneath-open.yaml).

## Source convention

Use exactly the same paired standalone line comments in authored Rust and Terrane:

```text
// SECURITY-BEGIN[filesystem.beneath-open]
// Explain the local invariant and why this region matters.
...implementation...
// SECURITY-END[filesystem.beneath-open]
```

Include relevant documentation comments, attributes, signatures, platform branches, and fail-closed alternatives inside the region. Markers must be paired in the same file with the same ID. Regions may nest but cannot cross; use several cohesive regions if one guarantee depends on noncontiguous code. Indentation and trailing whitespace are allowed. A case-insensitive `SECURITY-` prefix at the start of comment content reserves that comment for a marker: doc, hash, and block comments, noncanonical spelling, and trailing prose are errors. An ordinary prose mention elsewhere in a comment is not a marker. Marker-looking text in quoted/raw literals or text blocks is ignored. Rust block comments nest; Terrane block comments end at the first terminator. The extractor is not a language validator and does not replace the compiler.

The ID is the record's global `id`, not a filename or implementation-language identity. Do not reuse a deleted ID for a different guarantee. Place markers in authored sources; the current compiler does not propagate these maintenance comments into generated Rust. Changes to relevant lowering templates are tracked as dependencies instead.

## Validate and report changes

From the repository root, with a current installed Terrane compiler and Git:

```sh
terrane run tools/security-contracts -- check
terrane run tools/security-contracts -- check --json
terrane run tools/security-contracts -- impact --base main
terrane run tools/security-contracts -- impact --base main --json --fail-on-impact
terrane test tools/security-contracts
```

`--root DIRECTORY` selects a trusted Git checkout without changing the process working directory. `impact` requires an explicit base commit/ref; choose the actual review baseline, such as the pull request merge base. The comparison includes staged and unstaged changes plus nonignored untracked files. Ignored local applications, packages, the standalone manual, and generated build trees are outside discovery. Repository paths, records, and source files read by the checker must be UTF-8; diff bodies need not be. Linked sources are skipped, cannot satisfy enforcement, and are not read. Linked records are rejected. The checkout must stay stable while being inspected: the checker is not a sandbox against concurrent path replacement.

`check` validates record structure, duplicate IDs, referenced file existence, marker pairing, unknown contracts, and the correspondence between enforcement files and marked regions. It prints the contract inventory and inclusive marker locations. `impact` validates both snapshots and reports affected contracts using both old and new region coordinates. Removing a contract and all its markers still reports the old contract. Removing only one side of a declaration fails validation. Marker changes themselves count as changes to a region. Whole-file dependency and evidence changes conservatively flag their associated contracts; these files need not have source markers. JSON output has `format: 1`, `command`, resolved `base` (empty for check), and a `contracts` sequence with IDs, record paths, titles, audiences, regions, and review reasons. For a removed contract, locations refer to the base snapshot.

Base source discovery uses an ASCII case-insensitive Git content search before fetching candidate blobs, so malformed lowercase markers are still checked. Current source files are scanned directly, including untracked files.

Exit status is 0 for a successful report, 1 for invalid input or validation failure, and 2 when `--fail-on-impact` finds affected contracts. Errors go to stderr. These commands do not execute the evidence commands, grant security approval, enforce reviewer identity, or modify Git state. A CI/review workflow can use exit 2 to request the recorded review; it must not treat an empty report as proof that a change has no security consequences.

## Dedicated record format

Records use `format: 1`, `kind: security-contract`, the normal manual envelope, and a required structured `security` mapping. The [shared manual format specification](https://github.com/streaky/terrane-manual/blob/main/spec.md) owns this extension. Older processors reject the new kind rather than silently treating its security facts as an ordinary topic.

| Security field | Meaning |
|---|---|
| `audiences` | A nonempty, duplicate-free list of `core`, `application`, or both as separate entries. |
| `invariant` | The guarantee that must remain true, as nonempty Markdown prose. |
| `threats` | A nonempty list of threat descriptions. |
| `assumptions` | A nonempty list of preconditions and trusted dependencies. |
| `limitations` | A nonempty list of exclusions, platform restrictions, and evidence gaps. |
| `guidance` | Nonempty application/developer guidance. |
| `review.considerations` | A nonempty list of required review considerations. |
| `files` | Exact repository-relative `path`, `role`, and plain-text `detail` entries. |

Sequence descriptions are nonempty prose strings. Security prose cannot contain fenced/indented code or raw HTML; executable or preformatted material belongs in ordinary typed manual examples. Roles remain `enforcement`, `dependency`, and `evidence`. Paths are unique exact files, never globs or parent traversal. Enforcement files must be Rust or Terrane source, and at least one enforcement and one evidence file are required. A file containing implementation and tests can be listed once as enforcement, with its tests explained in `detail`; additional evidence must still be listed.

`documentation` owns title, summary, search tags, and optional explanatory blocks/sections. Tags do not encode audiences. The separate Terrane docs converter implements this format (commit `a38c7510c0`): it derives labelled sections, prominent limitations, an explicit no-audit/certification notice, and an accessible file table. JSON preserves the authored security object. Ordinary explanatory content follows the generated security material. Converter support is independent of whether a publication imports these records.

Generated page-local section IDs are `security-overview`, `security-invariant`, `security-threats`, `security-assumptions`, `security-limitations`, `security-guidance`, `security-review`, and `security-files`. Authored sections and blocks must not collide with them. Repository paths in `security.files` describe code ownership; publication renderers do not treat them as assets to open beneath the manual root.

The standalone checker validates the security schema, complete common metadata for active contracts, and code/evidence associations. The docs converter independently validates ordinary documentation blocks, prose safety, anchors, and reference resolution when converting a publication that includes the record. The local active registry requires current lifecycle status, a quoted `since` version, null migration fields, and empty specification/conformance provenance plus null compiler provenance; evidence lives in `security.files`. Unknown security fields, duplicate YAML keys, empty required content, invalid audiences/roles, and malformed paths are errors.

The old topic/seven-section encoding is not accepted. Both snapshots used by `impact` must use a schema understood by the current checker; a baseline predating security records is also valid. Format migration does not introduce automatic manual imports or version selection.

## Review discipline

When a report names a contract, read its invariant and limitations, assess the actual dependency impact, review the affected source, and run the relevant evidence. Record the review disposition through the project's review process; tags alone are not approval. Preserve accepted limitations honestly rather than implying that registering a contract closes its evidence gaps.

New authority, untrusted-input handling, interpreter/query construction, native boundaries, and new safety claims also need security consideration even when no existing region is touched. The inventory marks known territory; it is not a complete call graph or a security audit. Dependency changes can affect guarantees transitively beyond the explicitly recorded file set.

The tooling is Terrane in [`tools/security-contracts/`](../../tools/security-contracts/). A narrow inline Rust boundary invokes Git with explicit argv because production `/core/process` does not yet expose subprocess capture. Marker recognition, record validation, diff parsing, and review policy remain Terrane. No Python runtime or manual checkout is needed.

The Git bridge strips repository- and pathspec-selection environment overrides. It buffers subprocess output before rejecting stdout above 64 MiB (16 MiB for a base blob); those checks are not memory bounds. Working-tree file reads use the filesystem's bounded read instead.

On Linux the Rust CLI integration suite runs the Terrane security-tool unit suite with the compiler built by Cargo. A focused pass is not a substitute for the repository's measured workspace evidence required before handing compiler/runtime changes over for merge review.
