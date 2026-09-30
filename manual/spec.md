# Terrane manual IR and authoring format

This document defines a structured source format for producing Terrane manuals. The canonical authoring unit is a YAML record, normally one file per reference surface, conceptual page, tutorial chapter, or appendix. Human prose is written as Markdown inside YAML block scalars. Facts and content units that need stable identity, validation, reuse, or renderer-specific presentation remain structured rather than being inferred from Markdown.

The intended documentation set combines four forms:

- a browsable language reference, where syntax and semantic rules can be read independently and linked precisely, in the style of the Rust Reference;
- an entity reference, where namespaces, descriptors, classes, interfaces, traits, functions, methods, properties, constants, and diagnostics have predictable generated synopses, in the style of the PHP manual;
- an internals reference, where durable compiler, lowering, runtime, generated-code, projection, tooling, cache, artifact, diagnostic, and host-ABI contracts can be located without treating their implementation as an importable Terrane surface;
- a tutorial book, where concepts are introduced in an authored learning order and structured examples, exercises, figures, callouts, and cross-references are rendered as a continuous narrative.

This format is the canonical authoring and publication system for the current Terrane manual. Authority remains explicit:

- authored records under `manual/reference/` define the current documented language, standard-library, package, tooling, and compiler-internals contracts;
- executable conformance cases define what the current compiler supports;
- the compiler-emitted reference surface describes facts present in a particular compiler build;
- lifecycle and explanatory text remain explicit authored decisions;
- `docs/language-spec-and-compiler-architecture-draft.md` is a legacy design inventory, not an authority when it conflicts with the manual, conformance evidence, or implemented behavior.

## 1. Central model

The manual system has authored records and navigation, optional compiler inputs, and one assembled IR:

```text
compiler semantic model
        |
        v
compiler surface snapshot ---------+
                                    |
authored YAML manual records -------+--> assembled manual IR --> renderers
                                    |
manual navigation -----------------+
                                    |
release/profile configuration ------+
```

The important separation is:

```text
surface facts       compiler-owned
lifecycle decisions human/release-owned
documentation       human-owned
document structure  human-owned
provenance          shared, with field-level ownership
presentation        renderer-owned
```

Markdown is not used to encode signatures, parameters, inheritance, availability, member ownership, example execution contracts, image alternatives, or the semantic role of a callout. It is used for continuous prose, lists, inline formatting, explanations attached to structured units, warnings, migration guidance, and conceptual material.

### 1.1 Reference and book structures

The YAML envelope, documentation blocks, ID rules, cross-references, validation, and renderers are unified. `publication.kind` selects how those shared records form a manual; it does not select a separate schema dialect.

A reference publication is primarily a graph of independently useful pages and entities:

```text
reference publication
├── authored conceptual pages (`manual`, `topic`, language kinds)
├── compiler-backed entity pages
├── authored relationships and cross-references
└── generated entity, rule, term, namespace, and diagnostic indexes
```

Reference navigation makes that graph browsable but does not define the semantics or ownership of its records. A reader may enter an entity or topic directly from search, follow a relationship, or browse an index without reading preceding pages.

A book publication is primarily an authored reading sequence:

```text
book publication
├── unnumbered front matter (`topic`)
├── optional unnumbered introduction (`chapter`)
├── numbered body (`chapter`)
├── lettered back matter (`appendix`)
└── links into imported reference publications

chapter or appendix
├── title and summary
├── page-level introductory blocks
└── ordered section tree
    ├── prose and tables
    ├── examples and source annotations
    ├── images and callouts
    └── exercises
```

Book navigation defines progression, displayed numbering, and previous/next relationships. A chapter is expected to make sense in that progression, whereas a reference page is expected to remain useful when opened independently. A book may place one unnumbered introductory `chapter` before its numbered body without treating that introduction as a preface. Neither distinction changes the meaning of shared blocks: an `example`, `image`, `admonition`, or internal reference has the same schema and validation contract in both publications.

A record is owned by one publication and has one record kind. A book links to an imported reference contract instead of republishing that reference page as a chapter, and a reference publication does not use chapter order to imply semantic relationships.

## 2. Design goals

The format must provide:

1. **A real manual IR.** The processor receives typed entities, document units, and relationships rather than recovering them from prose.
2. **Compiler participation.** The compiler can emit the surface it actually understands from its semantic model.
3. **Safe reconciliation.** A tool can add records and update compiler-owned fields without rewriting authored prose or silently removing entities.
4. **Readable authoring.** YAML records and their Markdown scalars remain understandable in a normal editor.
5. **Stable identity.** Entities, pages, rules, examples, exercises, figures, terms, and sections retain IDs across title changes and file moves.
6. **One system for reference and tutorial material.** Both participate in navigation, search, cross-references, profiles, validation, and rendering without forcing a tutorial to become an exhaustive reference.
7. **Explicit lifecycle.** Presence in or absence from one compiler build is not confused with a human decision to add, deprecate, or remove a public contract.
8. **Generated views.** Signatures, parameter tables, inheritance, member indexes, availability, chapter navigation, and search records come from structured data.
9. **Checkable examples.** An example states whether it is illustrative, accepted, runnable, or intentionally rejected.
10. **Accessible semantic units.** Figures, tables, callouts, source annotations, and other special displays retain their meaning independently of one visual renderer.
11. **Multiple publication formats.** HTML, Markdown, print, and machine-readable output are derived from the same assembled content.
12. **Deterministic output.** Identical inputs and tool versions produce equivalent assembled IR and rendered output.
13. **Strict validation.** Unknown fields, stale references, ambiguous identities, invalid source annotations, missing accessibility text, surface drift, and incomplete documentation fail visibly.
14. **Reusable package schema.** Compiler-owned, standard-library, source-declared, and projected dependency surfaces can use the same entity model where their contracts permit it.

The first format deliberately excludes arbitrary YAML tags, anchors and aliases, raw HTML, embedded scripts, remote includes, prose macros, executable templates, and arbitrary renderer components.

## 3. Source layout

A manual source root has this shape:

```text
reference/
├── manual.yaml
├── records/
│   ├── manual/
│   │   └── introduction.yaml
│   ├── language/
│   │   ├── lexical-structure.yaml
│   │   └── functions/
│   │       └── arguments.yaml
│   ├── api/
│   │   └── core/
│   │       ├── output/
│   │       │   └── print.yaml
│   │       └── types/
│   │           └── string/
│   │               ├── index.yaml
│   │               └── concat.yaml
│   ├── internals/
│   │   └── execution-runtime.yaml
│   └── diagnostics/
│       └── unknown-name.yaml
├── lifecycle/
│   └── renames.yaml
└── assets/
    └── object-model.svg
```

A book uses the same source-root contract but may organize records by reading role:

```text
tutorial-book/
├── manual.yaml
├── records/
│   ├── preface.yaml
│   ├── chapters/
│   │   ├── 01-first-taste.yaml
│   │   └── 02-values-and-names.yaml
│   └── appendices/
│       └── syntax-map.yaml
└── assets/
    └── compiler-pipeline.svg
```

Rules:

- Every record file uses lowercase kebab-case and the `.yaml` extension.
- A numbered book chapter record prefixes its descriptive filename with the zero-padded decimal `number` from `manual.yaml`, for example `01-first-taste.yaml`. The prefix keeps file views in reading order but does not participate in record identity.
- A record file defines exactly one page or entity.
- Source paths organize work but do not define identity, ownership, navigation, chapter numbering, or output URLs.
- Moving a record does not change its ID.
- `manual.yaml` defines the publication, imported publication dependencies, navigation, grouping, and numbering.
- `lifecycle/renames.yaml` records explicit identity migrations used during reconciliation when the manual has lifecycle-managed identities.
- Assets are local. A build does not fetch remote content.
- Generated compiler snapshots are build inputs, not canonical authored files under `records/`.

## 4. YAML profile

Manual records use the YAML 1.2 core schema with these restrictions:

- UTF-8 only;
- mappings, sequences, strings, integers, booleans, and `null` only;
- no custom tags;
- no anchors, aliases, or merge keys;
- no duplicate mapping keys;
- versions are quoted strings;
- unknown keys are errors;
- list-valued fields always use sequences, including a one-item list;
- source-facing code and Markdown use block scalars rather than escaped single-line strings when multiline.

