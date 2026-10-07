# Terrane — Working Language Specification and Compiler Architecture

**Draft 0.1 — a human-facing object language lowered transparently to Rust**

> This document is the current integrated design source: normative language semantics, compiler/lowering contracts, and rationale share one file while the design is still changing quickly. Normative requirements are identified by the terms below; implementation sequencing lives separately in `compiler-plan.md`. A future publication may split these views without changing their contract. The constitutional invariants in §41 govern every section and take precedence over illustrative architecture or rationale.
>
> The project, language, and command-line interface have the working name **Terrane**; the CLI command is `terrane`.

---

## 1. Status and terminology

This is the **normative design specification**. Implemented language surface is tracked separately; inclusion here does not imply that every specified feature is implemented.

The words **must**, **must not**, **should**, and **may** are used in their usual specification sense:

- **must / must not** define the proposed language contract;
- **should** describes a strong implementation or ecosystem recommendation;
- **may** describes permitted behaviour or an optional capability.

Three representations are distinguished throughout:

1. **source** — the human-facing language described here;
2. **compiler model** — transient lexer, parser, AST, resolution, and analysis structures;
3. **generated Rust** — the canonical lowered representation passed to Cargo and `rustc`.

The compiler model exists because writing a parser without one would be needless theatre. It is not intended to become a second public intermediate language. For users, tooling, debugging, auditing, and performance work, **generated Rust is the authoritative lowered form**.

---

## 2. Executive summary

The language is designed around a deliberately small set of ideas:

- Everything is an object **semantically**.
- Not everything must be boxed or dynamically represented at runtime.
- Values are typed; bindings are dynamic unless explicitly constrained.
- Ordinary assignment has value semantics.
- Value assignment may use copy-on-write; `ref` observes mutable identity without owning it, `shared ref` shares ownership, and `move` transfers ownership.
- The default global namespace is extremely small and clean.
- Engineers may define or replace their own global and namespace-local bindings, including facilities such as `print`; compile-time constructs such as `import` use separate structural extension slots.
- Imports bind ordinary names, scoped to the block, function, or namespace containing them.
- Namespace segments are separated by `/`, which also anchors resolution at the root; `../` ascends one tier and nests.
- Ordinary syntax favours unshifted characters and readable words over punctuation gymnastics.
- Control flow is conventional where conventional syntax is already good.
- The language lowers to readable, deterministic Rust, then uses the normal Rust toolchain.
- Native Terrane packages, Rust crates, system/C libraries, and full and inline Rust are first-class.
- Compilation is transparent during development and explicit at deployment boundaries.
- Reflection, source mapping, diagnostics, debugging, tracing, allocation analysis, and performance explanation are designed in from the beginning.
- A VM or JIT is not required. Fast incremental Rust compilation is the default development model.
- `no_std`, embedded, firmware, and kernel targets are possible when the program uses only capabilities available on those targets.

A representative program is:

```terrane
namespace my-app

function main;

  project-name = >Terrane
  build-target = >native executable
  build-status = >ready to build

  message = ': '.join; project-name, build-target, build-status
  print; message
```

Conceptually:

1. `namespace my-app` declares this unit's namespace. Nested namespaces separate segments with `/`, as in `my-app/http/handlers`.
2. No import appears because none is needed: `print` is one of the seven default prelude bindings, and every type descriptor is a construct available without import.
3. `': '.join` looks up the `join` member on the `': '` text object.
4. Invoking that member joins its arguments using the receiver as the separator, accepting any number of arguments. This is the shape of Python's `str.join` and PHP's `implode`: the separator supplies the member rather than being passed to it. `join` is distinct from `concat`, which appends its arguments to the receiver without a separator — `'a'.concat; 'b', 'c'` is `abc`.
5. `print; message` invokes `print`’s default behaviour with `message` as its argument.

The output is:

```text
Terrane: native executable: ready to build
```

Imports exist for the cases the prelude does not cover — reaching a namespaced object, or binding one under a different name:

```terrane
namespace my-app

from /core/types import int64 as word

function main;
  size word = 4096
  print; size
```

Here `/` anchors the path at the root and separates its segments, `int64` is the exported name, and `as word` binds it under a different name in this scope. Writing `from /core/output import print` would be redundant, since `print` is already available.

---

## 3. Goals

### 3.1 One human language over mature machinery

The language should justify its existence by **removing the need to care about several lower-level languages for ordinary work**, not by creating another isolated runtime and library island.

The intended stack is:

```text
human source
  -> parse, resolve, analyse
  -> readable generated Rust
  -> Cargo and rustc
  -> native binary, library, firmware image, wasm module, or kernel artefact
```

