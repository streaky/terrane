# Rust dependencies through Terrane

Working notes for milestone 25. Records the design as it currently stands, the evidence in the
codebase that supports it, and the questions that still need answers before the milestone can be
written normatively.

## 1. What we are solving

A Terrane project declares a Rust package. The compiler resolves it, fetches it, links it into the
generated crate, and makes it usable from Terrane source. The language server shows the package's
interface *as Terrane* so the author gets completion, signature help, and hover without the compiler
owning a model of the crate.

The project already does this by hand for a fixed set of support crates. The milestone is to do the
same thing mechanically, for an arbitrary crate, deterministically, without an author-written
support layer.

### 1.1 Generic by construction

Nothing in this design may be crate-specific. `reqwest` is the first crate we want working and the
witness the milestone uses, not the target: it is a plausible worst case (async, generic bounds,
builders, an opaque response type, a feature matrix) which is why it is worth proving against. But
every rule below is stated over rustdoc JSON and Cargo metadata in general, and the projector must
contain no knowledge of any particular package.

The practical guard is the exit criterion. Proving the milestone with `reqwest` alone will produce
`reqwest` support by accident, because a single witness cannot distinguish a general rule from a
tuned one. The fixtures should include **at least one further crate, deliberately dissimilar** — a
synchronous, non-network, data-shaped crate with a different API idiom — projected and called through
the same machinery with no special casing. If the second crate needs a code change, the design was
not generic.

## 2. What already exists

`crates/terrane-document-support` wraps `serde_json`, `yaml-rust2`, and `url`. Four layers connect it
to Terrane source:

1. **Support crate.** `crates/terrane-document-support/src/lib.rs` exposes flat free functions over
   `serde_json`/`yaml-rust2`/`url`, hiding those crates' types behind `DataResult` and `UrlResult`.
2. **Platform namespace.** `/core/platform-data` names the primitives the compiler recognises
   (`crates/terrane-compiler/src/semantics.rs`, the name list and the call-to-type mapping).
3. **Generated shims.** `crates/terrane-compiler/src/runtime/platform_urls.rs` and its siblings emit
   one thin Rust function per crossed member into the generated crate.
4. **Authored Terrane wrapper.** `crates/terrane-compiler/src/standard/urls.trn` imports the platform
   primitives and presents an ordinary Terrane API.

Milestone 25 generalises layers 2, 3 and 4: `/deps/<crate>` replaces the hard-coded platform
namespace, the shims are generated from a projection instead of being written by hand, and layer 4
becomes optional rather than mandatory.

### 2.1 The shape the existing boundary already has

The hand-written boundary converged on a shape, and it is worth reading as evidence rather than
theory. Across `platform_urls.rs` and `platform_documents.rs`:

- free functions only, no methods, no generics, no traits, no lifetimes in signatures;
- parameters are Terrane-lowered types: `String`, `bool`, `terrane_int_support::Int`, or a shared
  reference to an opaque foreign value;
- returns are owned: `String`, `bool`, `Int`, or a new opaque value;
- opaque values are shared, not moved. `DataResult` holds `Option<Arc<Document>>` behind a private
  field, receivers are `&DataResult`, and every mutator returns a new value;
- fallibility is carried in the value (`failed`, `message`, `path`, `expected`), never as `Result`
  across the boundary;
- integer width crossings are explicit at the edge.

Because receivers are shared and results are owned, **no move ever crosses the boundary**, which is
why the project has needed no ownership tracking for foreign values so far. That is the default rule
the generated boundary should adopt, and section 6.2 records where it does not stretch.

## 3. The reframe: there is no FFI here

Terrane lowers to Rust. Generated code calling `reqwest` is Rust calling Rust inside one crate. A
`reqwest::Response` held by a Terrane binding is a Rust local with a Rust type — no box, no proxy, no
marshalling, no conversion. Terrane's type system only has to carry an opaque node it never inspects,
which `ValueType::PlatformUrlResult` already demonstrates: the entire representation is a string
naming a Rust path (`crates/terrane-compiler/src/lowering.rs`).

The boundary is therefore not a data bridge. It is exactly the set of places where a guarantee
Terrane makes does not automatically hold on the crate's side.

## 4. Two problems, one shared spec

**Compiler side.** Find the minimum practical and safe boundary between lowered Rust and the package,
and generate only the machinery the program actually crosses.

**Tooling side.** Present that boundary as a Terrane API to editors.