Use `>-` for a short paragraph whose source may wrap and `|-` for Markdown or source code whose line structure matters:

```yaml
documentation:
  summary: >-
    Writes the canonical display of its arguments followed by one newline.

  sections:
    - id: behaviour
      title: Behaviour
      blocks:
        - type: markdown
          markdown: |-
            Arguments are evaluated from left to right.

            No separator is inserted between their displays.
```

The processor parses Markdown only in fields explicitly declared as Markdown by this format. A string field such as `symbol`, `id`, `type`, or `title` is plain text unless its schema says otherwise.

## 5. Record envelope

Every record has the same top-level envelope and canonical key order:

```yaml
format: 1
id: api.core.output.print
kind: function

surface: {}
lifecycle: {}
documentation: {}
provenance: {}
```

Top-level fields:

| Field | Required | Owner | Meaning |
|---|---:|---|---|
| `format` | yes | format | Integer record-format version. |
| `id` | yes | identity/release | Stable global manual identity. |
| `kind` | yes | identity/release | Record kind from the registry. |
| `surface` | entity kinds | compiler | Compiler-known source-facing contract. |
| `lifecycle` | yes | human/release | Publication status and compatibility history. |
| `documentation` | yes | human | Title, summary, prose, examples, exercises, figures, and authored labels. |
| `provenance` | yes | shared | Specification, compiler, and conformance evidence. |

Conceptual records such as language pages, tutorial chapters, and appendices have `surface: null`. Their explanations and authored structures live under `documentation` because they cannot be reconstructed merely by enumerating compiler symbols.

A processor must reject unknown top-level fields. A format revision may add optional fields only under the format-evolution rules in this document.

## 6. Stable IDs

An ID consists of lowercase dot-separated segments. Each segment begins with an ASCII letter and continues with lowercase ASCII letters, digits, or internal hyphens:

```text
[a-z][a-z0-9-]*(\.[a-z][a-z0-9-]*)*
```

Recommended roots:

| Root | Use |
|---|---|
| `manual` | Introduction, conventions, and manual-level pages. |
| `book` | Tutorial chapters, appendices, and other learning material. |
| `lang` | Language syntax and semantic topics. |
| `api` | Namespaces and object/API entities. |
| `tool` | Compiler, command-line, manifest, and tooling reference. |
| `diag` | Compiler diagnostics. |
| `term` | Globally defined terms. |
| `rule` | Addressable semantic rules. |

Examples:

```text
manual.introduction
book.first-taste
book.async
book.appendix.syntax-map
lang.functions.arguments
api.core.output.print
api.core.types.string
api.core.types.string.concat
diag.type.unknown-name
rule.call.arguments.left-to-right
term.callable
```

IDs are identities, not generated slugs. A source rename or title change does not automatically change an ID. If an ID itself must change, the release owner records an explicit rename and redirect.

The default output URL replaces dots with slashes and uses the renderer's configured extension. A deployment may map one configured landing ID to the publication root, but other mappings remain deterministic.

All pages, rules, and terms in a documentation set share one global ID registry. Section IDs and block IDs other than globally registered rule and term IDs are local to their page and are addressed as `page-id#local-id`.

## 7. Record kinds

Format version 1 defines:

| Category | Kinds |
|---|---|
| Organizational | `manual`, `topic`, `chapter`, `appendix`, `namespace`, `index`, `glossary` |
| Language | `syntax`, `statement`, `expression`, `operator`, `literal`, `protocol` |
| Object/API | `descriptor`, `class`, `interface`, `trait`, `function`, `method-family`, `method`, `property`, `field`, `constant` |
| Tooling | `command`, `option`, `manifest-key`, `diagnostic` |

A new kind requires a format update. Unknown kinds are errors.

Entity kinds have a non-null `surface`. Organizational and conceptual language pages have `surface: null`. `chapter` and `appendix` identify authored reading roles; their order and displayed numbering come from `manual.yaml`, not their IDs or filenames. A protocol that has a compiler-visible descriptor uses a surface; a purely explanatory protocol page does not pretend to have one.

## 8. Field ownership

Ownership is part of the format contract, not an implementation suggestion.

| Subtree or field | Owner | Sync behaviour |
|---|---|---|
| `format` | format | Never rewritten except by explicit migration. |
| `id` | release/identity | Never inferred from a changed symbol after creation. |
| `kind` | release/identity | Changed only through an explicit migration. |
| `surface` | compiler | Replaced from a matched compiler entity. |
| `lifecycle` | human/release | Never changed merely because a compiler entity appears or disappears. |
| `documentation.status` | human | Scaffold may initialize it; sync does not promote it. |
| Other `documentation` | human | Never semantically rewritten by sync. |
| `provenance.compiler` | compiler | Updated from the emitted snapshot. |
| `provenance.conformance` | compiler/tool plus review | Reconciled from verified evidence. |
| `provenance.specification` | human | Preserved and validated. |

A sync tool must preserve human-owned scalar spelling, Markdown block contents, comments, and ordering inside human-owned subtrees. It may canonicalize the compiler-owned `surface` and `provenance.compiler` subtrees.

A plain parse-and-reserialize implementation that reformats the whole file is non-conforming. The implementation must either edit a YAML concrete syntax tree or splice canonically serialized owned subtrees into the existing document without touching authored spans.

## 9. Compiler surface schema

### 9.1 Common entity fields

Every non-null `surface` begins with:

```yaml
surface:
  symbol: /core/output::print
  aliases:
    - print
  namespace: api.core.output
  owner: null
  visibility: public
  availability:
    targets:
      - all
    feature: null
  origin:
    category: compiler
    package: core
```

Fields:

| Field | Required | Meaning |
|---|---:|---|
| `symbol` | yes | Canonical Terrane-facing spelling. Never a Rust lowering name. |
| `aliases` | yes | Source-visible aliases. Empty sequence when none. |
| `namespace` | top-level entities | Namespace record ID. |
| `owner` | members | Owning descriptor, class, interface, trait, method family, or namespace ID. |
| `visibility` | yes | Canonical Terrane visibility. |
| `availability` | yes | Target and feature/profile restrictions known to the compiler. |
| `origin` | yes | Compiler, source package, dependency, adapter, or tool origin. |

Exactly one of `namespace` and `owner` is normally non-null for an API entity. Namespace root records may set both to `null`.

The compiler must derive these values from resolved semantic objects. It must not scrape Terrane source text, generated Rust, documentation comments, or demo files to infer the surface.

### 9.2 Type references

A type reference is structured rather than a presentation string:

```yaml
type:
  display: text-display
  entity: api.core.interface.text-display
  arguments: []
  nullable: false
```

Fields:

| Field | Meaning |
|---|---|
| `display` | Deterministic canonical source rendering. |
| `entity` | Principal descriptor/protocol ID when one exists. |
| `arguments` | Nested type references for constructors or composites. |
| `nullable` | Whether `none` is part of the contract, when this representation applies. |

Union, callable, reference, and other composite types should gain explicit tagged shapes rather than encoding their semantics only in `display`. `display` is for rendering and drift review; typed fields drive validation and indexing.

### 9.3 Callable signatures

Functions, methods, method families with a default invocation, class construction, and other callable entities expose signatures:

```yaml
surface:
  symbol: /core/output::print
  aliases:
    - print
  namespace: api.core.output
  owner: null
  visibility: public
  availability:
    targets:
      - all
    feature: null
  origin:
    category: compiler
    package: core

  signatures:
    - id: default
      form: call
      display: print; values...
      receiver: null
      parameters:
        - id: values
          name: values
          type:
            display: text-display
            entity: api.core.interface.text-display
            arguments: []
            nullable: false
          mode: variadic
          default: null
          passing: value
      returns:
        type:
          display: none
          entity: api.core.types.none
          arguments: []
          nullable: false
      throws: []
      contracts:
        asynchronous: false
        receiver-mutation: none
```

Signature rules:

- `id` is stable within the entity and is not derived from list position.
- `form` is `declaration`, `call`, `member`, `construct`, or `expression`.
- `display` is canonical source-facing synopsis text generated from the typed signature.
- Parameter IDs remain stable when a display name is renamed deliberately.
- Parameter `mode` is `required`, `optional`, or `variadic`.
- `default` is canonical source spelling or `null`.
- `passing` uses Terrane ownership vocabulary, not Rust ABI vocabulary.
- `returns` is always present, including `none` and `never`.
- `throws` is always a sequence. An empty sequence means the compiler has established no Terrane throwable outcome for the signature; absence of the field is invalid.
- Callable contracts retain orthogonal facts rather than collapsing them into a generic effect list.