The language borrows Rust’s implementation ecosystem rather than rebuilding:

- native code generation;
- optimisation;
- ownership machinery;
- platform support;
- linking;
- C ABI integration;
- async and concurrency libraries;
- debuggers and native debug formats;
- package compilation;
- cross-compilation;
- `no_std`.

### 3.2 Progressive strictness

The default experience should get out of the engineer’s way:

```terrane
x = 42
```

When a contract matters, it can be added locally:

```terrane
x int = 42
```

A declared destination performs its own conversion, preserving the value exactly or throwing. Where there is no destination, or where a policy other than that default is wanted, the conversion is written:

```terrane
x = x.coerce; float
```

Strictness should be additive and selectable at binding, member, function, class, namespace, package, and build-profile boundaries.

### 3.3 Clean names by default, real control when desired

The language should not begin by pouring hundreds of functions, variables, classes, helpers, and framework artefacts into global scope.

At the same time, the engineer should be able to define an actual project-global binding without fighting the language:

```terrane
global log = logger
global database = database;
```

If a runtime cannot tolerate a name being replaced, that facility should not masquerade as an ordinary replaceable binding.

### 3.4 Inspectable abstraction

The language should hide machinery when it is irrelevant and expose it unusually well when it matters.

A developer or coding agent should be able to ask:

- what Rust was generated for this function or class?
- what source expression caused this allocation?
- why was this value physically copied?
- was a value assignment satisfied through shared storage or a copy-on-write split?
- which generated Rust span caused this `rustc` diagnostic?
- what source-level object is represented by this native stack frame?
- what capability prevents this code compiling for `no_std`?

### 3.5 Pleasant ordinary typing

The common path should avoid braces, parentheses, colons, underscores, and shifted punctuation where they are not buying clarity.

This is an ergonomic target, not a religious prohibition. Shifted punctuation remains available where it is genuinely the cleanest answer.

---

## 4. Non-goals

The initial language is not intended to be:

- Rust with different punctuation;
- a compatibility implementation of Python, PHP, JavaScript, or another dynamic language;
- a new garbage-collected VM;
- a JIT research project;
- a macro language whose grammar can be rewritten by arbitrary packages;
- an attempt to expose every C++ ABI directly;
- a promise that every dynamic feature works without cost on every target;
- a promise that server processes dynamically recompile source in production;
- an excuse to hide generated code or compiler consequences;
- a second opaque IR layered between source and Rust;
- a language in which weak typing, implicit string/number coercion, and dynamic typing are treated as the same thing.

---

## General declaration annotations (planned)

**Planned for milestone 31.0, not implemented syntax.** General declaration annotations attach typed immutable metadata to declarations for explicit compile-time consumers. They are not executable decorators, HTTP-specific compiler hooks, or the separately deferred `with` declaration-realization protocol. The canonical design is [General declaration annotations](../manual/reference/records/internals/future/annotations.yaml); the [compiler plan](compiler-plan.md) specifies the generic mechanism and independent proving consumers.

### Annotation types and applications

The candidate application spelling is `@[name; arguments]`. Resolve the annotation type through ordinary imports, supply positional arguments before ordinary `name = value` arguments, and retain the explicit semicolon even when the argument list is empty. Brackets delimit the metadata arguments rather than introducing another function-parameter grammar.

Packages define annotation types using a proposed intrinsic marker under `/core/annotations`. The marker declares legal attachment targets and repeatability; it does not execute a decorator or make a runtime registry. The following is illustrative future Terrane. The target enum and definition marker are proposed APIs, not supported imports:

```terrane
from /core/annotations import annotation, annotation-target
from /core/collections import list

@[annotation; targets = (list; (instance annotation-target::callable;)), repeatable = false]
class command
    name string
    summary string

    function construct; name string, summary string
        this.name = name
        this.summary = summary
```

The typed metadata constructor schema determines application arguments, but admission evaluates only the specified immutable data-initialization subset. It never runs arbitrary class constructors, I/O, runtime state, or annotation callbacks. Scalars, immutable aggregates/constants, and explicit typed declaration/type-descriptor slots need precise admissibility and cycle rules before implementation. Annotation payloads are not a loophole for treating type names as unrestricted runtime values.

Annotation targets initially include classes, functions/methods, parameters, and fields. Declaration annotations precede their declaration at the same indentation. Parameter annotations precede the individual parameter inside the existing parameter list, including parenthesized multiline headers. Duplicate non-repeatable applications, inappropriate targets, unknown names, incompatible argument types, and inadmissible values are source diagnostics. Repeatable annotations retain source order without creating hidden behavior precedence.

### Declaration documentation and reflection