They are independent implementations, but they must derive from **one artifact**: a projection model
computed from the lock-resolved package. If hints and codegen derive the surface separately they
drift, and the editor eventually offers something the compiler will not lower. The boundary rules
below are the shared spec; the language server renders the model, the compiler consumes it.

## 5. Declaration and namespace

**The project manifest is the only place a dependency is declared.** The `.toml` carries the crate,
version, features, default-feature policy, and target conditions. There is no source-level
dependency declaration: no `use rust crate-name`, no per-file ceremony. Once a dependency is in the
manifest it is available, and importing from it is the only thing source does.

This is the distinction §23.3 already draws — dependency graph composition is not the same operation
as name binding — carried to its conclusion. `from /deps/reqwest import get` performs the
binding; the manifest performed the composition. A `from /deps/...` import naming a crate the
package's manifest does not declare is a Terrane diagnostic.

The root was shortened from `/dependencies` to `/deps` before milestone 25 merged, solely to keep
dependency imports concise; the longer spelling was never a compatibility contract.

The projected surface appears at a predictable path:

```terrane
namespace /deps/reqwest
```

`/deps` is a reserved root segment so a package cannot collide with it. Members are projected
under the crate's own module structure, with Rust naming mapped to Terrane naming.

Use is ordinary:

```terrane
namespace my-app
from /deps/reqwest import get as reqwest-get
foo = await reqwest-get; >https://httpbin.org/ip
```

## 6. The boundary rules

### 6.1 Free

- **Opaque values.** A `Response` lowers to its own Rust type. Terrane carries an opaque node, never
  inspects it, and never converts it.
- **Scalars, strings, collections.** Already lower to Rust types; pass directly or with the same edge
  coercion `platform_urls.rs` performs for `Int` today.

### 6.2 Codegen patterns

- **`Result<T, E>` projects as a function returning `T` with a `throws` contract.** Terrane's
  recoverable throws already lower through compiler-owned `Result`-like control flow rather than Rust
  panic unwinding (§15.4), so this is not a translation between two error models — both sides are
  already `Result` in the generated crate. The shim matches, and `Err(e)` becomes a Terrane throw
  carrying a foreign-error throwable. `E` satisfies `throwable` cheaply: `message` and `render` from
  `Display`, `cause` from `Error::source()`.

  Because `Result<T, E>` names its error type exactly, the projection can state a real `throws`
  upper-bound contract rather than leaving the surface silent about failure. `throws` is a closed
  function qualifier written after the return type (§15.4), and it is an upper bound, which is the
  right strength here: it promises what the crate's signature promises without claiming to know what
  the implementation currently produces.

  Projecting `Result` structurally instead would require source-declared generics and general pattern
  matching, both post-v1 (`docs/surface-v1.md`), and would give user code a second error style
  alongside `throw`.
- **`Option<T>` projects as `T|none`.** Terrane already has that union.
- **Receivers project faithfully, and the existing ownership model carries them.** A projected foreign
  value is an identity-bearing resource, not an ordinary COW value, under the foreign-resource
  ownership rule now recorded in specification §23.8. So:

  | Rust receiver | Terrane contract and call |
  |---|---|
  | `&self` | ordinary member call; receiver borrowed without transfer |
  | `&mut self` | ordinary member call; projected contract marks the receiver mutable, so lowering emits a mutable binding and borrow |
  | `self` | `move` under the ordinary foreign-resource ownership rule |

  Self-consuming methods therefore need no special handling. A builder chain binds no intermediate, so
  it reads without ceremony; `move` becomes visible only where an author binds an intermediate and
  then consumes it, which is precisely where Rust would also require them to think about it.
  Use-after-move is already a compile-time error (§13), and §29.3 already specifies the diagnostic —
  its worked example is a move error on a crate call.

  This supersedes the shape observed in §2.1 as a *constraint*: shared receivers and owned returns are
  how the hand-written support crates were designed, not a rule the projector must impose. Those
  crates never needed the ownership model because they were built to avoid it; an arbitrary crate is
  not, and the language already has the answer.
- **Returns are owned where the crate returns owned values**, with a clone at the edge where it
  returns a borrow of a `Copy`/`ToOwned` type. A borrow that cannot be cloned falls under 6.5.

### 6.3 Panic containment