A throwable entry has a stable local ID and a type:

```yaml
throws:
  - id: conversion-failure
    type:
      display: coercion-error
      entity: api.core.errors.coercion-error
      arguments: []
      nullable: false
```

The compiler may know the throwable type without knowing the best human explanation of its condition. That explanation belongs under `documentation.callables` keyed by signature and throwable ID.

### 9.4 Classes, interfaces, traits, and descriptors

Relationship fields are compiler-owned:

```yaml
surface:
  symbol: /example::connection
  aliases: []
  namespace: api.example
  owner: null
  visibility: public
  availability:
    targets:
      - all
    feature: null
  origin:
    category: package
    package: example

  extends: []
  implements:
    - api.core.interface.text-display
  uses: []
  construct-signature: default
  ownership: resource
```

Rules:

- A class has at most one `extends` entry; other kinds follow Terrane's object-model constraints.
- `implements` contains interface/protocol IDs.
- `uses` contains trait IDs.
- Members are discovered by reverse `owner` relationships. A class record does not duplicate a manually maintained member list.
- Construction references a signature on the class record when the class supports default invocation.
- Ownership and lifecycle facts use explicit Terrane categories.
- Inherited members remain owned by their declaring entity; generated views calculate inherited availability separately.

### 9.5 Properties, fields, and constants

A property or field surface includes its type and access contract:

```yaml
surface:
  symbol: connection.closed
  aliases: []
  namespace: null
  owner: api.example.connection
  visibility: public
  availability:
    targets:
      - all
    feature: null
  origin:
    category: package
    package: example

  value-type:
    display: bool
    entity: api.core.types.bool
    arguments: []
    nullable: false
  readable: true
  writable: false
  static: false
```

Constants additionally carry a canonical value representation only when that value is part of the source-visible contract and can be serialized without executing arbitrary code.

### 9.6 Diagnostics and tools

Compiler diagnostics, CLI commands, options, and manifest keys may be emitted through the same snapshot mechanism. Their `surface` uses kind-specific fields, for example:

```yaml
surface:
  symbol: T0001
  aliases: []
  namespace: null
  owner: null
  visibility: public
  availability:
    targets:
      - all
    feature: null
  origin:
    category: compiler
    package: terrane-compiler

  severity: error
  diagnostic-family: name-resolution
```

The explanation, rejected example, and correction remain authored documentation.

## 10. Lifecycle schema

Lifecycle is explicit and human-controlled:

```yaml
lifecycle:
  status: current
  stability: stable
  since: "0.1"
  deprecated-since: null
  removed-in: null
  replacement: null
```

Statuses:

| Status | Meaning |
|---|---|
| `planned` | Settled or exploratory design not present in the selected compiler surface. |
| `current` | Present in the selected published surface. |
| `deprecated` | Present but discouraged, with an explicit migration contract. |
| `removed` | Retained for historical lookup but absent from the selected current surface by decision. |

`status` may be `null` only in a newly generated scaffold whose
`documentation.status` is `missing`. This represents an unresolved release decision, not another
lifecycle state. It is an error in every publication profile.

`stability` is `experimental`, `stable`, or `null` for records where stability is inapplicable.

Rules:

- `current` and `deprecated` require `since`.
- `deprecated` requires `deprecated-since` and normally `replacement` or authored rationale.
- `removed` requires `removed-in` and retains its last known surface for historical rendering.
- `replacement` is a reference ID, never an external URL or prose label.
- An entity disappearing from a compiler snapshot does not change lifecycle status.
- A newly emitted entity does not become a complete public page merely because the compiler can describe it.
- Version strings are opaque quoted release identifiers interpreted by the selected release policy.

## 11. Documentation schema

### 11.1 Common fields

```yaml
documentation:
  status: complete
  title: print
  title-style: code
  summary: >-
    Writes the canonical text display of zero or more values followed by one
    newline.
  tags:
    - output
    - text-display
  callables: {}
  blocks: []
  sections: []
  see-also: []
```

Fields:

| Field | Required | Meaning |
|---|---:|---|
| `status` | yes | `missing`, `draft`, or `complete`. |
| `title` | yes except a new scaffold | Plain display title without generated chapter or appendix numbering. |
| `title-style` | yes | `text`, `code`, or another format-defined presentation enum. |
| `summary` | complete records | Short Markdown standfirst, normally one sentence. |
| `tags` | yes | Authored search terms; empty sequence when none. |
| `callables` | callable entities | Descriptions keyed to compiler-owned signature components. |
| `blocks` | no | Ordered page-level blocks rendered after the summary and before the first section; omission means empty. |
| `sections` | yes | Ordered, recursively nested authored content sections. |
| `see-also` | yes | Structured related-reference links. |

`documentation.status` is independent of lifecycle. A compiler-present entity may have missing documentation; a removed entity may retain complete historical documentation.

A public build includes only records whose lifecycle and documentation statuses satisfy its profile.

### 11.2 Callable documentation

Descriptions attach to stable signature, parameter, return, and throwable IDs without being stored inside the compiler-owned surface:

```yaml
documentation:
  callables:
    default:
      parameters:
        values: |-
          Values are evaluated from left to right and converted through their
          canonical text-display contracts.
      returns: |-
        Returns `none` after writing the line.
      throws: {}
```

Rules:

- The outer key is a signature ID.
- Parameter keys are parameter IDs, not display names.
- `returns` is Markdown.
- Throwable description keys are throwable IDs.
- Empty `throws` documentation is valid only when the compiler-owned throwable sequence is empty.
- Stale keys after a surface update are errors requiring review; the sync tool does not delete their prose.
- Missing descriptions may be warnings or errors according to the publication profile.

### 11.3 Sections and blocks
The currently supported block union is deliberately small. Use this table to
choose a block before consulting the complete definition below.

| `type` | Required distinguishing fields | Use it for |
|---|---|---|
| `markdown` | `markdown` | Ordinary prose, lists, quotations, and inline links; never headings, tables, images, or raw HTML. |
| `table` | `id`, `title`, `show-header`, `columns`, `rows` | Semantic tabular data with typed cells and stable row order. |
| `rule` | `id`, `level`, `since`, `evidence`, `markdown` | A stable normative contract that benefits from independent evidence or linking. |
| `grammar` | `id`, `notation`, `source` | Formal Terrane grammar in the selected notation. |
| `example` | `id`, `title`, `language`, `mode`, `target`, `timeout-ms`, `source`, `response`, `diagnostic`, `markdown` | Illustrative or verified source, requests, responses, and expected diagnostics. |
| `admonition` | `id`, `kind`, `title`, `markdown` | Notes, guidance, rationale, warnings, implementation details, and lifecycle callouts. |
| `term` | `id`, `name`, `definition` | A globally identified glossary definition. |
| `entity-index` | `query`, `sort`, `layout` | A generated member list queried from the assembled IR. |
| `image` | `id`, `asset`, `alt`, `caption` | A validated asset with explicit accessibility semantics. |
| `exercise` | `id`, `title`, `prompt`, `starter`, `solution` | A structured learning task with optional starter and solution content. |

Unknown block types and fields not allowed by the selected block type are
errors. This table is an index, not a substitute for the field contracts and
examples in §§11.4–11.14.


Page-level `documentation.blocks` contain introductions, learning objectives, or other content that follows the page title and summary but precedes the first titled section. Sections are ordered, may nest, and have stable local IDs:

```yaml
sections:
  - id: behaviour
    title: Behaviour
    blocks:
      - type: markdown
        markdown: |-
          The converted values are concatenated without an inserted separator.

      - type: rule
        id: rule.output.print.trailing-newline
        level: must
        markdown: |-
          Exactly one newline follows the concatenated display.
    sections:
      - id: examples
        title: Examples
        blocks:
          - type: example
            id: multiple-values
            title: Print several values
            language: terrane
            mode: run
            source: |-
              namespace reference-examples/print-values

              function main;
                print; 'answer: ', 42
            response:
              kind: stdout
              language: text
              display: on-run
              content: |-
                answer: 42
        sections: []
```

