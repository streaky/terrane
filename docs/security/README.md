# Security contracts

This directory owns security-sensitive maintenance contracts, versioned with Terrane's implementation. YAML files are canonical manual-format-1 topic records. They are not copies of the standalone manual and do not require its checkout. A later manual build can explicitly include these records under Internals → Security contracts; this change does not implement manual assembly or choose manual versioning.

A contract identifies a guarantee, its threat boundary, assumptions, limitations, application guidance, implementation, evidence, and review considerations. Registration means **security-sensitive**, not audited, certified, or proven secure. The pilot is [directory-handle confined opens](filesystem-beneath-open.yaml).

## Source convention

Use exactly the same paired standalone line comments in authored Rust and Terrane:

```text
// SECURITY-BEGIN[filesystem.beneath-open]
// Explain the local invariant and why this region matters.
...implementation...
// SECURITY-END[filesystem.beneath-open]
```

Include relevant documentation comments, attributes, signatures, platform branches, and fail-closed alternatives inside the region. Keep the region cohesive rather than tagging an entire subsystem by default. Markers must be paired in the same file with the same ID. Regions may nest but cannot cross; one contract can have several regions in several files. Indentation and trailing whitespace are allowed; other spelling changes and trailing prose are errors. Marker-looking text in quoted/raw literals, text blocks, or block comments is not a marker. The extractor understands Rust characters/lifetimes, raw/byte strings, nested block comments, Terrane interpolation, tail/block text, and indentation-delimited native Rust regions. It is not a language validator and does not replace the compiler.

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

`--root DIRECTORY` selects another Git working tree without changing the process working directory. `impact` requires an explicit base commit/ref and resolves it to a commit ID. Choose the actual review baseline, such as the pull request merge base; the tool does not infer branch policy. The comparison includes staged and unstaged changes plus nonignored untracked files. Ignored local applications, packages, the standalone manual, and generated build trees are outside discovery. Git and record paths must be UTF-8; unsupported input is an error, not a skipped file.

`check` validates record structure, duplicate IDs, referenced file existence, marker pairing, unknown contracts, and the correspondence between enforcement files and marked regions. It prints the contract inventory and inclusive marker locations. `impact` validates both snapshots and reports affected contracts using both old and new region coordinates. Removing a contract and all its markers still reports the old contract. Removing only one side of a declaration fails validation. Marker changes themselves count as changes to a region. Whole-file dependency and evidence changes conservatively flag their associated contracts; these files need not have source markers. JSON output has `format: 1`, `command`, resolved `base` (empty for check), and a `contracts` sequence with IDs, record paths, titles, audiences, regions, and review reasons. For a removed contract, locations refer to the base snapshot.

Exit status is 0 for a successful report, 1 for invalid input or validation failure, and 2 when `--fail-on-impact` finds affected contracts. Errors go to stderr. These commands do not execute the evidence commands, grant security approval, enforce reviewer identity, or modify Git state. A CI/review workflow can use exit 2 to request the recorded review; it must not treat an empty report as proof that a change has no security consequences.

## Record profile

The security profile deliberately uses existing manual fields rather than inventing an incompatible envelope or custom YAML tags. Records are a strict subset of manual format 1:

- `format: 1`, a stable lowercase dot-separated `id`, `kind: topic`, and `surface: null`.
- A complete current, experimental `lifecycle` with a quoted `since` version and null migration fields.
- Complete `documentation` with a text title, summary, tags, sections, and empty `see-also` (until publication assembly provides a reference registry).
- The tag `security-contract`, plus `security-core`, `security-application`, or both. Additional search tags are allowed. Core protects the language/toolchain host or its safety guarantees; application identifies guarantees developers rely on in their programs. Neither is a severity score.
- Exactly seven sections with stable IDs: `invariant`, `threats`, `assumptions`, `limitations`, `guidance`, `review`, and `files`. Each has a title and one block. Prose sections use a prose-only `markdown` block, without fenced/indented code or HTML code blocks.
- `files` contains the typed `security-files` table, with `path`/`role` code columns and a `detail` text column. Paths are unique, exact, repository-relative files, never globs or parent traversal. Roles are `enforcement` (Rust/Terrane regions), `dependency` (whole-file impact), and `evidence` (whole-file impact). At least one enforcement and one evidence file are required. A file containing implementation and tests can be listed once as enforcement, with its tests explained in `detail`; additional evidence must still be listed.
- `provenance` has empty specification/conformance sequences and null compiler provenance. Evidence associations belong in the file table; these records do not pretend to be compiler-generated entities or compiler conformance case registries.

Unknown profile fields, duplicate YAML keys, aliases beyond the configured zero-alias budget, missing required content, and invalid file roles fail validation. This validator checks the security authoring profile, not every possible manual-format record. Because the profile only uses existing topic/section/table/tag structures, future manual assembly can consume the authored records directly rather than translate a parallel registry.

## Review discipline

When a report names a contract, read its invariant and limitations, assess the actual dependency impact, review the affected source, and run the relevant evidence. Record the review disposition through the project's review process; tags alone are not approval. Preserve accepted limitations honestly rather than implying that registering a contract closes its evidence gaps.

New authority, untrusted-input handling, interpreter/query construction, native boundaries, and new safety claims also need security consideration even when no existing region is touched. The inventory marks known territory; it is not a complete call graph or a security audit. Dependency changes can affect guarantees transitively beyond the explicitly recorded file set.

The tooling is Terrane in [`tools/security-contracts/`](../../tools/security-contracts/). A narrow inline Rust boundary invokes Git with explicit argv because production `/core/process` does not yet expose subprocess capture. Marker recognition, record validation, diff parsing, and review policy remain Terrane. No Python runtime or manual checkout is needed.