Well-behaved crates return `Result` for operational failure and reserve panic for invariant
violation, so the common case never reaches this rule: a request against a down server is an `Err`,
projected under 6.2, and arrives in Terrane as an ordinary throw. Panic is the exceptional path, and
the specification already anticipates it — §15.4 reserves Rust panic for unrecoverable invariant
failure, explicit panic, or "an *untranslated* native dependency panic", wording that implies
translation wherever it is available.

The rule follows the build profile, which §15.6 already makes the deciding authority:

- **Unwinding profile.** The generated shim contains the unwind at the crossing and converts it to a
  throw of a `dependency-panic` throwable carrying the panic payload, the crate identity, and the
  crossed member. It is catchable like any other throw.
- **Aborting profile.** No containment. A dependency panic terminates, exactly as the profile already
  promises for Terrane's own panics. Kernel and embedded profiles commonly sit here.

Containment is a modest improvement on plain Rust rather than a compromise with it: in a Rust binary
an uncaught library panic unwinds to the top and ends the process, whereas a contained crossing lets
a Terrane program handle it. Where Rust genuinely must abort, it aborts.

Three residuals to state in the milestone rather than discover later: a panic on a thread the crate
spawned cannot be caught at the call boundary; `catch_unwind` imposes `UnwindSafe`, which not every
crate value satisfies; and containment cannot be elided per member, since panic freedom is not
statically knowable.

Milestone 25.2 represents profile panic policy explicitly. Abort profiles emit no catch boundary and
configure generated Cargo with `panic = "abort"`. Unwinding profiles retain `catch_unwind`; receiver-
free crossings use Rust's `UnwindSafe` proof, while receiver crossings use a deliberate
`AssertUnwindSafe` boundary because the foreign receiver is the captured logical invariant. This is
not applied blanket-wise to every dependency call. Fixture-owned generated Rust verifies panic
payload and crate/member preservation, and ordinary Terrane `try`/`catch` coverage verifies the
`dependency-panic` throwable class.

### 6.4 Diagnostic translation, not new language features

Rust already enforces these in the generated crate; the work is reporting them in Terrane terms
before rustc reports them against generated source. `docs/language-spec-and-compiler-architecture-draft.md`
§29.3 already commits to this and its worked example is a move error on a crate call.

- moves and drops;
- `Send`/`Sync` at task boundaries;
- lifetime errors from any borrow that escapes.

### 6.5 Not projected

Items the projector cannot render must be **visibly absent with a reason attached**, so hover can say
"not projected: unbounded generic — use a native Rust body" rather than leaving the author guessing.
The native Rust body remains the escape hatch for everything below.

- unbounded or open generic parameters;
- trait objects and trait-generic APIs;
- lifetime-parametric types;
- anything returning a borrow that cannot be cloned at the edge;
- macro-only APIs.

## 7. The projection

Input: the lock-resolved package, its enabled features, the target, and rustdoc JSON for those exact
versions. No package code is executed. Output: a namespace model.

Mechanical rules:

- module paths become namespace paths under `/deps/<crate>`;
- `async fn` projects as an async Terrane function; the existing async model applies, with tokio
  already in the generated crate;
- **bound-driven monomorphisation**: for a generic parameter with a closed, inspectable bound, the
  projector enumerates impls and keeps those with a Terrane-representable type. `T: IntoUrl` has
  impls for `&str`/`String`/`Url`, so `get<T: IntoUrl>(url: T)` projects as `url string`. Where the
  impl set is not closed, the item falls under 6.5.
- monomorphised names live inside the dependency namespace, so they collide with nothing;
- doc comments become hover text.

Worked example. `reqwest` documents:

```rust
pub async fn get<T: IntoUrl>(url: T) -> Result<Response>
```

which projects to (syntax illustrative):

```terrane
namespace /deps/reqwest
async function get response throws reqwest-error; url string
```

and generates, for that one crossed member, a shim in the shape `platform_urls.rs` already uses.

### 7.1 Caller-chosen type arguments

`Response::json::<T>()` takes its type argument from the caller under a `Deserialize` bound. The
projection supports **Terrane-native destination types only**: a Terrane `map string, string` becomes
`HashMap<String, String>`, a list becomes `Vec<_>`, scalars map directly, and the shim names the
concrete Rust type at the call site. A Terrane object type as the destination would mean generating a
Rust struct with a `Deserialize` derive, which makes `serde` a structural dependency of the projector
itself; that is deferred. Where the destination is not natively representable the member falls under
6.5 and a native Rust body is the escape hatch.