Section IDs are unique within a page regardless of nesting depth. Renderers derive semantic heading levels from the section tree; authors do not place headings in Markdown merely to simulate a subsection. The page title occupies heading level one, so a section tree may be at most five levels deep. A section's blocks precede its child sections. `blocks` and child `sections` are independently optional and omission means an empty sequence, but a complete published section must not leave both empty.

Format version 1 block types are:

| Type | Purpose |
|---|---|
| `markdown` | Ordinary explanatory Markdown. |
| `table` | Authored tabular data with typed columns and validated rows. |
| `rule` | Addressable normative rule. |
| `grammar` | Formal Terrane grammar productions. |
| `example` | Structured single-file or multi-file source example and expected result. |
| `admonition` | Semantic callout such as a note, rationale, guidance, warning, or implementation detail. |
| `image` | Local image with explicit accessibility and caption data. |
| `exercise` | Addressable reader activity. |
| `term` | Addressable term definition. |
| `entity-index` | Generated query over assembled entities. |
| `diagnostic` | Structured diagnostic condition where not represented by a dedicated record. |

Blocks are tagged unions. Fields not allowed for the selected `type` are errors. A renderer may give a block type a specialized visual treatment, but the block's meaning cannot depend on that treatment.

### 11.4 Markdown blocks

A Markdown block contains explanatory prose:

```yaml
- type: markdown
  markdown: |-
    Ordinary **Markdown** is allowed here, including lists, quotations, inline
    code, and links.
```

Markdown uses CommonMark 0.31.2 plus strikethrough. Raw HTML, Markdown headings,
Markdown image syntax, Markdown table syntax, fenced code blocks, and indented
code blocks are errors inside a newly authored Markdown block. Authored hierarchy,
images, tabular data, grammar, and preformatted source use `sections`, `image`,
`table`, `grammar`, and `example` blocks respectively. Use an `example` with
`mode: illustrative` when source or plain preformatted text needs no runner; its
language, source, files, response, and stable block ID remain available to every
renderer.

There is no generic code block in format version 1: a snippet that cannot accurately
be a `grammar` or illustrative `example` requires an explicit format extension,
never a Markdown fence. Internal references use the syntax in §12. External links
use ordinary Markdown links.

### 11.5 Tables

Authored tabular data uses a dedicated block:

```yaml
- type: table
  id: structural-words
  title: Structural words
  show-header: true
  columns:
    - id: area
      title: Area
      cell: text
    - id: words
      title: Structural words
      cell: code-list
  rows:
    - area: Namespaces and imports
      words:
        - namespace
        - from
        - import
        - as
```

`id` and `title` may be `null`. Column IDs are unique within the table and determine the keys allowed in every row. Column order determines rendered order; row order is authored and preserved. Every row must provide exactly one value for every column, and an authored table must contain at least one column and one row.

Column `title` is required even when `show-header` is `false`. A hidden header remains available to accessibility tools, non-visual renderers, exports, and processors; the flag controls its visual presentation rather than discarding its meaning.

Format version 1 cell representations are:

| Cell | YAML value | Rendering |
|---|---|---|
| `text` | string | Plain text with no Markdown interpretation. |
| `markdown` | string | Markdown inline content; block constructs are forbidden. |
| `code` | string | One code-styled value. |
| `code-list` | sequence of strings | A renderer-joined list of separately code-styled values. |

Tables do not contain nested blocks, column spans, row spans, sorting expressions, or executable values. Add those only if a concrete reference page cannot be represented clearly without them.

### 11.6 Rules

```yaml
- type: rule
  id: rule.call.arguments.left-to-right
  level: must
  since: null
  evidence:
    - calls/evaluation-order
  markdown: |-
    The receiver is evaluated before member selection, and supplied arguments
    are then evaluated exactly once from left to right.
```

Rule IDs are globally unique. `level` is `must`, `should`, or `may`. Rule prose is normative. Evidence values are configured conformance case IDs.

Not every contractual sentence needs a rule block. Use one when a fact benefits from a stable link, independent evidence, or distinct lifecycle.

### 11.7 Grammar

```yaml
- type: grammar
  id: function-declaration
  notation: terrane-ebnf
  grammar: |-
    FunctionDeclaration = [ "async" ], "function", Identifier,
                          ReturnType, ";", [ ParameterList ], Block ;
```

The local block ID produces `page-id#function-declaration`. Format version 1 uses:

```text
Production = Expression ;          definition
A, B                               sequence
A | B                              choice
[ A ]                              zero or one
{ A }                              zero or more
A, { A }                           one or more
"text"                             literal source text
TOKEN                              separately defined lexical token
(* text *)                         grammar comment
```

Precedence is grouping, repetition/option, sequence, then choice. Empty alternatives are forbidden.

### 11.8 Examples

```yaml
- type: example
  id: multiple-values
  title: Print several values
  language: terrane
  mode: run
  target: null
  timeout-ms: null
  source: |-
    namespace reference-examples/print-values

    function main;
      print; 'answer: ', 42
  files: null
  entry: null
  annotations:
    - id: call
      file: null
      lines:
        start: 4
        end: 4
      markdown: >-
        The semicolon invokes `print`; it does not terminate a statement.
  response:
    kind: stdout
    language: text
    display: on-run
    content: |-
      answer: 42
  diagnostic: null
  markdown: null
```

`language` is required and identifies the primary source language for rendering and verification. It is a lowercase identifier rather than a closed format-version enum; publication profiles register the languages and runners they support. Use `text` when no programming or data language applies. An unknown language is an error for executable modes and may fall back to plain rendering only for `illustrative`.

An example provides exactly one of `source` and `files`. `source` contains one source unit. `files` contains a non-empty ordered project:

```yaml
files:
  - path: package.toml
    language: toml
    source: |-
      [package]
      name = "work-report"
  - path: app/main.trn
    language: terrane
    source: |-
      namespace work-report/app

      function main;
        print; 'ready'
entry: package.toml
```

File paths are unique, use `/` separators, are relative to the example root, and cannot contain an empty, `.` or `..` segment. `entry` is `null` for a single source and otherwise names the file or package entry selected by the configured runner. A renderer preserves authored file order. A verifier materializes the files under one isolated temporary root.

Modes:

| Mode | Contract |
|---|---|
| `illustrative` | Rendered but not claimed as accepted by a configured runner. |
| `check` | Must pass the selected language's configured checking pipeline. |
| `build` | Must produce its configured build artifact without warnings. |
| `run` | Must build and run successfully. |
| `reject` | Must fail with the stable code in `diagnostic`. |

An example may carry a structured response:

```yaml
response:
  kind: stdout
  language: text
  display: on-run
  content: |-
    answer: 42
```

Response `kind` is `stdout`, `diagnostic`, `source`, or `text`. Response `language` is required under the same rules as example source language, including explicit `text`. Display is `shown`, `on-run`, or `hidden`:

- `shown` renders the response immediately;
- `on-run` keeps it out of the initial presentation and displays it when the reader runs the example; a non-interactive renderer provides an equivalent disclosure;
- `hidden` retains the response for verification but does not publish it.

For a `run` example, a `stdout` response must match normalized standard output exactly. A `reject` example still requires the stable code in `diagnostic`; a rendered diagnostic response supplements rather than replaces that stable contract. Other response kinds are validated by the runner selected for the source language and publication profile.

Source annotations are ordered and attach authored explanation to an inclusive source-line range. `file` is `null` for a single source and required for a multi-file example. The range must be within the selected source and `start` must not exceed `end`. Annotation IDs are unique within the page. Overlapping ranges are allowed. Visual renderers may place annotations beside highlighted lines; linear, print, and Markdown renderers emit the same annotations after the example with explicit file and line labels.

Rules:

- `reject` requires `diagnostic`; other modes forbid it.
- A response is optional. When present, all four response fields are required.
- `markdown` provides optional explanation associated with the example.
- `annotations` is optional and omission means an empty sequence.
- Examples are self-contained source units or projects in format version 1; one example cannot import the files of another.
- The processor never executes `illustrative` examples.
- Executable modes require a configured runner for `language`.
- Verification uses the real language pipeline, not a documentation-specific parser.

### 11.9 Admonitions

