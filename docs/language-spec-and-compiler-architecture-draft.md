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

The following already-motivated features may be specified later when implementation experience justifies them:

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