Proposed `///` line comments and `/** ... */` block comments form declaration documentation. Consecutive `///` lines form one block; leading `*` decoration on interior block-comment lines is optional and stripped. A contiguous documentation block attaches to the next declaration across its annotation block; empty `///` lines and blank lines inside block documentation preserve paragraphs and meaningful indentation is retained. A plain blank source line outside the comment, ordinary comment, or unrelated declaration ends the attachment opportunity; dangling blocks are diagnosed. Ordinary `#` (including `##`), `//`, and `/* ... */` implementation comments are not exported as documentation.

Star-prefixed block documentation is the canonical example style; undecorated blocks are equally accepted without warnings. Extraction removes delimiters and surrounding blank lines, strips common source-layout indentation, then removes each decorative leading `*` and at most one following space. Preserve paragraph breaks and meaningful indentation after decoration. Normalize extracted documentation only, leaving the original lossless syntax-tree comment unchanged.

Declaration descriptors retain annotation type identity/payloads, documentation, origin, and source spans alongside parameter names/types/defaults, fields/visibility, results, and existing callable contracts. Public metadata survives dependency export even when source and bodies are unavailable. An arbitrary callable value does not necessarily identify one declaration; aliases, re-exports, inheritance, and overrides do not silently copy metadata or manufacture a new origin.

Annotations and prose do not change nominal type/callable identity or compatibility. Artifact/interface fingerprints must nevertheless account for metadata and documentation edits that affect consumers. Compile-time inspection is independent of runtime reflection retention: only explicit runtime consumers require metadata to be embedded in a binary, never on every ordinary value.

### Generic consumers and possible uses

The compiler owns parsing, ordinary resolution, typed immutable construction, targets/repeatability, source diagnostics, canonical declaration reflection, and dependency retention. Packages/tools explicitly select declarations, interpret their metadata, validate domain promises, and generate ordinary code through one generic compile-time consumer mechanism. Invocation/configuration/output of that mechanism remains to be specified; these examples are consumer inputs, not claims of released package APIs.

The signature remains the source of structural type facts. Metadata supplies what signatures cannot infer, such as external names, binding sources, prose, constraints, examples, or tags. Descriptive metadata does not itself implement authentication, validation, storage, or registration. Consumers that promise executable behavior must enforce it and preserve visibility, ownership, receiver authority, effects, throwable bounds, and capability policy. Importing an annotation never globally registers the annotated declaration.

| Possible consumer | Metadata beyond declared types |
| --- | --- |
| Serialization and validation | External field names, omitted fields, bounds, lengths, documentation and examples |
| Command-line interfaces | Command/flag names, environment bindings, help text |
| Tests and benchmarks | Explicit discovery, tags, parameter sets |
| Database/persistence mapping | Table/column names, keys, mapping policy |
| RPC/message dispatch | Operation names, topics, wire versions |
| Documentation and tooling | Examples, grouping, deprecation explanations |
| HTTP endpoints | Method/path, parameter source, response documentation; not a special compiler facility |

#### Command-line metadata

The following illustrative packages define `command`, `option`, and `minimum`; their specific APIs are not standardized here. The CLI consumer derives an integer option with default 10 from the signature, uses annotations for spelling/help, and enforces the declared minimum through its validation consumer:

```terrane
from /example-cli import command, option
from /example-validation import minimum

/// Summarize the requested number of rows.
@[command; name = 'report', summary = 'Summarize rows']
function report int; (
    @[option; long-name = 'limit', short-name = 'n', help = 'Maximum rows']
    @[minimum; 1] limit int = 10
)
    return limit
```

The declared result remains `int`; a CLI adapter must define its own output/exit-status policy rather than silently reinterpret the return type. Explicit consumer configuration selects this declaration; no process-argument parser runs merely because the function is annotated.

#### Serialization and validation metadata

An independent codec consumer takes field types from the model, uses the same external name for encoding/decoding/schema output, and enforces admitted constraints. Descriptions enrich the derived representation without restating the field type:

```terrane
from /example-codec import wire-name, description
from /example-validation import minimum, length

class account
    /// Stable public account identifier.
    @[wire-name; 'account_id']
    @[minimum; 1]
    id int

    @[description; 'Name displayed to clients.']
    @[length; minimum = 1, maximum = 120]
    name string

    note string|none = none

    function construct; id int, name string, note string|none
        this.id = id
        this.name = name
        this.note = note
```

The consumer must specify optional/default/null and field-selection/secrecy rules, supported wire types, constraint units, and invariant-preserving construction. It cannot bypass private fields, assign arbitrary model state, advertise unsupported encodings, or silently reinterpret a field's type. Metadata constraints are enforced by that admitted consumer, not by every ordinary Terrane assignment.