```yaml
- type: admonition
  id: why-explicit-calls
  kind: rationale
  title: Why calls use `;`
  markdown: |-
    The call marker keeps selection and invocation visibly distinct.
```

`id` and `title` may be `null`. Kinds are `note`, `tip`, `important`, `guidance`, `rationale`, `warning`, `implementation`, `experimental`, and `deprecated`. `rationale` explains why Terrane or its tooling makes a design choice; `guidance` identifies an authored practice such as a “Terrane style” recommendation. Status and lifecycle remain structured fields; an admonition cannot change them. Every renderer supplies a textual label for the kind even when it also uses colour, an icon, or distinctive placement.

### 11.10 Terms

```yaml
- type: term
  id: term.callable
  name: callable
  markdown: |-
    A **callable** is an object that supports default invocation through the
    call marker.
```

Term IDs are globally unique and share the documentation-set registry.

### 11.11 Entity indexes

```yaml
- type: entity-index
  query:
    owner: api.core.types.string
    kinds:
      - method
      - property
    statuses:
      - current
      - deprecated
  sort: symbol
  layout: table
```

At least one query constraint is required. Supported sort modes are `symbol`, `title`, and `manual`; layouts are `table`, `list`, and `compact`. Results come from the assembled IR. Authors do not maintain a second member list in Markdown.

### 11.12 Related references

```yaml
see-also:
  - id: lang.text-display
    label: Text display
  - id: api.core.types.string.concat
    label: null
```

`label: null` uses the target's title. The processor validates every target after profile selection.


### 11.13 Images

```yaml
- type: image
  id: native-pipeline
  asset: compiler-pipeline.svg
  alt: >-
    Terrane source passes through checking, Rust lowering, Cargo, and rustc
    before becoming a native executable.
  decorative: false
  caption: >-
    The visible path from Terrane source to a native program.
```

`asset` is relative to the source root's `assets/` directory. `id` and `caption` may be `null`; `caption` is Markdown inline content. A non-decorative image requires non-empty plain-text `alt`. A decorative image requires `alt: null` and has no semantic information that is absent from adjacent prose. The processor validates that the asset exists, remains inside the configured asset root, and has a permitted media type. Renderers preserve the distinction among alternative text, visible caption, and surrounding prose.

### 11.14 Exercises

```yaml
- type: exercise
  id: change-the-greeting
  title: Change the greeting
  markdown: |-
    Change the greeting so that it prints your name or the name of a project.
    Check the file before running it.
```

`id` is required and unique within the page. `title` may be `null`; `markdown` is the complete reader-facing prompt and may contain fenced illustrative code. Exercise numbering follows depth-first authored block order and is not part of the stable ID or title. Solutions and generated answer collections are deferred until the manual contains authored solutions whose requirements can determine their schema.

## 12. Markdown cross-references

Markdown fields use one minimal custom inline syntax:

```markdown
See [[api.core.output.print]].
See [[api.core.output.print|`print`]].
See [[lang.functions.arguments#evaluation-order|evaluation order]].
See [[rule.call.arguments.left-to-right]].
```

The target before `|` is a global ID with an optional local `#section-or-block`. The optional label is Markdown inline content. Reference links cannot nest. Literal `[[` is escaped as `\[[`.

The default label is:

- the page title for a page;
- the canonical code-styled ID or configured short label for a rule;
- the defined term name for a term;
- the authored title for a local section, example, exercise, image, or admonition;
- the local ID when an addressable unit has no authored title.

Unresolved targets, ambiguous redirects, filtered required targets, and references to non-public local IDs are errors. Normal Markdown links must not point to a manual YAML source file; authors use stable `[[...]]` identities rather than source paths.

## 13. Provenance

```yaml
provenance:
  specification:
    - document: language-spec
      section: "9.6"
  conformance:
    - output/print
  compiler:
    semantic-id: core.output.print
    surface-digest: sha256:...
```

Ownership:

- `specification` is authored and validated against a configured source revision;
- `conformance` is reconciled from explicit case associations and reviewed evidence;
- `compiler` is emitted and updated by the compiler/sync tool;

A compiler digest detects drift; it is not an entity identity. A digest change triggers semantic comparison of the structured surface.

A complete current public entity should have authoritative specification or package-contract provenance and executable evidence where the contract is implemented and testable. The publication profile decides which omissions are fatal.

## 14. Conceptual language pages

A conceptual page uses the same envelope without inventing a compiler surface:

```yaml
format: 1
id: lang.functions.arguments
kind: topic

surface: null

lifecycle:
  status: current
  stability: stable
  since: "0.1"
  deprecated-since: null
  removed-in: null
  replacement: null

documentation:
  status: complete
  title: Function arguments
  title-style: text
  summary: >-
    Function calls bind positional, named, defaulted, and variadic arguments
    according to one deterministic order.
  tags:
    - calls
    - parameters
  sections:
    - id: evaluation-order
      title: Evaluation order
      blocks:
        - type: rule
          id: rule.call.arguments.left-to-right
          level: must
          since: null
          evidence:
            - calls/evaluation-order
          markdown: |-
            The receiver is evaluated before member selection, and supplied
            arguments are evaluated exactly once from left to right.
    - id: examples
      title: Examples
      blocks: []
  see-also:
    - id: api.core.types.function
      label: Function objects

provenance:
  specification:
    - document: language-spec
      section: "13"
  conformance:
    - calls/evaluation-order
  compiler: null
```

Language pages can still be updated with compiler assistance: the processor can validate grammar, examples, diagnostics, and conformance links. The compiler does not own their explanatory rule text merely because it implements those rules.

### 14.1 Tutorial chapters and appendices

A tutorial chapter uses the same envelope and documentation blocks while remaining independent of compiler-owned surface data:

```yaml
format: 1
id: book.first-taste
kind: chapter

surface: null

lifecycle:
  status: current
  stability: experimental
  since: "0.1"
  deprecated-since: null
  removed-in: null
  replacement: null

documentation:
  status: complete
  title: A First Taste of Terrane
  title-style: text
  summary: >-
    Builds and runs a first Terrane program and introduces the compiler
    development loop.
  tags:
    - introduction
    - compiler
  blocks:
    - type: markdown
      markdown: |-
        This chapter takes the shortest route through building, checking, and
        running a complete Terrane program.
  sections:
    - id: first-program
      title: Your first program
      blocks:
        - type: markdown
          markdown: |-
            Create a file named `hello.trn` with this source:
        - type: example
          id: hello
          title: Hello from Terrane
          language: terrane
          mode: run
          target: null
          timeout-ms: null
          source: |-
            namespace hello

            function main;
              print; 'Hello from Terrane!'
          files: null
          entry: null
          annotations: []
          response:
            kind: stdout
            language: text
            display: shown
            content: |-
              Hello from Terrane!
          diagnostic: null
          markdown: null
      sections: []
  see-also:
    - id: lang.functions.declarations
      label: Function declarations and references

provenance:
  specification: []
  conformance: []
  compiler: null
```

`chapter` and `appendix` records use authored order and numbering from `manual.yaml`. A book's unnumbered front or back matter normally uses `topic` records; one unnumbered `chapter` may serve as the book's introduction before the numbered body. Page-level `documentation.blocks` hold the opening prose before the first titled section; the section tree then preserves the chapter's heading hierarchy. Book prose is explanatory rather than normative unless it contains a structured `rule` block. Tutorial wording may introduce an approachable partial model and link to a more complete reference contract, but executable examples still describe their verification status honestly. When a publication imports another, its records can link directly to the imported publication's pages, sections, and blocks through the shared global ID registry.

## 15. Complete entity example

A complete `print` record illustrates how structured surface and Markdown documentation join:

```yaml
format: 1
id: api.core.output.print
kind: function

surface:
  symbol: /core/output::print
  aliases:
    - print
  namespace: api.core.output
  owner: null
  visibility: public
  availability:
    targets:
      - all
    feature: null
  origin:
    category: compiler
    package: core
  signatures:
    - id: default
      form: call
      display: print; values...
      receiver: null
      parameters:
        - id: values
          name: values
          type:
            display: text-display
            entity: api.core.interface.text-display
            arguments: []
            nullable: false
          mode: variadic
          default: null
          passing: value
      returns:
        type:
          display: none
          entity: api.core.types.none
          arguments: []
          nullable: false
      throws: []
      contracts:
        asynchronous: false
        receiver-mutation: none

lifecycle:
  status: current
  stability: stable
  since: "0.1"
  deprecated-since: null
  removed-in: null
  replacement: null

documentation:
  status: complete
  title: print
  title-style: code
  summary: >-
    Writes the canonical text display of zero or more values followed by one
    newline.
  tags:
    - output
    - text-display
  callables:
    default:
      parameters:
        values: |-
          Values are evaluated from left to right and converted through their
          canonical text-display contracts.
      returns: |-
        Returns `none` after writing the line.
      throws: {}
  sections:
    - id: behaviour
      title: Behaviour
      blocks:
        - type: rule
          id: rule.output.print.concatenation
          level: must
          since: null
          evidence:
            - output/print
          markdown: |-
            Converted values are concatenated without an inserted separator,
            and exactly one newline follows the result.
    - id: examples
      title: Examples
      blocks:
        - type: example
          id: multiple-values
          title: Print several values
          language: terrane
          mode: run
          target: null
          timeout-ms: null
          source: |-
            namespace reference-examples/print-values

            function main;
              print; 'answer: ', 42
          response:
            kind: stdout
            language: text
            display: on-run
            content: |-
              answer: 42
          diagnostic: null
          markdown: null
  see-also:
    - id: lang.text-display
      label: Text display

provenance:
  specification:
    - document: language-spec
      section: "9.6"
  conformance:
    - output/print
  compiler:
    semantic-id: core.output.print
    surface-digest: sha256:example
```

IDs and case names in this example illustrate the format and become binding only when those registries exist.

## 16. Manual navigation

`manual.yaml` defines publication behavior, dependencies, and authored navigation separately from entity relationships:

```yaml
format: 1
publication:
  id: reference
  kind: reference
imports: []
title: Terrane Reference
landing: manual.introduction
navigation:
  - source: records/manual/introduction.yaml
  - group: Language
    numbering: none
    children:
      - source: records/language/lexical-structure.yaml
      - source: records/language/functions/arguments.yaml
  - group: Core API
    numbering: none
    children:
      - source: records/api/core/output/index.yaml
      - source: records/api/core/types/string/index.yaml
  - generated:
      title: Diagnostics
      query:
        kinds:
          - diagnostic
        statuses:
          - current
      sort: symbol
```

Publication IDs use the stable-ID segment syntax from §6. `publication.kind` is `reference` or `book`. A configured documentation set maps publication IDs to source roots and output bases; records and manifests never contain host paths or deployment URLs. `imports` names other publications whose included IDs may be referenced. Imports are acyclic, do not merge the imported publication's navigation into the importing publication, and do not republish imported pages. The processor resolves a cross-publication link using the imported publication's configured output base.

For compatibility with reference roots authored before publications were named, omission of `publication` means `{ id: reference, kind: reference }` and omission of `imports` means an empty sequence. New manifests write both fields explicitly.

A book imports the reference registry and uses navigation groups to supply visible chapter and appendix labels. During a source migration, navigation may load canonical YAML records, transitional Markdown pages, and unpublished plans:

```yaml
format: 1
publication:
  id: tutorial
  kind: book
imports:
  - reference
title: The Terrane Book
landing: book.preface
navigation:
  - group: Front matter
    numbering: none
    children:
      - source: records/preface.yaml
        contents: []
      - source: records/chapters/introduction.yaml
        contents:
          - section: audience
            title: Who this book is for
          - section: examples
            title: How to read the examples
  - group: Chapters
    numbering: decimal
    children:
      - number: 6
        source: legacy/06-complete-program.md
        id: book.complete-program
        kind: chapter
        title: Building a Complete Program
        contents:
          - section: starting-from-a-user-story
            title: Starting from a user story
          - section: dividing-the-program-into-functions
            title: Dividing the program into functions
      - number: 15
        planned:
          id: book.larger-program
          kind: chapter
          title: A Larger Program, Step by Step
          contents:
            - section: application
              title: Choosing a manageable application
  - group: Appendices
    numbering: upper-alpha
    children:
      - number: "C"
        planned:
          id: book.appendix.syntax-map
          kind: appendix
          title: A Reader's Syntax Map
          contents:
            - section: bindings
              title: Bindings and assignment
```

`source` is a publication-root-relative authoring path. Its extension selects how the page is loaded:

- A `.yaml` source names one canonical record. The processor obtains the page `id`, `kind`, title, and complete section tree from that record. The navigation child does not repeat `id`, `kind`, or title.
- A `.md` source names one explicitly transitional book page. Its navigation child also supplies `id`, `kind`, title, and `contents` because those identities are not represented by the legacy Markdown source. `kind` is `topic`, `chapter`, or `appendix`.
- Any other source extension is an error.

On a source entry, `contents` is an ordered expanded-contents list. Book source entries require it. A reference YAML source may omit it when the manifest does not need to repeat the page outline. Each item has `section` and `title`. For a YAML record, `section` names a top-level local section, `title` must exactly match its title, and a present sequence lists every top-level section exactly once in record order. For a Markdown source, IDs follow the local-ID rules and are assigned to its H2 sections; the H2 headings must exactly equal the required `contents` titles in order.

A transitional Markdown H1 begins with the entry's explicit number followed by `. ` and the authored title, or contains just the title for an unnumbered page. Its body is CommonMark 0.31.2 plus strikethrough and may retain ordinary Markdown headings, tables, images, and fenced examples during migration; it is not required to conform to the YAML record/block schema until cutover. Raw HTML, unsafe URLs, and paths outside the publication root remain errors.

`planned` reserves intended book structure without publishing a page. Its mapping contains `id`, `kind`, title, and `contents`, has no source or record, and is excluded from expanded navigation, previous/next links, search, links, and rendered output. Its `contents` items use the same `section` and `title` shape and reserve those local IDs immediately. A planning view may display the entry.

Migrating a Markdown source to the canonical IR is a clean cutover: transfer its page ID, title, and content IDs into one YAML record; change `source` to that record path; remove the duplicated `id`, `kind`, and title; and delete the Markdown file. The manifest remains the complete book outline while records remain the canonical home of migrated page structure.

Rules:

- `landing` is a global ID and must resolve to one published source entry in the current publication.
- Every navigation child is exactly one of `source`, `planned`, `group`, or `generated`; fields from different alternatives cannot be combined.
- A source path names exactly one file inside the current publication root. Two entries cannot name the same source.
- YAML source entries are allowed in every publication. Markdown sources and `planned` entries are allowed only in a `book` publication.
- A `group` is a label and does not create a page.
- Group `numbering` is `none`, `decimal`, or `upper-alpha`; omission means `none`. A page-bearing child of a `none` group omits `number`. Every page-bearing child of a numbered group provides it explicitly.
- In a `decimal` group, `number` is a positive integer. In an `upper-alpha` group, it is an uppercase ASCII sequence such as `A` or `AA`. Numbers are unique and strictly increase in authored child order, including planned pages; gaps are permitted so a publication excerpt or reserved position does not require false entries.
- In a `book` publication, every source or plan in a decimal-numbered group has `chapter` kind and every source or plan in an `upper-alpha` group has `appendix` kind. YAML sources obtain that kind from their record; Markdown sources and plans declare it. Unnumbered front and back matter normally uses `topic` records; one unnumbered `chapter` may appear before the first decimal-numbered group as the book introduction.
- A `reference` publication does not own `chapter` or `appendix` records.
- Displayed numbers are navigation metadata, not identity, and are not included in canonical record or manifest titles.
- A `generated` item declares a deterministic query over the current publication.
- Every published non-member page must be reachable once from its publication's navigation unless marked as intentionally index-only by the format.
- Member pages may be reached through generated owner indexes without all appearing in the global sidebar.
- Navigation order does not imply namespace, ownership, inheritance, or lifecycle.
- Previous/next links follow the current publication's expanded published navigation order.
- Duplicate global IDs across YAML records, transitional Markdown sources, planned entries, or the transitive publication import graph are errors.

### 16.1 Generated catalog

`generated/catalog.yaml` is a deterministic discovery view over every published YAML record. It is derived from the publication manifests and records; authors never maintain catalog descriptions or outlines separately.

Catalog format 2 includes the record's authored summary and an ID-and-title projection of its complete section tree:

```yaml
format: 2
generated: true
records:
  - id: lang.functions.declarations
    title: Function declarations and references
    summary: >-
      Defines top-level functions and distinguishes declaring a function
      from invoking it.
    kind: topic
    manual: reference
    group: Language
    position: 3
    source: reference/records/language/functions/declarations.yaml
    sections:
      - id: declaration
        title: Declaring a function
        sections: []
      - id: returns
        title: Returning a value
        sections: []
```

`summary` is copied from `documentation.summary`. `sections` preserves authored order and recursively contains only each section's stable `id`, authored `title`, and child `sections`; blocks and prose remain in the source record. The catalog also carries lifecycle, documentation status, surface, provenance, and generated declaration synopsis fields used by lookup tooling.

The catalog is a disposable lookup artifact, not another authoring source. Its generation metadata identifies the generator and exact input digest. Regeneration must produce byte-identical output for byte-identical manifests and records, and validation fails when the checked-in artifact is stale.

### 16.2 YAML formatting

Every published YAML manifest and record is formatted by Prettier v3.9.8. The generator first runs Prettier's YAML parser with an 80-character print width, which normalizes YAML syntax, folded values, indentation, quoting, and line endings.

Prettier deliberately treats YAML literal scalars as opaque. The generator therefore separately runs Prettier's Markdown parser over each authored `markdown` literal scalar, with Markdown `print-width` 80 and `prose-wrap` `always`, then runs the YAML parser again to stabilize the enclosing YAML. This reflows document prose while preserving fenced source blocks exactly; a deliberately long source line inside a fenced example or one unbreakable inline literal is exempt from the prose width target.

`python tools/generate_manual_catalog.py` runs this formatter pipeline over all manifest-derived YAML inputs before validating records and refreshing generated artifacts. This is transactional: it snapshots the authored YAML first and restores every input unchanged if formatting exposes a parse or generator validation failure. A formatting failure therefore never leaves a partially formatted invalid manual.

`python tools/generate_manual_catalog.py --check` formats the same manifest-derived inputs before checking generated artifacts. It leaves successful formatting changes in place, then fails when those changes make a generated artifact stale; rerun the normal command to refresh it. Generated files are not formatter inputs.

## 17. Compiler snapshot

The compiler emits a deterministic snapshot from the resolved semantic model:

```yaml
format: 1
compiler-version: "..."
profile:
  target: native
  features: []
entities:
  - semantic-id: core.output.print
    reference-id: api.core.output.print
    kind: function
    surface: {}
    provenance: {}
```

Requirements:

- Emission reuses the same semantic model as `check`, `rust`, `build`, and `run`.
- It does not create a command-specific parser or resolver.
- Every emitted entity has a stable compiler semantic ID.
- Compiler-owned reference IDs are assigned deliberately in compiler metadata; they are not regenerated from display symbols on each run.
- Entities are sorted by reference ID.
- Maps with semantically unordered content use canonical key order.
- No wall-clock time, host path, random ID, or directory traversal order enters the snapshot.
- Target/profile selection is explicit and recorded.
- Projected dependency entities retain their Terrane projection spelling and package provenance.
- Unsupported or unrepresentable dependency surfaces are omitted with structured reasons rather than emitted as plausible incomplete entities.

A snapshot describes one selected compiler/profile surface. It is not itself the historical manual and cannot make lifecycle decisions.

## 18. Reconciliation

A conceptual command such as:

```text
terrane reference sync <snapshot> <reference-root>
```

compares the compiler snapshot with canonical records.

Default invocation is read-only and emits a semantic change report. A separate `--write` action applies reviewable changes.

### 18.1 Matching

Matching order is:

1. exact stable reference ID;
2. explicit rename mapping in `lifecycle/renames.yaml`;
3. otherwise unmatched.

Symbol similarity is diagnostic evidence only. The tool must not infer a rename solely because an old and new symbol look alike.

### 18.2 Newly emitted entity

For an unmatched compiler entity, sync may create a scaffold:

```yaml
format: 1
id: api.core.types.string.partition
kind: method
surface: {}
lifecycle:
  status: null
  stability: null
  since: null
  deprecated-since: null
  removed-in: null
  replacement: null
documentation:
  status: missing
  title: null
  title-style: null
  summary: null
  tags: []
  callables: {}
  sections: []
  see-also: []
provenance: {}
```

The compiler-owned fields are filled from the snapshot. Human-owned unknowns remain `null` or
`missing`; the report may recommend `current` and a title style without writing either. Such a
scaffold cannot enter a public build until reviewed.

### 18.3 Changed entity

For a matched entity, sync computes a typed diff:

```text
api.core.types.string.concat
  signature default
    parameter values
      type: string -> text-display
```

With `--write`, it replaces the compiler-owned surface and compiler provenance. It then validates authored attachments:

- descriptions keyed by retained signature and parameter IDs remain attached;
- descriptions keyed by disappeared IDs are retained but reported as stale;
- new signature components without documentation are reported as missing;
- lifecycle and prose are unchanged;
- a changed canonical symbol is reported prominently even when the stable identity matches.

### 18.4 Missing entity

When a canonical `current` or `deprecated` record has no snapshot match, sync stops with an unresolved lifecycle event. It does not delete the record, empty its surface, or mark it removed.

The reviewer must choose one of:

- record an explicit rename;
- confirm a target/profile distinction;
- correct a compiler regression or snapshot configuration;
- mark the entity `removed` with `removed-in` and optional replacement;
- mark a planned record appropriately if it was never part of the selected compiler surface.

A removed record retains its last known surface and documentation for historical rendering.

### 18.5 Explicit rename

`lifecycle/renames.yaml` uses stable IDs:

```yaml
format: 1
renames:
  - from: api.core.output.write-line
    to: api.core.output.print
    in: "0.3"
    preserve-redirect: true
```

A rename cannot target an existing unrelated identity. Redirects are generated from the mapping. The destination record owns the current surface; historical rendering may retain the source record according to policy.

### 18.6 Report

A sync report groups:

```text
added
changed
renamed
missing-from-compiler
stale-documentation
missing-documentation
lifecycle-conflicts
profile-only-differences
unchanged
```

Every item carries stable IDs and field paths. A machine-readable report uses the same typed paths as the record schema.

## 19. Assembly and rendering

The manual processor:

1. loads the selected publication and its transitive configured imports;
2. parses all YAML while retaining source spans and comments;
3. parses and validates any transitional Markdown pages;
4. validates local record schemas, assets, example files, and source annotations;
5. builds the documentation-set publication, global ID, and redirect registries;
6. resolves owners, namespaces, inheritance, interfaces, traits, type references, and cross-publication links;
7. joins lifecycle, documentation, provenance, and selected compiler/profile facts;
8. validates Markdown references and local IDs;
9. verifies examples when explicitly requested by the build workflow;
10. expands navigation and entity-index queries;
11. creates the assembled manual IR;
12. renders HTML, Markdown, search data, indexes, print views, and optional machine-readable output.

### 19.1 Generated entity page shape

For a function or method, the default renderer produces:

```text
title and summary
availability and lifecycle
canonical symbol and aliases
signature synopsis
parameters with authored descriptions
return contract and description
throwable outcomes and descriptions
authored sections in order
related references
provenance/source links
```

For a class, interface, trait, or descriptor:

```text
title and summary
availability and lifecycle
canonical symbol and aliases
inheritance, interfaces, traits, and ownership
construction synopsis
properties and fields from reverse owner lookup
methods and method families from reverse owner lookup
authored sections in order
related references
provenance/source links
```

For a conceptual page, chapter, or appendix, the renderer uses the authored section tree directly and adds the navigation, lifecycle, provenance, and indexes appropriate to its publication. A book renderer derives visible chapter or appendix numbering from navigation and does not make that numbering part of the page title or stable ID.

### 19.2 Generated outputs

A default build provides:

- browsable manual navigation and breadcrumbs;
- stable canonical URLs and redirects;
- previous/next navigation;
- local section contents;
- source-facing synopses;
- parameter, return, and throwable presentations;
- owner/member and inheritance indexes;
- namespace indexes;
- glossary and rule indexes;
- diagnostic index;
- semantically labelled tables, figures, callouts, exercises, examples, responses, and source annotations;
- search by ID, title, symbol, alias, term, heading, summary, tag, and diagnostic code;
- a printable no-JavaScript reading surface;
- generated Markdown with resolved links and meaningful linear fallbacks for specialized blocks;
- machine-readable assembled IR for editors and other tools;
- a reconciliation and example-evidence report.