The same rule generalises past `json`: any caller-supplied type argument is admitted when the Terrane
type has a direct Rust representation and the bound is satisfied by it, and refused otherwise.

### 7.2 Naming

A third-party crate's naming is not Terrane's business. Projected names are **verbatim**:
`reqwest.ClientBuilder` is `ClientBuilder`, `parse_json` is `parse_json`. The surface matches the
crate's own documentation, which is most of the value of having a projection.

This requires uppercase and underscore to be lexically legal at a use site, since the author writes
those names in their own file. Terrane can lower them — the generated-Rust encoding note already
anticipates underscore "if it becomes legal in source identifiers" — so the restriction fails the
test of only restricting what we cannot lower. The working answer is therefore:

- uppercase and underscore become legal identifier characters;
- kebab-case remains the convention and stays mandatory for compiler-owned and standard-library
  names and for every documentation example; user code may use any case, with the kebab check
  available as an opt-in lint rather than enforced;
- **namespace segments are unchanged** and stay lowercase with hyphens. That rule exists because
  segments map to directory names and case-only-differing segments collide through the Win32 layer —
  portability, not convention. Rust module names are snake_case, so a segment maps `_` to `-`; type
  names are members within a namespace rather than segments, so they are unaffected;
- type parameters keep their existing uppercase carve-out.

Verbatim projection also removes the collision problem: Rust item names are already unique within
their module, so no mapping can fold two of them together.

### 7.3 Trait methods

Rustdoc reports inherent and trait impls separately, and merging them into one member list would need
a collision rule. Namespacing removes the need for one, because Rust already disambiguates the same
way — UFCS, plus the requirement that a trait be in scope to call its methods.

- **Inherent methods** project as members of the type. Rust forbids duplicate inherent method names on
  one type, so no collision is possible.
- **Trait methods** project into the **trait's own namespace**, as free functions taking the receiver
  as the first parameter. The trait's canonical Rust path gives the namespace deterministically, and
  it may belong to another crate. Two traits are two namespaces, so a collision is not representable.
- **Choosing between them is an import**, not a heuristic: `from /deps/tokio/io/AsyncReadExt import
  read-to-end` is Terrane's spelling of Rust's "the trait must be in scope", and the choice is
  recorded in the author's own file.

This is also the shape the hand-written boundary already uses: `platform_urls.rs` exposes
`url_query_key(result, index)` rather than a method on a receiver.

**Blanket impls** need no inclusion rule under this scheme. The trait namespace holds the method once;
whether a particular receiver satisfies the bound is decided by rustc when the shim is generated, so
the projector never computes impl applicability. Rustdoc's implementor list is still worth consulting
to filter what hover offers — advisory in the §23.1 sense, with correctness left to the build.

The cost, stated deliberately: a trait method reads as `read-to-end; response` rather than
`response.read-to-end`. That is less sugary than Rust and identical to the existing boundary. A later
ergonomic rule could permit method syntax where exactly one imported trait supplies the name and no
inherent member competes; it is not needed for correctness.

### 7.4 Enums

- **Data-free enums** (`Method`, `Version`) project as an opaque value with projected zero-parameter
  constructors and comparison.
- **Data-carrying enums** project as opaque values with whatever accessors the crate provides. Without
  general pattern matching there is no safe destructuring form to offer, so none is offered.
- `Result` and `Option` are not enums for this purpose; see 6.2.

`Result<T, E>` lowers to directly representable `T` plus the throwable path, and `Option<T>` lowers
recursively to `T|none`, including arbitrary projected foreign objects. Identity result conversions
emit the original `Option<T>` directly rather than an identity `map`. The projector preserves all
Rust integer widths with checked edge coercion, projects `f32` and `char` without narrowing, and
resolves concrete type aliases transparently. Generic, associated, cyclic, or otherwise unresolved
aliases remain explicit declines.

Data-free enum variants project as zero-parameter constructor shims returning the opaque enum value,
and generated Rust labels those shims as enum-variant constructors so they are not mistaken for
ordinary Rust functions during debugging. Ordinary object equality supplies comparison.
Data-carrying enums remain opaque and expose only representable crate-provided accessors; no
destructuring form is inferred.

## 8. Capabilities and transitive dependencies

Transitive Rust dependencies are Rust's business. The build resolves them into the final crate graph
and nothing is projected for them; a crate using Rust directly needs nothing from the Terrane layer.
Terrane presents the dependencies the application asked for, and stops there. We are not trying to
stop Rust doing Rust things.