#### Test and RPC metadata

These illustrative declarations show two other independent domains without requiring new compiler keywords. The test example belongs in a test source unit where `/core/testing` is admitted; an annotation does not grant a production source access to test-only namespaces.

```terrane
from /example-tests import test, tag
from /core/testing import assert-equal-int

@[test;]
@[tag; 'arithmetic']
function addition-example;
    assert-equal-int; (2 + 3), 5
```

```terrane
from /example-rpc import operation

@[operation; name = 'sum', version = 1]
function sum int; left int, right int
    return left + right
```

Test discovery must be explicitly activated; RPC dispatch/encoding must implement the declared signature and wire contract. Neither annotation changes direct invocation of the function.

### Implementation boundary

Milestone 31.0 must prove CLI and serialization/validation consumers through the same canonical metadata path, including real generated behavior, rejected contradictory declarations, dependency metadata without source, and metadata-only cache invalidation. Do not add compiler branches keyed to an HTTP framework or one annotation type.

Final delimiters, intrinsic target representation, immutable construction rules, generic consumer invocation/inspection APIs, and runtime metadata materialization need implementation evidence. The examples here remain illustrative until that end-to-end support exists. The separately deferred realization/construct-replacement designs remain deferred; this planned metadata capability does not activate them.

---

## 42. Deferred language additions

This section records directions that the current design should leave room for but does not make part of the version-one language contract. Entries here are neither reserved syntax nor permission for implementations to introduce incompatible private variants. Each requires a later specification change, grammar and tooling work, lowering rules, diagnostics, reflection behaviour, and conformance tests.

### 42.1 Core constructs supplied as objects

The object model may eventually extend beyond replaceable facilities such as `print`: named language constructs could be selected from `/core` through one uniform compile-time construct protocol. The family must be designed together rather than adding an isolated hook for `function`. Candidates include declarations and control-flow constructs such as `function`, `class`, `if`, `for`, `while`, `try`, `throw`, `async`, `await`, and `return`.

The intended architectural split is:

```text
fixed lexical and layout substrate
  -> structurally parsed construct
  -> scoped construct implementation selected from /core or a package
  -> validated typed semantic IR
  -> ordinary lowering
```

Tokenisation, comments, indentation, literals, grouping, separators, namespace anchors, ownership and safety invariants, and the mechanism that selects construct implementations remain constitutional compiler structure. A construct implementation may validate or constrain a parsed construct, select compiler-supported ABI or lowering behaviour, attach reflected metadata, and produce source-mapped declarations through declared extension points. It must not reinterpret arbitrary source text, mutate the grammar opportunistically, hide effects, bypass safety or capability checks, or emit unsourced code.

Construct selection must use a dedicated scope and explicit syntax; it must not depend on an ordinary binding that happens to be named `function` or `if`. The eventual design must specify lexical, namespace, package, and program-global replacement; interactions among related constructs such as `if`/`else` and `try`/`catch`; compatibility with editor parsing before dependency resolution; hygiene; reproducibility; compiler-protocol versioning; and how source declares the language profile it expects.

Declaration modifiers are the version-one local customization mechanism. A future construct binding would select the default semantics for a whole scope, while a modifier would customize one declaration. Until the common construct protocol is specified, version one keeps named core constructs structurally built in, and implementations must not expose an ad hoc replaceable `function` or any equivalent one-off hook.

### 42.2 Other deferred candidates



- source-declared generics, including constraints, inference, dispatch, reflection, and monomorphisation or erasure rules;


- compact map literals consistent with the punctuation and computed-key model;
- stateful hot-code replacement with explicit object migration semantics;
- arbitrary C++ ABI integration beyond C-compatible shims and Rust bridges;
- multimethod or generic-function dispatch supplied as a library or language feature without making overload resolution implicit;
- foreign-runtime adapters with explicit semantic, ownership, lifetime, thread, error, capability, tooling, and deployment contracts;

This list is intentionally non-exhaustive. Adding an item here protects a design direction from accidental closure; it does not give that feature priority over the version-one compiler plan.

---

## 43. Closing proposition

The language is not justified merely by prettier syntax.

Its claim is the combination:

```text
human-friendly object language
  + clean and controllable namespaces
  + dynamic bindings with typed values
  + strictness on demand
  + value semantics with explicit identity
  + transparent generated Rust
  + native/Rust/C package interoperability
  + first-class diagnostics and observability
  + direct Rust escape hatches
  + compiled deployment from ordinary dynamic-language ergonomics
```

That is a credible reason for one more language: not another isolated world, but a human-facing layer that consolidates several existing ones and deliberately refuses to trap its users above the implementation.