HTML, Markdown, and print renderers consume the same profile-selected assembled content. They may differ in navigation chrome, disclosure controls, annotation placement, and other medium-specific presentation, but cannot silently omit a semantic block because the target lacks its richer display. Generated Markdown is a publication artifact, not canonical source; authors edit YAML records, except while an explicitly declared `.md` source remains canonical for its transitional body.

Search ranks exact canonical symbols and aliases above prose matches. Excluded profiles do not leak planned or removed material into results.

## 20. Publication profiles

A publication profile selects:

- Terrane release;
- compiler surface snapshot;
- target and feature profile;
- lifecycle statuses;
- accepted stability levels;
- required documentation completeness;
- whether historical removed pages are emitted;
- example-verification evidence policy.

Typical behaviour:

| Lifecycle | Public current manual | Design preview | Historical manual |
|---|---:|---:|---:|
| `planned` | excluded | included | optional |
| `current` | included | included | version-dependent |
| `deprecated` | included | included | included |
| `removed` | redirect or excluded | optional | included |

Filtering operates on complete records and generated query results. The processor does not conditionally delete arbitrary paragraphs from Markdown. Target- or release-specific prose requires separate structured variation support in a later format version rather than ad hoc template conditions.

A current page must not rely on a filtered planned page for required semantics. Such cross-profile dependencies are validation errors.

## 21. Validation

Diagnostics use stable `R` codes and include record path, YAML path, line, and column where available.

| Range | Category |
|---|---|
| `R0001`–`R0099` | YAML and record syntax |
| `R0100`–`R0199` | IDs, symbols, and redirects |
| `R0200`–`R0299` | record-kind and field shape |
| `R0300`–`R0399` | ownership, types, and entity relationships |
| `R0400`–`R0499` | Markdown links, sections, and navigation |
| `R0500`–`R0599` | lifecycle and publication profiles |
| `R0600`–`R0699` | compiler reconciliation and surface drift |
| `R0700`–`R0799` | examples, example files, annotations, exercises, and conformance evidence |
| `R0800`–`R0899` | assets, accessibility, security, and rendering safety |

A public build fails on:

- invalid YAML or unknown fields;
- duplicate publication IDs, import cycles, duplicate global IDs across an import graph, or duplicate local section, block, exercise, figure, or source-annotation IDs;
- unresolved publication imports, owners, namespaces, type references, links, or redirects;
- relationship targets of the wrong kind;
- object-model cycles forbidden by Terrane;
- current/deprecated lifecycle inconsistent with the selected snapshot;
- incomplete required lifecycle metadata;
- `documentation.status` other than `complete` for included public pages;
- stale callable documentation keys;
- missing required callable descriptions under a strict profile;
- invalid, out-of-bounds, or missing-file source annotations;
- invalid or failed required example evidence;
- unsafe or duplicate multi-file example paths;
- navigation duplication, incompatible publication and record kinds, incompatible numbering, stale `contents`, missing transitional Markdown sources, or unreachable required pages;
- raw HTML, unsafe URLs, missing assets, missing required image alternatives, or record, Markdown-source, example-file, or asset path escapes.

A strict quality profile additionally requires:

- specification or package-contract provenance where the record states a normative public contract;
- conformance evidence for implemented observable contracts where applicable;
- checked accepted and plausible rejected examples for materially documented language boundaries;
- a summary within the configured search length;
- explicit local IDs for linkable sections and semantic display units;
- deprecation migration guidance;
- no hand-maintained member list duplicating assembled IR queries.

## 22. Determinism

An assembled manual build records:

- record-format version;
- compiler snapshot format and compiler version when a snapshot participates;
- processor version;
- selected release, publication, and publication profile;
- sorted input paths and content hashes;
- lifecycle rename map hash when present;
- example target/profile selections;
- assembled IR hash.

Semantic output must not depend on wall-clock timestamps, host paths, random values, directory iteration, YAML map insertion accidents, or process locale. Human deployment timestamps may be added outside the semantic build.

Generated lists sort by explicit navigation order or Unicode code-point order over canonical source spellings, as selected by the query. Sorting policy is recorded rather than inherited from the host locale.

## 23. Security and accessibility

The processor:

- never executes YAML tags, Markdown, or renderer attributes;
- rejects raw HTML and unsafe URL schemes;
- never fetches remote content during assembly;
- sanitizes active assets according to explicit policy;
- escapes all authored content at the renderer boundary;
- runs verified examples only in an isolated temporary package without network access and under resource limits;
- never follows a record, transitional Markdown source, example-file, or asset path outside its configured publication root.

Renderers preserve semantic heading order, visible keyboard focus, table headers, code-language labels, textual callout labels, figure captions, image alternatives, exercise boundaries, and source-annotation locations. Non-decorative images require alt text; decorative images are explicitly marked and use no alternative text. Colour, position, interactivity, and iconography are never the only means of conveying status, sequence, or warning. Generated relationship diagrams have equivalent text lists.

## 24. Authoring guidance

Manual authors should:

1. Put compiler-known facts in `surface`, never in duplicated Markdown tables.
2. Put continuous prose and explanations in Markdown fields; structure a unit when its identity, validation, accessibility, reuse, or semantic display role matters.
3. In canonical YAML records, use the section tree for headings and stable destinations rather than embedding headings in Markdown.
4. Give images meaningful alternatives or mark them decorative; do not repeat a caption mechanically as alt text.
5. Use typed examples for source with a claimed check, build, run, rejection, response, project layout, or annotation contract.
6. Use an admonition kind for rationale, guidance, warnings, and other semantic callouts rather than styling a blockquote to resemble one.
7. Use one exercise block per independently addressable reader activity.
8. Attach parameter and throwable prose through stable local IDs, not source list positions.
9. In public language and entity records, state observable Terrane behaviour rather than Rust implementation details.
10. In internals records, document durable current responsibilities, boundaries, invariants, formats, and implementation evidence without presenting them as public language APIs.
11. Use an `implementation` admonition in a public record when lowering details are genuinely useful but do not warrant their own internals topic.
12. Give each semantic or internal contract one authoritative home and link to it elsewhere.
13. Keep examples focused on one contract.
14. Distinguish accepted, runnable, rejected, and illustrative examples honestly.
15. Describe plausible negative boundaries, not only successful forms.
16. Distinguish `none`, empty data, iteration end, cancellation, throwable failure, and panic according to Terrane semantics.
17. Never describe planned behaviour as current merely because it exists in the design specification.
18. Never mark a missing compiler entity removed without an explicit lifecycle decision.
19. Avoid time-relative prose such as “currently” and “soon”; use lifecycle and release fields.
20. Use generated indexes rather than copied member inventories.
21. Keep entity pages useful when opened from search without duplicating their owner's general contract.
22. Keep tutorial progression selective: link to the reference for exhaustive detail rather than turning every chapter into an API inventory.
23. Keep book `manual.yaml` `contents` synchronized with the complete top-level section order; use `planned` rather than inventing an empty source file for unwritten material.

## 25. Format evolution

`format` versions the YAML manual-record syntax, not the Terrane language. A processor supports only declared versions and fails clearly on newer ones.

A backwards-compatible addition may introduce an optional field only if older processors already reject it rather than silently misrender it. Removing a field or changing its meaning requires a new integer version and a mechanical migration path.

The compiler snapshot format is versioned separately from canonical records. The sync tool owns explicit migrations between supported snapshot versions and the current record format.

Deferred until concrete need justifies them:

- localization and translated-record identity;
- reusable prose includes;
- prose-level target and version conditions;
- shared example fixtures across otherwise independent records;
- structured exercise solutions and generated answer collections;
- user-contributed notes;
- interactive playground execution;
- arbitrary renderer components;
- automatic ingestion of undocumented third-party prose;
- lifecycle decisions inferred from compiler absence.

The durable rule is: **the compiler owns what the surface is, humans own what it means, how it is taught, and how its lifecycle changes, YAML carries the joined manual model, Markdown carries prose inside that model, and renderers own presentation.**