Capability enforcement follows from that stance: **the manifest declaration is the grant.** Declaring
`reqwest` grants the effects it carries, transitively, and the build report identifies what was
pulled in and what executed code during compilation. `reqwest` opens sockets whatever the profile
says, so there is no useful call-site enforcement to add.

The consequence is worth accepting deliberately rather than tripping over: a profile that forbids
networking must **reject the dependency at manifest resolution**, not at a call site. For Rust
dependencies, capability enforcement is a property of the declaration; for Terrane-native code it
remains a property of the code. That difference is real and should be written down where the
capability model is specified, not left implicit here.

### 8.1 Sandbox policy

Containment is the control that actually reduces the exposure a declared dependency creates, and it
pays twice: the
restrictions that stop a hostile crate exfiltrating are the same ones that stop a build script reading
something undeclared and making the cache identity a lie. Safety and determinism want the same
mechanism, so the milestone should take both from it.

Default tier: **fetch online, build contained.** Resolution needs the network; compilation does not.
The build phase runs `--offline --frozen` with the filesystem scoped to the project, the cargo home,
and the target directory, and with no process execution outside the toolchain.

Three constraints to state rather than discover:

- **Granularity is the cargo invocation, not the crate.** Build scripts are separate processes and
  could be contained individually, but proc macros expand inside rustc, so containing them means
  containing the compiler. The realistic unit is the whole build. (A separate proc-macro server, as
  rust-analyzer uses, is independently containable; cargo's in-process expansion is not.)
- **An allowlist tier is required.** `pkg-config` discovery and system-library linking reach outside
  the project tree by design — the §23.9 case. That tier is where the explicit build capability stops
  being ceremony and starts naming what a build is permitted to reach. The `reqwest` slice avoids it:
  `rustls-tls` rather than `native-tls` means it should build fully contained.
- **Sandbox strength varies by platform.** The policy must declare when it cannot contain a build
  rather than proceeding as though it had.

Sandbox tier and policy inputs are part of cache identity, so a build that reached further is not
cache-equivalent to one that did not.

## 9. Determinism and cache identity

Cache identity covers manifest contents, lock checksum, enabled features, default-feature policy,
target triple, toolchain version, and package source checksums — the same set §23.1 already requires
of tooling. The projection is a pure function of that identity, so the language server and the
compiler compute the same model or neither does.
The project-local cache retains the current projection and at most three prior projection artifacts.
This bounded history avoids repeated rustdoc work during ordinary lockfile rollback and editor churn
without allowing one project directory to grow indefinitely. Durable, machine-independent
`terrane-projection.lock` history records projected namespace/member pairs by resolved dependency
version.

The projection pass and generated-crate compilation use the build capability policy: fetch may run
online, then rustdoc and compilation run offline and frozen inside `bwrap` where available. A platform
without enforcement reports the unavailable host tier rather than refusing the dependency.

Generated Cargo and generated Rust are golden-tested. A lock bump that removes a crossed projected
member is a source-visible interface change; `S2031` diagnoses it at the Terrane import and names the
member, previous version, and current version. A member absent from both history and the current
projection retains the ordinary absent-or-declined diagnostic.

## 10. Conflicts with the current specification

`docs/language-spec-and-compiler-architecture-draft.md` §23.1 already states the governing principle
in the form this design needs: declarations name packages not APIs, the build bridges only what is
crossed, and tooling projects an advisory *Terrane-visible* surface.

§23.8 then hardens that into something stricter and incompatible: that Rust constructs are "touched
only inside native Rust bodies", and that a Terrane-visible wrapper is "authored deliberately, never
produced automatically". Under this design the projection is produced automatically, from the
lock-resolved package, and an authored wrapper becomes optional ergonomics rather than the price of
entry. §23.8 must be reconciled with §23.1.

§23.2 and §23.8 also show `use rust serde` as a source-level dependency declaration. Section 5 above
removes it: dependencies are declared in the project manifest only. The `use` form is redundant once
`from /deps/...` performs the binding, and its removal makes §23.3's own distinction cleaner
rather than weaker. Whether the same applies to the other two version-one dependency origins in
§23.2—native Terrane packages and system libraries—is a larger question this note does not settle,
but consistency argues for one answer across all three. Foreign-runtime adapters are explicitly
post-version-one and are not a fourth current origin.

`docs/surface-v1.md` §14.1 asserts the §23.8 position and needs the same treatment. The milestone 25
text in `docs/compiler-plan.md` carries the wrapper sentence and inherits the foreign-adapter phrase
"boundary machinery Terrane source actually crosses", which for Rust means generated shims in one
crate rather than a marshalling layer.

The drift argument §23.1 makes against a predefined surface still holds and is not weakened here: the
projection is derived per lock rather than checked in, so it cannot go stale, and rustc remains the
authority on whether the generated crate compiles.

## 11. Actions

Concrete work implied by the resolved design. Each action names the section that decided it. Nothing
here is conditional or outstanding.

### 11.1 Specification amendments

- **A1 — §5 identifiers.** Make uppercase and underscore legal identifier characters, so that
  verbatim third-party names are writable at a use site. Remove the specified rejection of uppercase
  in user-declared names, along with its diagnostic and formatter fixit. Namespace segments and the
  type-parameter carve-out are unchanged.

  Case becomes a matter of naming, not of legality. §5 keeps stating kebab-case as Terrane's naming
  convention, with its existing reasoning, and the amendment states its own scope:

  - **kebab-case remains mandatory for compiler-owned and standard-library names** — `/core/...`,
    the language-mandated throwable classes, and every name the language itself declares. Enforced,
    not advised; a non-kebab name in core is a defect, and the check belongs in the project's own
    build rather than in the lexer;
  - **all documentation examples use kebab-case**, including examples in this note and in the
    specification;
  - **user code may use any case, and the language does not enforce its own house style on it.** The
    kebab-case check survives as an available compiler advisory with the §5 wording and declaration-specific help, but
    it is advisory and off by default: a project that wants Terrane's convention can turn it on, and
    one that does not is not nagged. What changes is that the rule stops being a rejection and stops
    being applied to code Terrane does not own;
  - **projected dependency names are verbatim**, matching the crate's own documentation, and are
    exempt from the lint even where it is enabled — otherwise every dependency import is flagged for
    obeying its own ecosystem's conventions.

  The consequence to accept, since §5 currently avoids it deliberately: with both spellings legal and
  the check off by default, one corpus can contain `parse-json` and `parseJson`, and the
  acronym-casing question returns for user code. The trade is that the language stops restricting what
  it can lower perfectly well, and constrains only the names it owns. (§7.2)
- **A2 — §23.8.** Reconcile with §23.1: remove "touched only inside native Rust bodies" and the
  "authored deliberately, never produced automatically" wrapper sentence. An authored wrapper becomes
  optional ergonomics, not the price of entry. (§10)
- **A3 — §23.2, §23.3.** Remove the source-level `use rust crate-name` form and its examples.
  Dependencies are declared in the project manifest only; `from /deps/...` performs the
  binding. Decide separately whether the same applies to the other two version-one origins. (§5)
- **A4 — `docs/surface-v1.md` §14.1.** Restate to match A2; it currently asserts the §23.8 position.
  (§10)
- **A5 — capability model.** Record that for Rust dependencies the manifest declaration is the grant,
  effects are transitive, and a profile forbidding an effect rejects the dependency at manifest
  resolution rather than at a call site. (§8)
- **A6 — standard throwables.** Add `dependency-panic`, carrying the panic payload, crate identity,
  and crossed member. (§6.3)
- **A6a — §23.1 execution rule.** Reserve "must not execute arbitrary package code merely to inspect
  it" for a future post-version-one foreign-runtime adapter, where importing `numpy` to enumerate it
  would be genuine and avoidable execution. For Rust it is not avoidable—inspection is compilation,
  since rustdoc runs the
  front end and expands proc macros. State the Rust rule instead: the projection pass executes package
  code under the same explicit build capability and sandbox policy as a build script, and without that
  grant neither the editor nor the build runs it.

  Record the reasoning, because the rule looks like a weakening and is not. Deferring execution to
  build time does not reduce the exposure — a declared dependency runs with the author's privileges
  the first time they build, so the choice of trigger relocates the risk by minutes and buys a cold
  editor. The exposure is real; the trigger is not the control. Containment is (A15a), and it applies
  to both paths identically. (§8.1)

### 11.2 Compiler

- **A7 — reserve `/deps`** as a root namespace segment. (§5)
- **A8 — manifest schema** for Rust dependencies: crate, version, features, default-feature policy,
  target conditions. Diagnose a `from /deps/...` import naming an undeclared crate. (§5)
- **A9 — generalise the opaque value type.** Replace `ValueType::PlatformDataResult` and
  `ValueType::PlatformUrlResult` with a general foreign value type keyed by crate and Rust path.
  (§3)
- **A10 — the projector.** One artifact, computed from lock-resolved package, features, target, and
  rustdoc JSON, consumed by both the compiler and the language server: module paths to namespaces,
  verbatim names, `async fn` to async, bound-driven monomorphisation, inherent methods as members,
  trait methods into the trait's namespace, enums per §7.4, and a recorded reason for every item it
  declines to project. (§4, §6.5, §7)
- **A11 — shim generation** for crossed members only: receivers projected faithfully (`&self` as a
  shared receiver, `&mut self` as receiver mutability on the projected contract, and `self` as
  `move`), owned returns with an edge clone where the crate returns a cloneable borrow, and edge
  coercion for scalars. Mutable receivers use ordinary Terrane member-call syntax; the contract drives
  mutable binding and borrowing in generated Rust. Until object identity includes its namespace,
  conflicting receiver contracts on same-named projected types are a compile-time ambiguity rather
  than being selected by projection order. (§6.2)
- **A11a — foreign values are identity-bearing resources**, so ordinary value assignment does not
  apply to them and use-after-move is diagnosed by the existing foreign-resource ownership rule in
  specification §23.8. (§6.2)
- **A12 — `Result` and `Option` lowering.** `Result<T, E>` becomes a return of `T` with a `throws`
  contract naming the projected error; `Option<T>` becomes `T|none`. (§6.2)
- **A13 — panic containment** at the crossing on unwinding profiles, converting to `dependency-panic`;
  none on aborting profiles. (§6.3)
- **A14 — diagnostic translation** for moves, drops, `Send`/`Sync` at task boundaries, and escaping
  borrows, reported against Terrane source per §29.3. (§6.4)
- **A15a — sandbox policy.** Default tier: fetch online, then build `--offline --frozen` with the
  filesystem scoped to project, cargo home, and target, and no process execution outside the
  toolchain. An allowlist tier for crates needing system discovery. Contain at the cargo-invocation
  level, since proc macros expand inside rustc. Declare, rather than assume, containment on platforms
  where it cannot be enforced. Sandbox tier and policy inputs join cache identity. (§8.1)
- **A15 — cache identity and build parity.** Gate the projection pass on the build capability per
  A6a, under the A15a policy. Manifest, lock checksum, features, default-feature
  policy, target triple, toolchain version, package source checksums. The projection pass runs under
  the same build capability, sandbox policy, and cache-identity inputs as a build script. (§9)
- **A16 — lock-change diagnostics.** A missing crossed member is diagnosed at its Terrane import
  rather than as a rustc error against generated source. Machine-independent projection history
  distinguishes a removed member from one never present and names the resolved version change. (§9)

### 11.3 Tooling

- **A17 — language server** renders the A10 model: completion, signature help, hover. Hover shows the
  verbatim Rust path and, for declined items, the recorded reason. Advisory throughout; rustc remains
  the authority. (§4, §6.5)

### 11.4 Fixtures and exit criteria

- **A18 — `reqwest` slice.** `default-features = false`, explicit `blocking` and `rustls-tls`, chosen
  roots variant, deterministic loopback server, generated Rust compiled with warnings denied, run.
- **A19 — a second, deliberately dissimilar crate.** Synchronous, non-network, data-shaped, different
  API idiom, through the same machinery with no special casing. If it needs a code change, the design
  was not generic. (§1.1)
- **A20 — goldens** for generated Cargo and generated Rust; accepted and rejected dependency fixtures;
  lock and feature mismatch diagnostics.
- **A21 — conformance cases** for an uppercase identifier and an underscored identifier in user code,
  and for a verbatim projected name, covering A1.

### 11.5 To carry into milestone 25 when it is written

A1 is a prerequisite of verbatim projection rather than a footnote to it, so the milestone must list
the lexical change, the lint demotion, and A21 as explicit deliverables. Not applied to
`docs/compiler-plan.md` yet.

### 11.6 Deferred

Migrating `/core/platform-data` and the `platform_*.rs` shims onto this machinery. The general path
does by hand what they do by hand today, so they could move onto it once it exists. Not part of this
work, and the design should not make it impossible.
