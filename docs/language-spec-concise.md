# Terrane AI language and compiler reference

SOURCE_OF_TRUTH: `docs/language-spec-and-compiler-architecture-draft.md`
ROLE: lossy retrieval/index layer for AI agents; not an independent specification.
SYNC_RULE: any semantic/grammar/architecture change to SOURCE_OF_TRUTH MUST update this file in the same work unit. If they conflict, SOURCE_OF_TRUTH wins.
IMPLEMENTATION_TRUTH: executable conformance cases define implemented behavior; this file includes planned/unimplemented language.
SELF_HEAL_RULE: when this reference is missing or unclear and SOURCE_OF_TRUTH resolves the question, update this file with the smallest durable rule/index improvement that prevents recurrence. Prefer compression, replacement, or a retrieval pointer over added prose; preserve fast scanning and bounded size.

## Retrieval map

| Need | Read here | Full spec |
|---|---|---|
| write/parse source | `LEX`, `GRAMMAR`, `CALL`, `DECL`, `CONTROL` | §§6, 9, 13–14, 34 |
| names/imports | `NAMESPACE`, `IMPORT`, `PRELUDE` | §§7–8 |
| types/numbers | `TYPE`, `INTEGER`, `OPERATOR`, `COERCION` | §§11, 17 |
| display/printing | `TEXT DISPLAY` | §9.6 |
| globals | `GLOBAL` | §20 |
| ownership | `VALUE`, `REF`, `MOVE`, `LIFETIME` | §12 |
| errors/callable contracts | `ERROR`, `CALLABLE` | §§15, 19 |
| collections/text | `COLLECTION`, `TEXT` | §16 |
| classes/protocols | `OBJECT_MODEL` | §§9, 18 |
| packages/interop | `PACKAGE`, `RUST`, `FOREIGN` | §§23–24 |
| async/targets | `ASYNC`, `TARGET` | §§21–22 |
| application testing | `TESTING` | §31.5 |
| compiler work | `COMPILER` | §§26–33, 36, 38 |
| unsettled/deferred | `OPEN`, `DEFERRED` | §§40, 42 |
| constitutional rules | `INVARIANT` | §41 |

## STATUS

- Normative design specification; implemented surface is tracked separately, and inclusion does not imply implementation.
- Rust is canonical lowering; no bespoke production VM required.
- Generated Rust is deterministic, readable, inspectable, source-mapped.
- Everything is semantically an object; representation may specialize when behavior is identical.
- Compiler owns source files/spans/tokens/syntax/semantic IR/diagnostics.
- Never silently repair unsupported/ambiguous source.

## LEX

```yaml
encoding: UTF-8
layout: indentation-delimited; NEWLINE/INDENT/DEDENT
empty_block: legal; no pass/no-op statement
comments: ['# line', '// line', '/* first terminator closes */']
identifier_case:
  legality: uppercase and underscore are legal; user declarations may use any case
  verbatim_projection: third-party member/type names retain their Rust spelling and are exempt from Terrane naming lint
  convention: kebab-case remains mandatory for compiler-owned and standard-library names and every documentation example
  lint: kebab-case is advisory, opt-in, and off by default for user code
  namespace_segments: unchanged lowercase ASCII with hyphens; Rust module `_` maps to `-`
charset:
  v1: ASCII only, per the version-one identifier policy
  namespaces: ASCII PERMANENTLY - non-ASCII segments hit the filesystem, where macOS NFD and Linux NFC produce different bytes for one identifier
  post_v1_extension: non-ASCII permitted in non-namespace identifiers only, and only with UAX #31 for the character set, NFC for equality, UTS #39 mixed-script confusable linting, and bidi control characters rejected outright (CVE-2021-42574, Trojan Source)
  ordering: widening an identifier set is backward compatible, narrowing is not; ship ASCII and extend later
identifier:
  version_1_characters: ASCII letters, digits, underscore, and normative joiners
  start: ASCII letter|underscore
  continuation: ASCII letters|digits|underscore|joiners
  joiners: punctuation admitted by normative grammar
  exact_identity: punctuation retained; no normalization
  examples_valid: [http2, sha256, ipv4-ipv6, foo+bar, sha3-256sum, ClientBuilder, parse_json, parse_json_]
  declaration_reserved_names: [function, instance, self, this]
  permanent_identifier: compact letter-joiner-letter, e.g. total-count
  lexical_error: terminal joiner + digits-only unit, e.g. count-1, x+4
  slash_excluded: '/' is the namespace separator, NOT an identifier joiner; a character cannot be both without making 'namespace foo/bar' ambiguous
  fix: insert operator spaces, e.g. count - 1
operators:
  spaced_infix: 'a + b'
  compact_letter_form: 'a+b' is identifier
  left_attached: 'a+ b' requires declared postfix behavior, otherwise error
  right_attached: 'a +b' is infix because preceding whitespace starts operator
numeric_literal:
  forms: [decimal digits, one optional '.' fraction, '0x' hex run]
  separator: "'_' between digits only; never leading, trailing, doubled, or beside '.'"
  absent_in_v1: [exponent, radix prefixes other than 0x, type suffix]
  lexical_error: digit run followed by identifier characters, e.g. 1e9, 0b101, 123abc, 0x
  dot_rule: "'.' joins a literal only before a digit; 1.type stays member access"
member_dot: no whitespace: value.member; '.' has NO other role in the language
invalid_adjacency: 'value member'
structural_statement_start_words: [if, for, while, select, try, throw, return, break, continue]
structural_word_resolution: a structural word at statement start does not resolve an ordinary binding of the same name
newline: normally ends statement; grammar-defined continuation only
```

Text literals:

```terrane
'single quoted default'
>rest of physical line is literal text
>>
  indentation-delimited multiline text
  common structural indentation removed
```

`>`/`>>` text is valid only in expression-start position, including directly after a value-bearing `return` whose keyword starts a logical line/block statement. Member names such as `.return` and `throw` do not trigger text lexing. Tail/block text cannot be a non-final ungrouped subexpression. Preserve content exactly per full spec §6.7.

## NAMESPACE

```yaml
separator: '/' - one delimiter for every boundary, including the root anchor
namespace_declaration: 'namespace my-output/formatters'
root_anchor: '/leading' - same character as the separator
relative_parent: '../tier', '../../tier'; repeated parents nest as ordinary path components
relative_current: unanchored path
segment_grammar: '[a-z]([a-z0-9]|-[a-z0-9])*' - lowercase ASCII letter, then letters, digits, internal hyphens
segment_vs_identifier: DISTINCT production and a strict subset; identifier admits joiners + * % < >, a segment admits only '-'; 'foo+bar' is a legal identifier and an illegal segment; never reuse the identifier production for segments
segment_allowlist_rationale: excludes / \ : * ? " < > | NUL, control chars, leading/trailing space, dot, '.', '..' BY CONSTRUCTION; no blocklist needed
segment_reserved: con prn aux nul com1..com9 lpt1..lpt9 (Windows devices, reserved with any extension); empty segment
segment_reserved_rationale: made of legal characters, so the allowlist cannot exclude them; reserved now because adding later breaks existing names
uppercase: parses, then REJECTED semantically with a fixit; never silently lowercased (principle 5, no silent repair)
identity: exact source spelling
package_sources: manifest declares namespace-root to directory-root mappings; discovery is bounded to declared roots
filename_mapping: namespace tree corresponds to directory tree; a declaration that disagrees with its location is an ERROR unless the manifest declares that mapping
root_mapping: 'foo/bar -> ./some/path' makes foo/bar/dave resolve under ./some/path/dave
overlap: longest matching namespace prefix wins; two roots mapped to one directory is a manifest-load error
unmapped_file: a .trn file no mapping reaches is not part of the package; it is neither compiled nor reported
third_party: a dependency's namespaces come from ITS manifest and are never discovered by scanning its tree
determinism: the resolved source set is recorded in build metadata; sorted expansion, ambiguity is an error
lookup: ONE view; a name resolves through lexical scope, then namespace, then program-global, then prelude
lookup_from_a_function_body: the namespace tiers yield constants, constructs, imports, functions, types - NEVER a namespace variable; mutable state crosses a function boundary only as 'global', a parameter, or a return
dot_rule: '.' appears only between a receiver and its member, never as a name prefix
dot_exception: NONE - the dot has no other role anywhere in the language
scope: lexical + namespace
namespace_variable_scope: the namespace tier ONLY - not its own function bodies, not descendant namespaces, not importers; its role is composition at the tier
leaving_the_tier: a value that must leave is a 'constant', a 'global', or a function result
collision: selective import introduces different object under same name in same scope => S2011
shadowing: nearer binding shadows farther binding
reimport_same_export: idempotent, including namespace-wide reimport; no warning
namespace_wide_visible_chain: lexical scopes -> current/parent namespaces -> globals -> /core/types -> prelude; shadow different object => W4004
namespace_declaration_precedence: top-level declaration in importing namespace always keeps its name; conflicting namespace-wide object skipped => W4004
namespace_wide_order: otherwise last import wins; multi-file order is normalized relative source path; replacement warning points to invalidated earlier import
retain_both: establish selective alias before namespace-wide import
```

Top-level plain assignment is namespace-local, including root namespace. `global` explicitly creates/replaces program-global identity and does not erase lexical provenance/visibility. A namespace variable is readable and writable only by other namespace-level declarations in that namespace, so `public` on one is meaningless and rejected rather than accepted as documentation.

```terrane
namespace application/commands
from /core/collections import map
private cache = map;
global shared-limit int = 10
```

## IMPORT

```terrane
use (system) sqlite
from /image/codec import resize
from /core/collections import map as ordered-map
import /core/filesystem
from ../shared/config import settings
import with custom-import
```

Rules:

- `use` declares a build dependency; it does not automatically bind supplied names.
- `from ... import x` binds ordinary `x` in the scope containing the import; `as` renames it. No declare-then-bind step.
- `import /namespace` binds every public function-body object under its declared name; identities stay in the source namespace and private objects remain hidden. It checks the complete visible chain (lexical parents, namespace parents, globals, implicit `/core/types`, then prelude) and emits `W4004` before shadowing a different object. An authored top-level declaration in the importing namespace always retains its name, so the conflicting imported object is skipped with `W4004`.
- Namespace-wide imports otherwise replace one another in source order; multi-file packages use normalized relative source-path order, and a replacement warning identifies the earlier import binding it invalidates. Identical reimports are silent. Namespace-wide imports have no alias clause. Retain both objects by establishing a selective `from ... import ... as ...` alias before the namespace-wide import. Selective same-scope collisions remain `S2011`. `/deps/*` remains selective-only until dependency projection supports namespace-wide imports.
- Imports at every lexical depth load their bundled core package and contribute its capability requirements; discovery never leaks their names out of the declaring scope.
- Prelude names and descriptor constructs need NO import: `print; value` and `value int8 = 42` are complete programs. Importing `print` or `int8` is redundant, not required, and should not appear in examples or fixtures unless the case is specifically about importing.

- Imports are structural compile-time slots, never ordinary calls/bindings.
- Importer selection is scoped; `global import with` selects program fallback.
- Ordinary binding named `import` cannot affect importer selection.
- Version-one execution: only declared precompiled/versioned host extensions run as importers/modifiers; never recursively execute arbitrary Terrane source.
- Structural stage order: manifest+lockfile -> host extensions -> imports in source order -> namespaces -> build selection -> resolve/type/modifiers.
- Import plans/inputs enter deterministic cache keys.

## PRELUDE

Version-one default ordinary program-global bindings EXACTLY:

```text
print task-scope utf8 utf16-le utf16-be utf32-le utf32-be
```

- These seven need no import. `print; value` is a complete statement and `scope = task-scope;` creates a structured-concurrency scope in a program with no import lines at all.
- Prelude may be disabled.
- Explicit `/core` object imports still work and may shadow/replace defaults deliberately.
- Library facilities such as `map`, `list`, `range`, and filesystem tools are NOT implicit prelude bindings; import them.
- Every `/core/types` descriptor — scalar, fixed-width, abstract category, `string`, `bytes`, and `none` — is construct vocabulary available without import independently of prelude selection.

```yaml
prelude_bindings: the seven ordinary program-globals listed above
descriptor_constructs: every descriptor registered under /core/types
construct_availability: usable in construct position without import ('value int8 = 42')
construct_value_use: still rejected in value position; a construct is not a runtime value
explicit_import: remains available for rebinding, aliasing, and shadowing ('from /core/types import int64 as word')
```

## CALL

```terrane
thing;                         # explicit zero-arg default call
print; message                 # positional arg
connect; host, port, timeout = 10
buffer.clear;                  # zero-arg instance-member call
widget::default;               # zero-arg static-member call
instance widget;               # zero-arg class construction
print; render                  # a bare name passes the object
print; (render; report)        # nested call MUST be grouped
```

Rules:

```yaml
call_marker: semicolon
zero_arg: semicolon required
member: receiver.member (no whitespace before dot) selects an instance member
static_member: class::member selects a class/static member; '.' and '::' are never interchangeable
construction: 'instance class; arguments'; semicolon required with zero arguments; class name invocation never constructs
member_kind: semantic, never inferred from arity; property selection yields receiver state/classification, method selection yields a callable object and requires ';' even with zero parameters
adjacency: 'receiver object' invalid; NEVER invocation
call_extent: unwrapped call owns remainder of containing logical expression
arguments: one comma-separated list; optional '(' immediately after ';' delimits wrapping
argument_layout: parentheses are general explicit expression continuation; newline/indent/comments and block strings inside remain non-structural; closing outermost ')' restores logical-line termination
argument_calls: ungrouped calls forbidden; delimited argument list admits nested calls
three_clause_for: its semicolons belong to for; calls in clauses parenthesized
evaluation: left-to-right
receiver: evaluated before selection
and_or: short-circuit
other_binary: both operands evaluated
default_args: call site, after supplied args, parameter order
```

## GRAMMAR

Compact precedence, high -> low:

```text
postfix member/index/call   (NOTE: '++'/'--' are NOT here - they are update STATEMENTS)
prefix: not - ~ ; shared ref / ref / move / await consume postfix operand
* / %
+ -
<< >>
&
^
|
comparisons (non-associative)
is / is a
and
or
```

- Arithmetic/shift/bitwise/`and`/`or` associate left.
- Prefix operators associate right.
- Comparisons do not chain: use `a < b and b < c`.
- Unary `+` absent.
- `shared ref` is one compound type/value prefix; bare prefix `shared`, `ref ref value`, `shared ref ref value`, and `move move value` rejected.
- Parentheses override precedence and re-enable nested calls.
- Assignment target: bare mutable binding or assignable member/index path only. Receiver/indices evaluate exactly once left-to-right before value.
- Bare `name = expr`: declare where permitted if unresolved; otherwise rebind mutable resolved name.
- Qualified/uninitialized declarations use explicit binding grammar.

Canonical statement inventory (some not version-one implementation scope):

```terrane
namespace, use, from/import, import with
binding/declaration, assignment, expression
function, class, interface, trait
if/else, while, for-in, three-clause for
return, break, continue
goto/label
try/catch/finally, throw
yield
rust block
```

Compound clauses align with owner. Empty bodies legal. `return` expression optional; `throw`/`yield` expression required; version-one `break`/`continue` have no value. `try` requires catch or finally.

## DECL

```terrane
name = value
name int = 42
name string
constant max-size int = 1024
private cache = map;
global service = service;

function main;

function add int; left int, right int
  return left + right

function connect connection; host string, port int, timeout int = 10
  ...
```

- Type expression follows binding/parameter name.
- A typed binding may omit its initializer (`name string`); flow-sensitive definite assignment must prove a value before any read, reference, member access, argument pass, or capture.
- Bare `_ = expression` and `_ T = expression` are discard bindings. They require an initializer, accept no declaration qualifiers, evaluate the expression exactly once with the optional destination type, create no readable binding, suppress binding/store warnings, and release the result at statement end. A longer name beginning with `_` remains an ordinary lexical binding and retains its value to the normal scope endpoint; it suppresses `W4001`, but its first read emits `W4006`.
- [binding-initialization-dependencies] An initializer resolves against the scope as it stands immediately BEFORE its declaration, so the declared name is not in scope from its own initializer. Where nothing else binds that name, reading it — directly or through a called function — is a compile-time error naming the absent binding. Namespace initializer dependencies, including later namespace-level assignments folded into initialization, must be statically acyclic and rejected before lowering when they form a cycle.
- [redeclaration] Where the name is already bound in the SAME LEXICAL scope, the initializer reads the earlier binding and the declaration REPLACES it: `a int8 = 12` then `a int = a`. One name means one thing at each point in a scope, read top to bottom. Lexical only — a namespace top-level declaration may not replace another, because namespace initialization is ordered by dependency, not source position.
- [redeclaration-identity] after evaluating the initializer, replacement releases the old owned value and installs a new identity; identical type is an assignment with a redundant annotation, not identity preservation. Existing `ref` becomes unusable at release; `shared ref` continues owning the old identity and is never retargeted.
- [redeclaration-retype] type changes => the binding's type changes. Release remains deterministic and occurs at replacement rather than scope exit, so an unreachable resource is not retained.
- [block-scope] Function bodies and every indented control-flow body create lexical scopes. A nested declaration is visible through that body and deeper scopes, never in sibling bodies or after exit; its value is released on each exit. A `for` target spans its loop body only. A nearer declaration shadows until exit, while untyped assignment to an enclosing name assigns that existing binding.
- Function result type follows the function name. The complete header ends with a mandatory semicolon, followed by the parameter list; `function main;` declares no parameters. The same marker is required for methods, interface requirements, lifecycle methods, and anonymous functions. For multiline parameters, `(` must be the first non-trivia token after the semicolon on the declaration line; newlines and indentation are non-structural until its matching `)`, commas alone divide parameters, and `)` may share the final parameter's line. Preferred form: one parameter per line with `)` on its own line; other layouts inside the delimiters remain legal.
- A final `name T ...` parameter captures zero or more remaining positional arguments as an ordinary `list of T`; it cannot have a default or be bound by name. Fixed required and optional parameters may precede it. Calls evaluate and convert arguments left-to-right, and fixed versus variadic arity is part of callable types (`function from T ... to R`), interface/trait conformance, and deterministic identity. An omitted `T` follows ordinary finite inference/defaulting and never creates an unbounded universal value. Rust `Fn` projection remains fixed arity; C ABI variadics require an explicit foreign adapter.
- Named arguments require stable exposed parameter names.
- `constant`, not `const`.
- Default visibility public; strict visibility mode can require explicit qualifiers.
- Source-declared type parameters/generics are unsupported and MUST be rejected. Use concrete types, unions, interfaces, or generated concrete declarations. This does not prevent the dependency projector from selecting one Rust result-only generic from an explicit Terrane destination; Rust angle-bracket arguments never become source syntax.

## TYPE

Core:

```yaml
int: exact arbitrary-precision signed semantic value; adaptive representation
float: a SPELLING of float64, not a separate type; one canonical descriptor, so '.type', 'is a', reflection, and diagnostics all report float64
float_meaning: 'float' denotes THE DEFAULT PRECISION whatever that currently is; 'float64' denotes binary64 PINNED. Same descriptor this version, different meaning over time - which is what makes the default repointable
float_intent: code in 'float' moves with the language; code in 'float64' stays pinned because it must (wire format, foreign ABI, binary layout, beside float32)
float_default_reason: failure modes are asymmetric - wanting float32 and getting float64 wastes memory (found by profiling, fixed locally); wanting float64 and getting float32 computes wrong answers (integers stop round-tripping above 2^24: timestamps, byte counts, money in minor units)
float_future: may be repointed at a VERSION boundary, never by target or profile - the same source computing different results per build is what 'int' avoids by being semantically fixed
bool: true|false
string: Unicode text, UTF-8 standard representation
bytes: arbitrary binary
none: singleton absence value
void: no produced value; not storage/type erasure
opaque: hidden representation type; not void
fixed_signed: int8,int16,int32,int64,int128
fixed_unsigned: uint8,uint16,uint32,uint64,uint128
fixed_float: float32,float64
abstract: number, integer, fixed-integer, signed-fixed-integer, unsigned-fixed-integer, floating
abstract_roots: value, object (identity/ownership categories; carry no numeric members)
union: 'T|U'; none is ordinary union member
constructor: 'list of string'; arguments classified semantically as type or compile-time value
function_type: 'function from A, B to R [throws T]' | 'async function from A, B to R [throws T]'; associates right, and each postfix throws clause binds to the nearest function type
```

- Values always have types; an unconstrained binding may be dynamic without weakening values. Numeric constant expressions are the exception before context: their spelling denotes a mathematical constant but a destination or typed operand selects its numeric type and arithmetic.
- Numeric values cross a single declared destination exactly or throw; mixed integer values promote exactly. Arithmetic across integer/floating values or unrelated categories remains rejected without an explicit policy conversion.
- Written coercion is object-driven and selects a different conversion policy; it is not permission merely to satisfy a numeric destination.
- Abstract descriptors are interface/category contracts exported from `/core/types`, never prelude names. `int` implements `integer` and `number`; fixed widths add `fixed-integer` plus their signedness contract; `float`/`float32`/`float64` implement `floating` and `number`. Conformance drives member attachment and finite-union reasoning; it creates no storage supertype.
- Type violations compile-time when provable.
- Conditions invoke truth protocol.
- `==` value equality; `is` source-visible identity; `is a` type membership, not numeric destination convertibility. A typed `int8` value is not an `int`; a numeric constant uses the queried type as context, so `42 is a int8` is true and an inadmissible constant answers false rather than failing. `===` invalid.
- `c is a` is identity against binding `a`; `c is a widget` is membership when complete type follows.
- Ordinary scalars/strings/collections are identity-less: `is` is false even for `x is x` and `42 is 42`. Explicit refs, linear resources, and canonical descriptors carry identity; ref/shared-ref pairs compare true exactly when both denote the same referent. Descriptor metadata exposes the boolean `inherently-identity-bearing`; it is true for ref types/resources/descriptors and false for ordinary values and collections. Exact-type-and-value comparison is `left == right and left.type is right.type`.
- Type descriptors are language constructs backed by canonical compiler-owned objects, not independently instantiated values.
- Class, interface, and trait identity is nominal and namespace-qualified: `(declaring namespace, declared name)`. Import aliases change spelling, not identity; same-named declarations in different namespaces are unrelated types, and diagnostics qualify them when the short form is ambiguous.
- Named built-in descriptor identity is namespace-qualified: scalar descriptors use `/core/types::<name>` and instantiated collection descriptors use `/core/collections::<family> of <arguments>`. Synthesized composed descriptors such as references, callables, optionals, and unions retain their canonical source-shaped spelling unless the language assigns their family a named namespace identity. Alias spellings never replace canonical identity.

```yaml
binding: REJECTED - 'd = int8' would store a type in a value slot; a construct is not a value to bind
rename: at the IMPORT only - 'from /core/types import int8 as byte'; one spelling per name in a scope, renamed where the name enters it
rename_use: the renamed construct is legal in annotation position, coercion destination, and 'is a' right side
type_in_a_value: holding a type to dispatch or instantiate through it is a DISTINCT capability belonging with reflection; it needs its own construct, not assignment syntax
value_use: REJECTED at the source span - no display or value protocol in v1 (print; d, arithmetic, value parameter)
lowering: a statically resolved descriptor needs NO runtime storage and lowers to nothing
materialisation: reflection or dynamic descriptor use may require the canonical descriptor object at runtime; 'not an ordinary value' does NOT mean 'never has a runtime representation'
defect: emitting a plain Rust binding for a descriptor, as if it were an ordinary value, is a compiler defect
backing_object: real - type returns it, 'is a' compares it, identity survives rebinding, reflection exposes it later
```

- Type descriptors are semantic objects with stable canonical identity, not ordinary values. Version-one type expressions/coercion destinations must resolve to finite compiler-known descriptor alternatives; lowering may erase the descriptor only when source behavior is unchanged.
- Version-one descriptor contract: built-ins and source-declared classes/interfaces/traits must share one compiler-owned model. It answers member lookup, nominal relations, interface conformance, dispatch metadata, reflection, and general structural-protocol satisfaction. Iteration and `truth` are protocol-member queries, not parallel hardcoded class tables. Current delivery status is tracked in the compiler plan and scoreboard, not by this normative requirement.

- Union destinations choose an exact type match first, otherwise the unique arm admitted by contextual constant typing or numeric destination conversion. Multiple admitted arms are a compile-time ambiguity; arm order never decides. Repeated arms normalize by canonical semantic identity and each authored duplicate reports `W4003`; aliases of one descriptor are duplicates.
- `T|none` is a declared type anywhere a source type is accepted: bindings, parameters, and returns. A direct guard `value != none`, `none != value`, or `not (value is a none)` narrows that named binding to `T` in the guarded block; `and`/`or` combinations do not, and assignment invalidates the fact. Member/index/call expressions are not narrowing subjects because repeated evaluation may change; bind once, then guard and use the stable name.
- Static optional return postcondition (distinct from block narrowing): a concrete `T`-returning function or method may return an exact static `T|none` member after a preceding sibling `if member == none` / `if none == member` with no `else`, exactly one direct compatible assignment to the same textual member in that body, no other body write, and no intervening write. Every unmet condition leaves the return expression `T|none`; instance members and other member expressions never receive this proof.

## INTEGER

```yaml
int_semantics: mathematical exact signed integer
runtime_tiers: i64 -> i128 -> arbitrary precision limbs
overflow: representation promotion, NOT source throw
normalization: after every operation choose smallest exact tier
fixed_width: distinct types; retain width; ordinary arithmetic checked
signed_division: Euclidean quotient/remainder
host_lowering: direct Rust operators only if complete semantics match
capability: target without arbitrary promotion must prove bounds or reject; never silently bound int
bitwise_int: infinite two's-complement
right_shift: arithmetic/flooring
left_shift: exact
negative_shift: throws negative-shift-count
```

- Small multiplication computes exact `i128` intermediate; wider operations preserve exactness.
- Division by zero throws `division-by-zero`.
- Fixed widths require explicit `checked`/`wrap`/`saturate`/`overflowing` family children, never host build-mode behavior; fixed-width shift counts need their own source-language contract rather than inherited host behavior.


- A constant expression is a literal, unary-negated literal, parenthesised constant, or compile-time arithmetic combination. Its whole-number/decimal spelling does not fix a type. Typed binding initialization or assignment, a parameter default, a declared argument or return, a declared element or field, and a typed numeric operand supply context.
- Integer constant folding uses exact arithmetic with unbounded intermediates and checks only the final destination value. Floating folding performs each operation at destination precision, matching runtime arithmetic rather than rounding an exact result once; finite decimal/non-integral results may round normally, but an integral whole-number value must be exactly representable. An admitted constant materialises directly with no conversion/check. Outside context, whole-number constants are `int` and decimal constants are `float`.
- With one typed numeric operand, a constant takes that type; shift counts are exempt. Two differently typed integer values promote to the smallest integer type containing both source ranges, or `int`. Integer/floating value mixtures remain rejected.
- Numeric destinations admit every numeric source exactly or throw: range-contained widening has no representability check or conversion-error path; narrowing checks; integer-to-float requires exact representability; float-to-integer requires finite, integral, in-range input. Declared types and constant evaluation decide acceptance; range analysis may remove checks, never decide it.
- Named arithmetic families attach to `integer`: `add`, `subtract`, `multiply`, `divide`, `remainder`, `div-rem`, `negate`, `shift-left`, `shift-right`. Operators invoke each family's default child.

```yaml
policy_children: checked | wrap | saturate | overflowing
policy_receiver: wrap/saturate/overflowing attach to fixed-integer ONLY; int has no bounds to wrap or clamp
int_children: throwing default always; checked only where genuinely fallible (divide, remainder, div-rem by zero)
overflowing: returns 'overflow-result of T' with value T and overflowed bool
div_rem: returns 'div-rem-result of T' with quotient T and remainder T; default and checked only, never wrap/saturate
div_rem_reason: a wrapped quotient breaks the quotient/remainder identity the result object exists to guarantee
shift_fixed: default and checked reject counts outside the width; wrap reduces count modulo width; saturate absent
shift_int: shift-left unbounded and total; shift-right arithmetic; no count-policy children
postfix: '++' and '--' are STATEMENTS, never expressions; they produce no value
postfix_rationale: expression-valued increment is the source of C read-modify-write sequencing problems and buys nothing; write the two operations
postfix_policy: they select the default add/subtract child only; other policies need explicit assignment
```
## FLOATING-MATH

```yaml
receivers: float32 | float64
method_shape: computational members are methods; floating results preserve receiver type
properties:
  finite: neither infinity nor NaN
  infinite: either infinity
  not-a-number: NaN
  negative-sign: IEEE sign bit, including negative zero and signed NaN
  zero: either signed zero
  normal: finite non-zero value with a full-precision significand
  subnormal: finite non-zero value below minimum-positive-normal
roots_powers:
  square-root: 'value.square-root;'
  cube-root: 'value.cube-root;'
  hypotenuse: 'value.hypotenuse; other'
  power: 'value.power; exponent' -> same floating type
  integer-power: 'value.integer-power; exponent int32'
exponentials_logs:
  exponential: base e
  binary-exponential: base 2
  exponential-minus-one: exp(value)-1 without near-zero cancellation
  natural-log: base e
  natural-log-one-plus: ln(1+value) without near-zero cancellation
  binary-log: base 2
  decimal-log: base 10
  logarithm: 'value.logarithm; base'
trigonometry:
  sine: radians
  cosine: radians
  sine-cosine: tuple [sine, cosine], combined target operation when available
  tangent: radians
  arc-sine: radians
  arc-cosine: radians
  arc-tangent: radians
  arc-tangent-two: 'y.arc-tangent-two; x' -> quadrant-aware angle
scalar:
  absolute: clears the sign bit
  copy-sign: 'magnitude.copy-sign; sign-source' -> magnitude bits with source sign bit
  minimum: number-preferring minimum; -0 precedes +0
  maximum: number-preferring maximum; +0 follows -0
  clamp: 'value.clamp; lower, upper' -> ordered minimum/maximum; NaN bound or lower > upper produces canonical NaN
  fractional-part: value minus truncation; signed zero follows receiver
  multiply-add: 'value.multiply-add; multiplier, addend' -> one fused final rounding
algorithm:
  next-up: next representable value toward +infinity
  next-down: next representable value toward -infinity
  decompose: 'value.decompose;' -> object with mantissa Receiver and exponent int32; finite non-zero value = mantissa * 2^exponent and 0.5 <= abs(mantissa) < 1
  scale-binary: 'value.scale-binary; exponent int32' -> value * 2^exponent with one destination-precision rounding
descriptor_constants:
  radix: int value 2
  significand-digits: int value 24 for float32, 53 for float64
  epsilon: distance from 1 to the next larger value
  minimum-positive-normal: least positive normal
  minimum-positive-subnormal: least positive representable value
  minimum: least finite value
  maximum: greatest finite value
edge_contracts:
  no_throw: floating domain/range conditions produce IEEE NaN, infinity, signed zero, overflow, or underflow
  nan: unary target operations preserve NaN category; payload/sign are unspecified except copy-sign, next-up/down, decompose, and receiver-NaN clamp retain the input bits
  hypotenuse: infinity wins over NaN
  clamp: receiver NaN is retained; an invalid bound pair produces canonical NaN
  fractional-part: NaN or either infinity produces NaN
  next: NaN and the outward infinity are unchanged; moving inward from infinity yields the corresponding finite extreme
  decompose: signed zero and non-finite receivers are returned unchanged with exponent zero
rounding: every operation runs at receiver precision; scale-binary has one ties-to-even rounding; only multiply-add is fused
accuracy: exact bit operations and classification are exact; elementary/transcendental operations use the target primitive's documented result with no portable ULP guarantee
reproducibility: deterministic for one compiler version, target, standard library, and float mode; cross-target bit identity is not required
lowering: direct target primitive or smallest compiler-owned scalar support routine; no scientific dependency
excluded: Bessel, incomplete gamma, distributions, linear algebra, arrays
```

- Classification and `negative-sign` are properties and cannot be invoked. Every other member above is a method, including zero-argument methods.
- Same-floating arguments must admit the receiver type. `integer-power` and `scale-binary` require an `int32` destination-compatible argument. `decompose.exponent` is `int32`.
- `minimum` and `maximum` return the numeric operand when exactly one input is NaN and return NaN when both are NaN.
- `power`, logarithms, inverse trigonometry, roots, and exponentials inherit the stated target primitive's IEEE special-value selection. A negative finite base with a non-integral floating exponent and an out-of-domain logarithm or inverse trigonometric input produce NaN.
- Compile-time evaluation, if provided, must match the runtime contract and approximation policy.

## COERCION

```yaml
form: receiver family/policy; 'value.coerce; destination-type' | 'value.coerce.checked; destination-type' | 'value.coerce.lossy; destination-type' | 'value.coerce; destination-type, converter'
family: invocation is the throwing default | coerce.checked | coerce.lossy | coerce.wrap | coerce.saturate
default_child: 'default' exists in compiler metadata for reflection only; source lookup of 'default' is rejected
implicit_numeric_destination: assignment/argument/return/element/field accepts exactly or throws; no written coerce required
exact_widening: source range contained by destination exact values; representation change, no representability check/conversion-error path
checked_narrowing: direct representability check; integer destination failure throws integer-conversion-overflow
float_to_integer: succeeds only for finite, integral, in-range values; otherwise integer-conversion-overflow
integer_to_float_implicit: succeeds only when this integer is exactly representable; otherwise throws
integer_to_float_written: 'value.coerce; float-type' requests IEEE round-to-nearest, ties-to-even; inexact result is ordinary
float_narrowing: exact finite values, signed zero, and signed infinity arrive with sign preserved; rounded finite values and every NaN throw integer-conversion-overflow
float_to_integer_written: NO declared coerce pair - choosing an integer for a fractional value needs a rounding mode and coerce never takes one; 'ratio.coerce; int' is absent while 'count int = ratio' is admitted
float_rounding_methods: round (ties-to-even) | floor | ceiling | truncate; each is invoked with ';' and yields an integer before destination conversion
float_out_of_range: written coerce throws coercion-error; never yields an infinity
float_nonfinite_written: floating-to-floating coerce preserves signed infinity and NaN category; these are not finite-overflow failures
lossy_float_narrowing: 'float64.coerce.lossy; float32' is the sole declared lossy pair; it is infallible IEEE round-to-nearest, ties-to-even, preserving signed zero/infinity and NaN category, allowing underflow to signed zero and overflow to signed infinity; NaN payload preservation is not portable
string_parse: accepts exactly the destination's canonical text-display spelling
coerce_options: NONE - compiler-declared coerce takes only its destination; the optional second positional argument on bare coerce is the complete caller-supplied converter, never a radix or format option
parse_family: 'value.parse; callback' - the callback is REQUIRED; there is no built-in destination-owned parse
parse_result: result type comes from the callback's declared return, not from a destination descriptor
parse_checked: 'parse.checked; callback' catches a throwing callback and yields absence
parse_v1: callback must be a statically resolvable function name, not an arbitrary expression; resolved and inlined like a coercion destination
parse_union: a union return is checked at the destination by ordinary union rules; no parse-specific recheck
radix: third distinct operation - 'text.radix; 16' -> int (interpret), 'value.radix; 16' -> string (render)
radix_narrowing: ordinary coercion, grouped per call extent: '(text.radix; 16).coerce; int8'
declared: conversions are declared per source/destination pair; an undeclared pair is absent from the type, not a runtime failure
bool_to_int: declared, total, lossless (false 0, true 1)
int_to_bool: NOT a conversion; use an explicit comparison
failure_value: default child throws, checked returns none; neither substitutes a value
lenient_child: a total 'substitute on failure' conversion (PHP intval style, 0 for unparseable) is allowed ONLY as a separately named child, never as plain coerce; optional and unspecified in v1
callback: "'value.coerce; Destination, converter' admits an otherwise undeclared pair; converter is an ordinary synchronous 'function from Source to Destination', both arguments must be positional, exactly one callback is invoked exactly once, and ordinary declared throws propagate"
locale_parse: imported formatting facilities only, never coerce
universality: no guarantee any type coerces to any other
destination: version-one destinations resolve to finite compiler-known descriptors
lowering: contextual constants materialise directly; widening changes representation; checked conversion narrows/widens-back/compares; equivalent implicit and written integer narrowing emit equivalent checks
```

- The receiver evaluates once before policy selection and destination arguments. The whole call resolves policy availability statically; a selected family is not a storable bound-method value in version one.
- `checked` returns `T|none`; `wrap` and `saturate` exist only for supported fixed-width destinations. Flat `checked-coerce`, `wrapping-coerce`, and `saturating-coerce` spellings are invalid.

## TEXT DISPLAY

- Core text display returns `string`; version one implements it for strings, all integer types, all float types, booleans, and `none`, but not arbitrary `bytes`.
- Integers render base ten without grouping; floats use shortest round-trippable decimal text and preserve negative zero; booleans/absence render `true`, `false`, `none`.
- Core `print` displays arguments left-to-right and appends a newline. Unsupported display is a typed error; locale, styling, width, and precision require imported formatting facilities. Float lowering must normalize Rust's `NaN` spelling to canonical `nan` while also pinning `inf`, `-inf`, negative zero, and shortest round-trippable finite output.
- Version-one dynamic alternatives are finite and compiler-known, so protocol availability and typed-boundary compatibility are checked across all alternatives statically. Runtime display type errors are reserved for later or foreign erased dynamic values.

## VALUE / REF / MOVE / LIFETIME

```yaml
ordinary_assignment: copyable values use value semantics; resource-owning values transfer automatically
implementation: COW/share storage allowed if mutation cannot leak
mutation: separates backing storage before observable change
ref: explicit non-owning source-visible identity; does not extend lifetime
shared_ref: explicit shared identity plus shared ownership; extends lifetime
reference_provenance: compiler-tracked; derived references may narrow, never widen lifetime
interior_ref: separates COW, pins path, cannot escape/replace/remove while live
reference_provenance_shape: owner source span + external-lender parameter span + ordered Field/Element/CallResult projections + first mutation/move/replacement lifetime end; parentheses preserve it and bindings, arguments/results, captures, and borrowed iteration copy or narrow it
reference_lender_inference: a reference-returning callable must select exactly one reference parameter by return-flow analysis, including fixed-point propagation through calls; parameter count never chooses, and by-value/local/ambiguous origins reject
reference_elements: list/map/unordered-map element borrows are supported; owner mutation that can invalidate storage ends the element path; unsupported index owners reject before lowering
reference_lowering: a provenance-bounded ordinary ref lowers to a target borrow and reads without owner upgrade/lock/clone; selected reference-return lenders become explicit Rust lifetimes; shared ref alone keeps Arc/Mutex ownership, while an ordinary observer of the same explicitly shared owner uses a non-owning weak handle
reference_escape: returns are accepted only when provenance reaches the selected external lender; ambiguous call-result lender paths, local or by-value-parameter return, post-lifetime-end use, and ref-to-shared-ref promotion are source errors
resource_ownership: inferred transitively from compiler-known noncopyable fields; no 'linear class' qualifier
resource_assignment: transfers identity and makes source unavailable; no 'move' ceremony required; loop-body declarations initialize a fresh binding value on every iteration, so their consumed state does not cross the back-edge, while moved bindings originating outside the loop remain unavailable on later iterations until explicitly rebound
resource_argument: a statically non-copyable value passed by value transfers automatically; lowering may use a target-language move
move: explicit transfer request for a value ordinary assignment would copy; optional emphasis for an already-required non-copyable transfer
constants: cannot rebind
constant_scope: rejects reassignment regardless of lexical, namespace-local, or program-global identity tier
shadowing: a namespace-local binding may shadow a distinct program-global constant; writes resolve to the local identity
parameter_and_for_target_reassignment: allowed within lexical scope; value semantics preserve caller arguments and iterated collections
lowering_mutability: emit mutable target storage only when resolver-backed write analysis finds a reassignment
cleanup: deterministic lexical destruction; each independently owned source value has one lifecycle lineage and invokes each applicable `destruct` once when that lineage ends, ordered most-derived class to root base; value separation copies state into a fresh lineage, compiler representation clones cannot multiply the hook, transfer moves it, and `ref` never delays it
cycles: only `shared ref` forms ownership edges; the native target rejects statically provable strong cycles in descriptor fields, including through collection element types, while acyclic shared fields remain valid; later mutation-created cycles not visible in that graph are not collected and must be broken explicitly; ordinary `ref` back-edges are excluded
```

Reference choice, in expected order of frequency:

```yaml
ordinary_value: default; independent value semantics
ref_T: normal non-owning observer; preserves identity and requires proven lifetime
shared_ref_T: uncommon shared owner; preserves identity and keeps the value alive
```

Ordinary value assignment may share copy-on-write backing storage while no copy is mutated, making
read-only passing, returning, and assignment reference-cheap without shared source-level identity.
Mutation separates the value before it becomes observable elsewhere. Do not introduce a reference
merely to avoid a copy; use it only when aliases must observe the same identity and mutations.

Use `ref` when an alias observes an identity owned elsewhere, such as a bounded local alias,
child-to-parent back-pointer, subscriber, or cache entry. Direct access is accepted only while the
originating owner is proven alive; escape or use after its lifetime ends is rejected. Use
`shared ref` only when the alias must also extend the identity's lifetime. Lowering may optimize
representation but must never silently promote `ref` to `shared ref` or discard authored ownership.

[reference-async-suspension] A non-owning `ref` may cross `await` only when its originating owner is
proven alive throughout the suspended state. `shared ref` may cross by carrying ownership, subject
to the referenced value's thread-safety contract. Neither form changes ownership implicitly.

[reference-observation-transparency] A valid `ref T` or `shared ref T` exposes `T`'s ordinary
members, methods, and value consumers: `ref bytes` may call `decode`, and printing `ref int`
observes the integer. This is receiver/read transparency, not type conversion. Assignment,
parameter, and return boundaries retain the authored reference contract; neither `T`, `ref T`, nor
`shared ref T` silently becomes another.

Distinct contracts: `ref T`, `shared ref T`, `user-ref of T`, `raw-address of T`, `array-ref of T`,
`c-pointer of T`, callable ABI addresses. Never silently convert or weaken.

## CONTROL

```terrane
if condition
  ...
else
  ...

while condition
  ...

for item in things
  ...

for i = 0; i < limit; i++
  ...
```

- Three-clause calls require grouping: `for i = (start-at; limit); ...`.
- `++`/`--` are statement/update operations on compatible mutable numeric bindings. They produce NO value and cannot appear in expression position; grammar places them in `update-statement`, not `postfix-expression`, and they are permitted in a three-clause `for` clause.
- Labels/goto function-local; cannot enter deeper scope or cross initialization/lifetime/cleanup unsafely.
- `match` reserved shape but outside minimum compiler milestone.

## ERROR / CALLABLE

```terrane
from /core/errors import throwable

class config-error implements throwable
  message string = ''
  path string = ''
  function construct; path string, message string
    this.path = path
    this.message = message
  function render string;
    return this.message

// Executable coverage: tests/conformance/run/custom-throwable/case.trn.
function load string throws config-error; path string
  throw config-error; path, >configuration is invalid

- Every thrown value MUST statically conform to structural `throwable`; arbitrary dynamic values are
  never throwable.
- `throw expression` throws an existing instance; `throw class; args` ordinarily invokes the class
  constructor and throws the resulting instance.
- `throwable` surface: class-provided `message string` and synchronous, non-throwing, zero-argument
  `render string`; compiler-supplied `cause throwable|none`, defaulting to `none`, in the runtime
  envelope. Runtime also retains the concrete descriptor and deterministic source-context chain.
  Descriptor identity, never message text, drives matching.
- `/core/errors` defines `throwable` and EXACTLY these language-mandated implementing classes:

```text
arithmetic-overflow          checked fixed-width result outside receiver range, incl. signed MIN / -1
division-by-zero             zero divisor for / % div-rem, every integer type and mode
integer-conversion-overflow  exact-or-throw numeric arrival cannot preserve the source value: integer out of range, fractional/NaN/infinite float into an integer, or an integer not exactly representable by a float. Name is broader than 'integer'/'overflow'; see OPEN
negative-shift-count         negative shift count on unbounded int << >>
coercion-error               coercion has no compatible result outside the overflow case above
```

- `catch` clauses are tried in source order against compatible classes/interfaces; an unreachable
  later clause is a compile-time diagnostic.
- `finally` always runs and may replace a pending outcome only by explicitly returning or throwing.
- Each core throwable carries `message`, `cause`, context, and structured operation/type detail;
  other throwable classes are declared by packages/adapters, never implicitly synthesized.
- Recoverable throws lower through compiler-owned Rust `Result`-like flow; panic is separate and
  fatal.
- Generated errors use a 16-byte `kind + origin TerraneSite + optional boxed detail` header.
  Built-in messages derivable from `kind` stay lazy; message/cause/propagation frames allocate detail
  only when present. Fresh failures set origin once; propagation only appends frames, through
  type-distinct lowering helpers.
- `TerraneSite` is a compiler-owned, dense, deterministic per-program semantic-site ID. Emitted
  tables map it to file, enclosing callable, and exact source range; comments preserve local
  generated-Rust readability. Numeric IDs are golden-level lowering details, not cross-build keys.
- Tagged-pointer error packing is rejected absent a hard binary-size or measured calling-convention
  requirement: it trades derived safe ownership for generated `unsafe` without a throughput gain.
- Compiler infers the exact escaping throwable set transitively for every callable, public or
  private, after catches and `finally` replacement.
- Optional postfix `function name Return throws T; parameters` is an upper-bound contract, NOT
  effect narration: every escaping throwable must conform to T or compilation fails. Omission means
  inference, not `nothrow`.
- Reflection separately exposes `throwable-contract` (written upper bound, if any) and
  `escaping-throwables` (current inferred concrete set), even when private bodies are stripped.
- Callable compatibility admits fewer compatible throwables, never an incompatible one.
- A written callable type with `throws T` admits only implementations whose exact escaping set
  conforms to `T`. An implementation may be infallible or narrower. Omitting `throws` declares an
  infallible callable type. Every source callable has either exact inferred metadata or an explicit
  written storage contract; no implicit broad fallback is introduced.
- Nested callable results associate right. In
  `function from A to function from B to R throws Inner throws Outer`, `Inner` constrains the result
  callable and `Outer` constrains the outer callable. `async function` uses the same postfix rule.
- Exact named-function, closure, bound-method, and inferred-binding metadata survives callable
  conversion. An explicitly typed binding retains both that exact initializer summary for reflection
  and its written contract as the storage ABI and invocation bound. ABI selection follows the written
  bound independently, so widening a proven-empty implementation does not falsify its exact escaping
  set even though storage becomes result-bearing.
- Callable contracts are orthogonal, not one permission-like effect algebra:

```yaml
invocation:
  written: function = shared; mutable function; consuming function
  exact: inferred authority used by the body; may be weaker than written, never stronger
  substitution: shared -> shared|mutable|consuming; mutable -> mutable|consuming; consuming -> consuming
  environment: shared observes; mutable owns repeatable receiver/captures exclusively for the full call and serializes overlapping invocations of the same value; consuming transfers and is one-shot; separated mutable values have independent environments and serialization boundaries
  composition: invocation prefix precedes async; throws remains a postfix upper bound
throws: exact inferred escaping set plus optional written upper bound
async: invocation produces a task; `await` consumes a task and marks a possible suspension point
unsafe_boundary: only concrete adapters or `unsafe rust`; never a bare callable qualifier or generic unsafe block
derived_facts: suspension points, exact receiver/capture authority, `unsafe rust` use, I/O, allocation, blocking, shared mutation, and foreign transitions MAY be inferred for validation/tooling but are not source qualifiers
foreign_boundary: expressed by a concrete runtime/import/adapter/ABI construct; never a bare callable qualifier and never transitive to ordinary callers
purity: no `pure` qualifier; a future contract requires independently defined observable guarantees
```
- Reflection may group retained contracts and derived facts for inspection, but compatibility and
  validation apply each contract's own rules. Ordinary I/O requires no compiler-issued authority
  token.
- Reflection is ordinary value access: invoking a consuming callable consumes that value, so its
  contract metadata must be inspected before invocation or through a separate owning value.
- Uncaught throwables render deterministic cause/source chains; foreign failures preserve native
  traceback/details after translation to a declared throwable class.

## COLLECTION / TEXT

Core environment provides object protocols/facilities for list, map, set, tuple, range, and entry; import them explicitly from `/core/collections` unless prelude changes normatively.

- List construction uses ordinary invocation; maps use named construction arguments; sets/tuples likewise object facilities.
- Tuple type application is `tuple of Item`; tuples are homogeneous and fixed-length after construction, but runtime length is not part of the type.
- Indexing: `value[index]`; slices/ranges are objects.
- `for x in y` invokes the iteration protocol; comma-separated targets destructure a matching tuple/object item, including `for key, value in mapping`.
- `string` is Unicode text/UTF-8; default length is grapheme count and requires capability.
- Explicit scalar and byte views avoid ambiguity.
- `bytes` distinct from `string`; encode/decode explicit.

```yaml
lookup: default child THROWS (missing-key for map, index-error for sequence); checked returns V|none
lookup_rule: absence is always the checked spelling; no operation returns absence by default
map_removal: remove; key -> Value or throws missing-key; remove.checked; key -> Value|none without mutation on absence
map_removal_order: ordered map preserves remaining insertion order; reinserting a removed key appends it; an absent removal does not separate COW storage
unordered_iteration: deterministic for the same operation history; new items append, removal may fill the removed position with the prior final item, and content-equal values reached through different histories may iterate differently
list_sort: sort and sort.descending stably mutate and return the resulting list; supported items are int, fixed integers, float32/float64 (including float alias), and string only
list_sort_order: strings use Unicode scalar sequence; integers use mathematical value; floating NaNs remain a stable final bucket in both directions, signed zeros compare equal, and descending compares directly rather than reversing
byte_index: integer index -> uint8; negative/unrepresentable/out-of-bounds throws index-error
byte_slice: range index -> new bytes; visits authored half-open/inclusive range and step in order; any invalid selected index throws index-error; valid empty boundary -> empty bytes; never text-decodes
mutators: return the resulting collection for value/COW collections; none for in-place resource mutators unless a removed/replaced value is meaningful
discarded_mutation: statement-form collection mutators require a retained binding receiver; temporary entry/index snapshots are rejected, with explicit expression-result writeback such as nested.set; index, (nested[index].append; value)
release_replacement: displaced logical element released before mutation returns once no owner retains it; COW separation preserves unmutated collection without creating shared source identity
release_removal: removed element transferred to caller; released when returned value is released (immediately if discarded)
release_clearing: elements released in collection iteration order before clear returns
release_destruction: ordered collection releases remaining elements in iteration order
collection_identity: collection remains identity-less even when COW storage is shared or elements carry identity
stored_reference_identity: ref/shared-ref operands compare true iff same referent (including weak/strong pair); releasing stored shared ref destroys referent only after final owner
order: map and set preserve insertion order as an observable contract
unordered_variant: a separate unordered map/set type exists for layout cost; it is DETERMINISTIC (fixed hash seed), not merely unordered
unordered_rule: the performance option must never be the nondeterministic option; it is a distinct type, not a flag
source_iterable: structural; nonthrowing nonasync zero-arg nonmutating iterator -> iterator of Item OR object whose nonthrowing nonasync zero-arg next -> iteration-step of Item; no interface annotation required
iteration_step_construct: iteration-step; value -> item; iteration-step.end; -> end
iteration_end: dedicated end, never none (none may be an item); source iterator retains exhaustion and post-end next does not consult source
iterator_transfer: iterator objects are stateful linear values; transferring a named iterator into for makes its binding unavailable
source_iterator_lowering: for calls authored iterator then next; compiler does not synthesize a collection-specific loop
range: half-open by default; explicit 'through' constructor for inclusive ends
range_step: defaults to 1, must be non-zero; direction inconsistent with endpoints yields an empty range
inference: homogeneous literals infer the narrowest common declared type; heterogeneous require explicit union or annotation
cow: separation at first mutation visible through a non-unique value handle
hash_keys: mutable values and identity-bearing resources cannot be hash keys
```

String members follow the same callable-family shape:

```yaml
concat: 'a.concat; b, c' -> 'abc'; appends arguments to the receiver, NO separator
join: "': '.join; a, b, c" -> 'a: b: c'; the RECEIVER is the separator
join_bounds: zero args -> ''; one arg -> that arg with no separator; separator never precedes the first or follows the last part
composition_display: every argument converts through canonical text display; no display protocol is a typed error, never a silent rendering
composition_purity: neither member mutates the receiver; both return a new string
concat_vs_join: distinct operations sharing a subject, not modes of one; two members, not a family
trim: 'text.trim;' both ends | trim.start | trim.end
trim_argument: 'trim.start; "foo"' removes that literal when present, returns unchanged when absent; no separate strip-prefix member
position_children: start means logical index 0 and end the logical last scalar, for every string regardless of script
position_reason: writing direction is a display property; a string stores none, so left/right belong to a directional text type
contains: 'text.contains;' anywhere (default) | contains.start | contains.end; all boolean
contains_v1: exactly start and end; any/all await variadics or collections; 'at' awaits an index-unit decision
find: separate family; default -> text-range|none, find.all -> list of text-range, find.count -> int
empty_search: contains empty -> true; find empty -> first zero-width grapheme boundary; find.all empty -> every grapheme boundary including both ends; count = graphemes + 1
literal_search_boundary: non-empty find/contains operate on scalar sequences, not only grapheme boundaries; a match may end inside a grapheme
trim_modes: default Unicode whitespace; literal argument removes exactly one matching selected prefix/suffix
case_mapping: upper/lower default, .first, and upper.words are locale-independent Unicode operations; case-fold is explicit and locale-independent
normalise: explicit nfc/nfd/nfkc/nfkd children
unicode_profile: version one pins every compiler-owned Unicode table to Unicode 16.0.0; case folding, normalization, word/grapheme segmentation, and affected support crates update together; generated manifest records version; pinned/system Rust choice does not alter it
split_replace: literal, left-to-right, non-overlapping; empty split -> grapheme list without synthetic empties; empty replace -> insert at every grapheme boundary including ends
family_rule: a family is modes of ONE operation, not a bucket of related operations; group by subject uses a namespace instead
case_search: no case-insensitive child; apply explicit case-fold to both operands
regex: never a child of contains; regex stays match/matches; no member dispatches on whether its argument is a pattern object
string_views: length defaults to graphemes; bytes/scalars/graphemes explicit; text-range retains immutable source with checked byte/scalar/grapheme views
bytes: immutable octets, distinct from string; b'...' literals; only \\, \', \n, \r, \t, \0, \xHH escapes; iteration -> uint8
encoding: explicit utf8/utf16-le/utf16-be/utf32-le/utf32-be; encode total; decode validates all input and throws decode-error, never replacement text
```

## OBJECT_MODEL

- Objects expose unnamed structural protocols rather than compiler-special-cased runtime species.
- `name` resolves through one lookup view; `value.name` selects an instance member; `class::name`
  selects a static/class member; calls are explicit with `;`. The `class` designator is a simple
  visible name (including an imported local name), or late-bound `self` inside a class method.
- `instance class; arguments` is the only class-construction form. `instance` means semantic
  instantiation, not allocation; bare class invocation never constructs.
- `this` is the current instance and exists only in instance methods. `self` is the effective,
  late-bound class receiver, exists in instance and static methods, and may be used by
  `instance self;`.
- Function/class/namespace/type objects are reflectable semantic objects.
- `construct` is the conventional constructor method invoked by `instance`.
- Protocol: unnamed structural operation shape; not a declaration, nominal type, or dispatch object.
- Interface: named nominal contract/type and dispatch boundary adopted with `implements`.
- Trait: source field/method implementation composed with `uses`; not a type, subtyping relation, or
  Rust-style foreign contract.
- Compiler descriptor operations are internal lookup/reflection machinery, not another public
  source construct.
- Class: single inheritance initially; subclass-to-base assignment preserves dynamic value (no slicing).
- Overloading by implicit same-name signature dispatch is not initial behavior.
- Mutation visible by default; immutable behavior explicit via `constant`/contracts.
- Mutable static fields share storage per effective class. The Rust backend uses the same
  compiler-owned, per-operation `LazyLock<Mutex<...>>` strategy as mutable globals: reads copy the
  Terrane value, poisoning is an internal runtime failure, and this implementation detail neither
  makes source operation sequences atomic nor replaces explicit concurrency objects.
- A typed class or trait field may omit its initializer only when its declared type has a canonical
  default: `bool` -> `false`; numeric -> typed zero; `string` -> `''`; `bytes` -> empty bytes;
  `T|none` -> `none`; `list`/`map`/`set`/`unordered-map`/`unordered-set` -> empty collection.
  Plain `none`, tuples, source objects, references, callables, resources, and other runtime
  contracts are nondefaultable. This never infers the type, invokes an arbitrary constructor, or
  hides `none` in plain `T`; nondefaultable effective class fields require an explicit initializer.
  A trait may supply it or the using class may override the field. Effective inherited fields are
  validated before lowering, and explicit initializers use their declaring source/object context.
- Field metadata has one trailing clause:
  `field T = value metadata (external-name = 'wireName', secret = true)`.
  It is valid only on instance fields. `external-name` is a string, `secret` is a boolean, names
  cannot repeat, and effective external names are unique per class. Canonical or explicit defaults,
  including trait/base contributions, plus `T|none` optionality derive the resolved field
  descriptor's `defaulted` and `optional` flags rather than duplicating either policy.
- Class descriptor reflection exposes parallel declaration-ordered `field-names`,
  `field-external-names`, `field-defaulted`, `field-optional`, and `field-secret` lists plus
  `field-count`. Document mapping and redaction consume this same metadata; no facility-specific
  annotation or generated-Rust field name participates.

## GLOBAL

- Program globals form explicit initialization graph; cycles diagnosed.
- Mutable globals used across threads must satisfy shared-thread-safe protocol.
- Prefer standard thread-local object over second global grammar.

## ASYNC

- `async function` has a distinct async callable type; `await` is valid only in async context, and sync/async callable types require an explicit adapter.
- Async invocation returns a linear `task of T`; `await` consumes it exactly once. Scope `spawn` accepts an async callable or an unpolled task transferred automatically by its non-copyable contract and returns a linear `scoped-task of T`; `join` consumes that scoped task by value and creates `task of task-outcome of T`, so `await scope.join; child` consumes the child at the call boundary and the join task at `await`. Neither transfer requires source-level `move`; unconsumed tasks are compile-time errors, never implicit detach/cancel.
- Every async callable/task carries inferred `local` or `transferable` execution metadata from parameters, captures, live-across-suspension values, and invoked async boundaries. Projected Rust futures are local unless their admitted contract or an exact probe proves transfer. Callable compatibility preserves the distinction; a threaded spawn rejects local work.
- Direct calls, local bindings, and immediate await keep concrete Rust future types. Pin/box erasure appears only at heterogeneous storage or callable ABI boundaries, and only transferable erased futures receive the strategy's transfer bound. A task may move before first poll; after advancement it is pinned in executor-owned state.
- `select` is an async-only statement with at least two static `case` clauses. Each case header is
  either `await expression` or one ordinary local binding initialized by exactly one top-level
  `await`; the binding is scoped to that case body. Case result types need not agree.
- A selection consumes every task named by its headers, constructs each operation exactly once in
  source order, and polls wakefully from a per-statement cursor local to the current callable
  activation. Every invocation (including an async closure, recursion, or concurrent activation)
  owns a distinct cursor initialized to the first case. Simultaneously ready cases therefore use
  deterministic rotating priority without cross-activation interference.
- The cursor advances to one past the winner for either a ready value or terminal error, before
  cleanup starts. It does not advance on an all-pending poll, construction failure, or
  cancellation/deadline observation before a winner is fixed.
- After a winner is fixed, every loser receives cancellation before any loser is drained. Draining
  and release occur in reverse source order, projected async cleanup is included, and the selected
  body cannot begin early. Cleanup errors do not stop later cleanup; the later cleanup failure
  replaces the earlier result or error. A retained winning value is released if cleanup replaces it.
- External task cancellation or a deadline observed before winner selection takes the same cleanup
  path without advancing the cursor. After a winner is fixed, loser cleanup is shielded from later
  cancellation until the complete drain transaction finishes.
- Selection storage is fixed-size and stack-local: one pinned future and typed result slot per case
  plus the cursor, with no helper task, universal payload box, busy polling, or private event loop.
  Source-only cases retain the dependency-free cooperative runtime; requirements already needing
  native cancellation or projected async support select that runtime through the ordinary
  execution-requirement pipeline.
- Case guards, default cases, dynamic case lists, and expression-position `select` are unsupported.
  A linear task may appear in only one case. Exclusive move state is merged after all possible
  winning branches, and incompatible mutable/shared receiver borrows across simultaneously live
  cases are rejected before lowering. Case-operation borrows end with the `select` statement after
  loser cleanup, so a receiver may be read or mutated by the following statement.
- Source executor profiles map to compiler-owned execution strategies. Semantic lowering records generic requirements—runtime context, wake support, local/transferable work, blocking delegation—and chooses a runtime later; language contracts never name a runtime crate.
- The native backend enters exactly one selected wake-driven runtime around async `main`; projected futures are constructed on first poll inside it. Pending dependency futures and timers sleep until their waker fires. No failed runtime requirement silently falls back to blocking; cancellable legacy scope polling parks on its waker while scope scheduling remains a separate contract.
- Projected async metadata records runtime-context, wake-support, and transfer knowledge independently as `required`, `not required`, or `unknown`; Rust `async fn` currently requires wake support while context and transfer remain unknown absent exact evidence. Completion and hover show these Terrane requirements, not Rust future internals.
- A non-async Rust callable whose concrete result implements canonical `core::future::into_future::IntoFuture` returns a Terrane task. Concrete owner arguments substitute into the associated output; `Result<T,E>` retains ordinary projected success/failure mapping. Constructing the task is inert. Its first poll evaluates the Rust call once, invokes `IntoFuture::into_future` once, and polls through the ordinary async panic/cancellation/executor boundary. Same-named traits and unresolved, borrowed, lifetime-escaping, or otherwise unrepresentable outputs decline.
- `task-outcome of T`: `completed bool`, `cancelled bool`, `value T|none` present exactly on completion, `error throwable|none` present exactly on failure.
- Cancellation is cooperative at defined points, but observation while suspended promptly drops the in-flight operation. A wakeable compiler-owned signal—not timer polling—drives native observation. Lowered async `try` regions install nested finalization guards: the innermost active guard drops its operation, runs compiler-separated `finally` state exactly once, and then exposes the request to its parent. Async cleanup remains protected until the outermost guard releases cancellation to the task boundary. Guard state is non-catchable and uses no Rust panic/unwind control flow. Owned foreign operation state runs Rust `Drop`; cleanup-owned state survives until cleanup. Reject overlapping exclusive ownership, insufficient-lifetime borrows, or other captures that cannot split operation and cleanup state.
- Failure requests sibling cancellation; the scope still joins cleanup and retains outcomes. Completed work is never erased, so completed+cancelled may both be true. A cleanup error replaces pending cancellation under ordinary `finally` rules while cancellation remains observable.
- Deadlines use the same cancellation transition and are not hard kills. Cleanup may suspend beyond a deadline; join remains pending, and no liveness is promised for a foreign cleanup future that never wakes.
- `/core/time` requires `clocks`. `duration` is an exact non-negative value with canonical
  `seconds`, `nanoseconds`, and arbitrary-precision `total-nanoseconds`; factories reject known
  negative constants statically and dynamic negatives with `invalid-duration`. Addition,
  multiplication by a non-negative integer, and `subtract.checked` preserve exactness.
- `clock::wall` returns a UTC Unix `instant`; `clock::monotonic` returns a comparable point in the
  current runtime activation's monotonic domain. Monotonic points and deadlines cannot cross a
  persistence, foreign, or runtime boundary. Wall-clock adjustment never changes elapsed-time,
  deadline, sleep, or ticker behavior.
- `clock::sleep` captures its absolute monotonic target when the task is constructed, before first
  poll. `clock::sleep-until` uses an existing monotonic target. Both suspend through wake-driven
  host timers, re-check the exact target after every wake, chunk only at the host boundary when a
  timer range is bounded, and drop timer registration promptly on cancellation or `select` loss.
- `clock::deadline` creates a typed absolute deadline from a duration and `deadline::at` wraps a
  same-runtime monotonic point. `deadline.remaining` returns `duration|none`; `deadline.expired`
  is true once the target is eligible. `task-scope` and child scopes accept only typed deadlines;
  children clamp to the earlier parent/requested target. Networking and TLS operation options
  likewise carry typed deadlines; integer millisecond timeout aliases do not exist.
- `clock::interval` rejects a zero period and returns a linear ticker anchored once at construction.
  Each `ticker.next` schedules `anchor + period * index`, so handler latency does not accumulate
  drift. A `tick` reports scheduled/observed monotonic points and an exact coalesced count; a
  cancelled `next` does not advance the index and no ticker helper task is detached.
- `/core/process-signals` requires `process-signals`. It exposes typed `process-signal` values for
  interrupt, terminate, hangup, and quit; `process-signals; selected-set` creates a linear
  subscription whose async `next` participates normally in `await` and `select`. Each event carries
  signal, exact count, deterministic broker sequence, host-observation monotonic point, and explicit
  overflow status. One process-wide broker fans every observation out to all interested live
  subscriptions and coalesces repeated pending observations without silent loss.
- Process-signal ownership is explicit: `close`/`destruct` unregisters the subscription; closing the
  last subscription stops and joins the broker worker, removes wake registrations, and restores the
  exact prior host dispositions. Supported Unix hosts with lock-free 64-bit atomics use the audited
  signal ABI boundary; selecting this package on an unsupported host family is a build error rather
  than a runtime fallback.
- Compiler-owned async stream, TCP, UDP, and DNS operations use the selected runtime. Blocking standard-handle/socket ABIs are explicitly delegated to its blocking pool, never run on executor workers or through a second runtime.
- No borrow crosses suspension unless its owner lifetime and executor transfer requirements are proven.
- Runtime remains profile-selected; concurrency objects synchronize existing executor/runtime host threads and expose no thread lifecycle.
- `/core/concurrency` requires `threads`. `channel; Item, compile-time-capacity, policy` creates compiler-owned typed linear sender/receiver endpoints without boxed value erasure; capacity 0 is supported only with channel-block and is a rendezvous whose send completes after receive accepts the exact value; accepted rendezvous delivery is completed work and wins if cancellation is simultaneously ready when the sender resumes, while the cancellation request remains independently observable; other policies require positive capacity; policies: channel-block suspends, channel-fail-send => unaccepted/open/undropped + rejected-value, channel-drop-newest => submitted dropped-value, channel-drop-oldest => accepted + evicted dropped-value outcome; send local task => accepted/closed/dropped plus rejected/dropped item outcome; receive local task => available/closed/optional value; sender close drains buffered values before closed; consuming receiver close returns all already accepted buffered values and wakes blocked sends closed; cancellation/deadline unregister pending waiter; duplicate/use-after-close rejected
- [shared-state-owner-task] v1 deliberately has no generic shared mutable cell: one task owns and mutates application state, peers exchange typed commands/results through bounded channels; compiler-owned diagnostic names `mutex`, `read-write-lock`, `shared-cell` reject construction with this guidance; avoids guard lifetimes, suspendable critical sections, and a second shared-ownership model
- `int-mutex` and `int-read-write-lock`: low-level integer-specialized synchronized load/store/update cells with shared opaque identity; no guard-scoped arbitrary critical section and no promised generic element form
- `atomic-int64`: typed `memory-order`; load allows relaxed/acquire/seq-cst, store relaxed/release/seq-cst, increase all five; host validates mutable authored order objects defensively.
- `thread-local-int`: one value per existing host thread and shared object identity; dropping last owner makes entries stale and later accesses sweep them.
- Unavailable target capability rejects async or concurrency facilities statically.

## DOCUMENTS

```yaml
opt_in: class implements /core/documents::document-decodable; no implicit class decoding
entrypoints: /core/documents/json::decode-typed-json | /core/documents/yaml::decode-typed-yaml
arguments: source string, concrete class descriptor, matching parser options, explicit allow-unknown bool
result: document-decode-outcome of T; concrete initialized T value + typed list of every diagnostic; failed iff diagnostics nonempty
fields: scalar | nested opted class | list | homogeneous tuple | map of string,V; all nested values decodable
numeric: integers exact/range-checked; decimal/integer to float rounds to nearest finite destination value, ties-to-even; out-of-finite-range diagnoses
absence: canonical type default or explicit initializer is retained; T|none may be absent; every other absent field diagnoses
names: one OBJ field metadata record supplies semantic name, external-name, defaulted, optional, secret
unknowns: rejected or recursively ignored only by explicit call policy
diagnostic: deterministic path, expected, actual-kind, reason, message, decode-call source, field source
validation: optional document-validatable.validate-document -> string|none after error-free field decoding
ineligible: resource ownership | inheritance | custom construction | recursive value cycle | unsupported field -> implement deserializable manually
boundary: generated Rust materializes statically known T; parser/metadata/default/validation/diagnostic policy remains Terrane; no universal boxed value
```

## LOGGING

```yaml
package: /core/logging; profile capability logging; /core/logging/async also requires threads
levels: trace | debug | info | warning | error | critical
logger: explicit sink + minimum severity + hierarchical target prefix + target + immutable fields/spans + field/byte bounds
constructors: default-logger(sink) | named-logger(sink,target) | make-logger(sink,options); no ambient application logger
event: controlled timestamp + per-sink sequence + severity + target + message + ordered fields + emission source + spans + origin
field: key + log-value protocol renderer + field-call source + secret bit; field/secret-field call sites compiler-injected
filter: severity/target policy runs in Terrane before renderer and sink; debug/info/warning/error operations preserve caller source
value: log-value.render -> bounded document-value; scalar/document/error-chain/user wrappers; no universal debug formatter
redaction: without explicit sink reveal permission, secret renderer is not called and sink receives only structured '<redacted>'
sinks: explicit memory | console | failing; bounded capacity + named overflow + observable discarded-count; deterministic memory clock/drain; nonrecursive fallback
async: send-event + consume-events reuse typed channel endpoints and their overflow/backpressure; no logging-specific queue
dependency_bridge: explicit install into sink; preserve foreign event/log-kv/span fields (unsupported values become `<field>.debug`) plus target/module/file/line; normalize absolute Cargo files to stable crate-relative paths; no Terrane source claim; bridge layer separately composable, absent optional remote layer does no work
rust_boundary: sink ID/synchronization/storage/clock/console I/O + foreign callback bridge only; policy/enrichment/filter/redaction/event API in Terrane
```

## STREAMS

```yaml
package: /core/streams; ordinary bundled Terrane source included recursively only when imported
resource_objects: byte-reader | byte-writer | text-reader | text-writer; inferred from stored process handle; no copy, use after transfer/consume, or double release
process_factories: stdin -> byte-reader; stdout/stderr -> byte-writer
text_adapter: 'byte-endpoint.text; encoding' transfers endpoint; adapter carries explicit encoding
read_result: data bytes, completed byte count, end bool, failed bool, message string
text_read_result: text string plus the same byte-count/status fields; malformed input throws decode-error
write_result: incomplete result retains encoded data bytes for resume; completed result releases data; completed byte count, failed bool, message string
partial_policy: read/write/resume may complete partially; resume performs one host write and may be repeated; read-exact fails on short EOF, bounded read-all accepts EOF, write-all loops until complete/failure/no progress
bounded_read_all: explicit limit REQUIRED
newline: no implicit translation; text-writer.line explicitly appends '\n' and its completed count includes the encoded newline
close: explicit, consuming, idempotent host release with observable failure; destruction explicitly discards release failure
writer_durability: byte/text writer flush, sync-data, and sync-all are distinct; unsupported/failure never silently weakens
async: read-async/write-async preserve synchronous result contracts; cancellation is task-outcome state and never fabricates stream progress
rust_boundary: process-I/O syscall/ABI handle registry and one partial operation only
terrane_layers: public protocols/results, loops, adapters, policy, factories, async wrappers
generated_ownership: transfers/use-after-consume/double-release checked statically; current handle refcount only coordinates one host release after adapter transfer
```

## CRYPTOGRAPHIC_ALGORITHM_IDENTITY

```yaml
hash_descriptor: selects SHA-256 or SHA-512 for both unkeyed digest and corresponding HMAC
hmac_mapping: SHA-256 -> HMAC-SHA-256; SHA-512 -> HMAC-SHA-512
independence: HMAC consumes descriptor/key/message, never a prior digest; no cross-algorithm pairing is inferred
value_types: digest and MAC are distinct and retain algorithm identity; constant-time equality requires matching value kind and algorithm
unsupported: structured operation failure; never fallback substitution
```

## PATHS_FILESYSTEM_PROCESS

```yaml
packages: /core/filesystem/paths | /core/filesystem | /core/process; ordinary import-driven Terrane source
path: platform-neutral lexical value with canonical '/' separator; no host lookup or existence implication
path_normalise: discard empty/'.'; resolve '..' lexically; rooted paths never cross root; unrooted unresolved leading parents remain
path_join: absolute child replaces base; otherwise concatenate then normalise
canonicalisation: `filesystem-canonical` is the capability-mediated native host resolution operation; follows filesystem and may fail; `filesystem-realpath` is its deliberate POSIX spelling alias with the same implementation; lexical path operations remain Terrane
filesystem_authority: unforgeable value acquired only through `filesystem-capability`; REQUIRED by every host filesystem operation and handle method
metadata: metadata follows final link; symlink-metadata inspects it; portable kind/size/read-only plus platform permission detail
bounded_read: explicit limit; excess fails, never truncates
atomic_replace: sibling temporary then rename over destination without following destination link
directory_relative: resource-owning directory handle; final anchor component and all beneath operations are no-follow; intermediate components of the caller-supplied anchor path use ordinary host resolution; beneath rejects escape; cross-filesystem requires explicit permission
handles: linear resource transfer and idempotent host release shared with streams; partial file write exposes completed offset for resume
platform_string: exactly one host component; is-text selects lossless Unicode text or lossless raw bytes; NEVER replacement decoding
snapshots: arguments and environment are explicit; environment returns paired native-string names/values
host_name: /core/process process-host-name returns process-host-name-result with failed/available/message plus a lossless native-string value; requires process capability; Rust std has no portable host-name query, so audited hostname crate owns host ABI retrieval and non-Unicode conversion; boundary returns owned data and exposes no host handle
cli_schema: exact flag:/value: long-option spellings; parser returns flags/options/positionals plus diagnostic argument indices/messages; NEVER exits
cli_v1_limits: no --option=value, -- separator, or short clustering; undeclared short spellings remain positional
exit_status: exact int 0..=255 valid; invalid construction yields valid=false and sentinel 255 without terminating; exit alone terminates
rust_boundary: filesystem/descriptor syscalls, lossless OS argument/environment access, process exit
terrane_layers: paths, filesystem objects/policy/results, native-string model, CLI parser, exit validation
```

## TARGET

- Build selects target profile/capabilities.
- Missing required capability => source diagnostic naming construct and requirement; never silently change semantics.
- Dynamic/static lowering choices may differ only with identical source behavior.
- `no_std` uses minimal support + target capabilities.
- Minimal support includes adaptive exact `int` and its normative failures when that feature lands; constrained targets prove supported bounds or reject by capability rather than changing semantics.
- Hosted convenience must not preclude allocator-free/embedded/kernel realization where capabilities permit.
- Low-level representation/ABI/pointer/volatile/atomic operations require explicit contracts and concrete unsafe adapters or `unsafe rust`.

## PACKAGE

```yaml
origins: terrane packages | Rust crates | system/C libraries
use: declares dependency
from_import: binds exported objects into the containing scope via namespace/importer
lockfile: reproducible exact graph
cargo: compiler owns generated Cargo manifest/source tree
build_scripts: declarative metadata preferred; arbitrary scripts capability-gated and reported
```
```toml
package = "example.tools" # required non-empty identity
prelude = true            # optional; defaults true
artifact = "executable"   # executable | dynamic-library | library

[namespaces]               # required non-empty mapping table
"example/tools" = "src"
"example/generated" = "generated"

[terrane-dependencies.codec]
path = "../codec"          # local development; hash is optional

[terrane-dependencies.remote]
git = "https://example.invalid/remote.git"
tag = "v1.2.3"             # Git dependencies accept tags only
hash = "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
```
- Optional `[profile]`: `name` defaults to `default`; `capabilities` is an effect allowlist drawn from `build | entropy | filesystem | networking | process | threads | tls`; `panic` is `unwind` (default) or `abort`. Rust dependency effects and gated bundled core imports must be allowed. Missing capability is source diagnostic S2032 naming profile, capability, imported namespace, and importer. Gates: `/core/streams` + `/core/process` -> `process`; `/core/filesystem` -> `filesystem`; `/core/random` + `/core/random/uuid` -> `entropy`; `/core/networking` -> `networking`; `/core/networking/tls` -> `networking` + `tls`; `/core/concurrency` -> `threads`.
- Optional `[testing]`: `unit`, `integration`, and `end-to-end` override the conventional relative roots. `[testing.profile]` selects a named test profile with the same `capabilities` and `panic` fields as `[profile]`; omitted fields inherit the ordinary profile, so test compilation cannot acquire an undeclared capability implicitly.
- Authored manifest filename: `package.toml`; syntax is TOML; unknown fields rejected. Optional `artifact` is `executable` (default), `dynamic-library`, or `library`. A dynamic library emits a Cargo `cdylib`; a Terrane library is source-composed with its consuming application and may be checked or rendered as Rust directly but is never built, run, debugged, or profiled as a standalone artifact. Terrane declarations do not yet export host-visible symbols automatically; a maintained authored Rust module supplies a dynamic-library host entrypoint and may call lowered Terrane functions within the generated crate.
- A library has no `main`; its namespace roots must equal or descend from its package identity. An application still has exactly one `main`, independent of its libraries. Each library source unit retains its library manifest's `prelude` setting when composed; the application does not silently enable or disable its prelude. The consuming application owns reflection, executor, capability/panic profile, build toolchain, and testing policy for the final artifact; corresponding library manifest values do not replace those application settings. Library sources otherwise join the application's semantic and lowering pipeline, so ordinary imports and source-intelligence operations cross the package boundary without an ABI or generated wrapper.
- `[terrane-dependencies.<name>]` declares exactly one source: a relative local `path`, or `git` plus a required immutable `tag`. Git sources also require `hash`; local sources accept an optional hash for reproducibility. Hashes use `sha256:<lowercase-hex>` over every sorted normalized relative file path and byte in the tree, excluding only `.git` and `.trn` directories and rejecting symbolic links. Consequently local README files, ignored build output, and editor files affect the hash; hash a clean source tree when matching a Git tag. Git sources are verified in a temporary checkout before publication into the consuming application's content-addressed `.trn/packages/<digest>` cache.
- `terrane package hash <directory>` prints the exact manifest hash for a local source tree; `terrane package hash --git <url> --tag <tag>` clones and verifies the exact tag before printing the same tree hash. `terrane package install` accepts either source form, requires `package.toml` in the current directory, validates that the target is a Terrane library, and adds a `[terrane-dependencies.<name>]` entry without replacing existing dependency declarations. Local installs remain unpinned for development and write only `path`; Git installs record the required full hash. `--name` overrides the dependency table key; otherwise the library package identity is used.
- Dependent packages must declare `artifact = "library"`. Their own Terrane dependencies, `[rust-dependencies]`, and `[rust-modules]` compose transitively into the application. One package graph cannot contain distinct source roots with the same library identity. Conflicting Rust dependency aliases or authored Rust module names are rejected, and every dependency effect must remain allowed by the consuming application's capability profile.
- `namespaces`: canonical namespace-root keys mapped to distinct, relative directory roots; no absolute/parent paths. Source discovery recursively includes `.trn` files only, resolves overlapping mappings by longest namespace prefix, and assigns stable file IDs in sorted package-relative path order.
- Every discovered declaration must equal the namespace derived from its mapping and relative parent directory. Duplicate mapped directories and mapped roots containing no `.trn` files are manifest-load errors.
- A direct source CLI input (an existing non-manifest file or a path ending in `.trn`) is implicit package `single-file`, one unit, default prelude, and exempt from directory correspondence. If byte zero begins `#!`, the shebang is comment trivia and the script may omit an authored namespace; the compiler supplies an implementation-owned identity outside the authored namespace grammar, making source collision impossible. Manifest-discovered application sources never receive this exemption. `terrane <file-or-manifest>` is implicit `terrane run <file-or-manifest>`, forwards remaining arguments (with an optional leading `--`), and uses the same compilation/cache/execution pipeline.
- Compiler-bundled support source is copied content-addressably into generated builds and referenced only by generated-project-relative Cargo paths; no registry, network, or installation absolute path enters reproducible output. Apply the same vendoring mechanism to admitted authored third-party dependencies.

- Package import does not imply runtime mutation.
- Dependency graph/order deterministic.
- Separate compilation honors published representation/ABI; downstream cannot silently respecialize upstream public layout.

## TESTING

```yaml
command: terrane test [--list] [--filter TEXT|--exact IDENTITY|--glob PATTERN|--regex PATTERN] [--tier unit|integration|end-to-end] [--jobs N] [--timeout Nms|Ns] [--argument VALUE] [--fail-fast] [--show-output] [--report FILE] PACKAGE
implementation: public framework/case execution/reporting in bundled Terrane /core/testing; process fixtures in capability-gated /core/testing/process; compiler owns discovery, typed registry generation, shared lowering, and narrow host isolation/capture
discovery: conventional tests/unit | tests/integration | tests/end-to-end roots; bounded manifest overrides; top-level zero-parameter test-* functions returning none; sync/async/throwing
order: explicit unit -> integration -> end-to-end tier order, then logical path, source order, and function name; substring/exact/glob/regex filters are mutually exclusive; semantic analysis covers production sources and every populated test root, while only tiers containing selected cases are lowered and backend-validated
unit: production source set plus unit source set; ordinary namespace-private access only in the declaring namespace
integration: independently compiled external consumer view; public package surface only
end_to_end: independently compiled tests drive the actual production artifact exposed as string|none; /core/testing/process process-fixture carries lossless encoded args/environment, byte-exact input/output, exit/crash/deadline state, bounded incremental stdout/stderr capture, and independent truncation flags; it requires process capability
test_profile: [testing.profile] explicitly selects name/capabilities/panic; omitted fields inherit the ordinary package profile; no capability appears unless it is declared by one of those profiles
entrypoint: one compiler-owned generated runner main per selected populated tier; --list and an empty selection perform semantic discovery without runner lowering or native compilation; ordinary programs/scripts still require authored parameterless main
isolation: one fresh working directory and process per selected case by default; crash/exit/timeout cannot suppress later cases; completed run directories are removed; parent-side process and pipe closure waits are bounded
assertions: assert | deny | fail | fail-values for caller-owned typed comparisons with test-value rendering | concrete scalar/bytes equal/not-equal | concrete scalar/bytes optional present/none | float near with tolerance | throwable callback assert-throws with expected descriptor identity | skip
assertion_invariants: operands evaluated once; structured causes retain source frames and bounded useful values; concrete types stay static; no universal boxed test value; rendering honors explicit display/test-value and secrecy contracts
context: stable identity/tier/seed/temporary-directory, typed deadline duration, repeatable controlled process arguments, checked explicit monotonic-time advancement, and test-clock sleeps that advance immediately to their target when awaited; reading this runner-supplied context is intentionally ungated, while operations such as process spawning remain capability-gated
reporting: deterministic human counts distinguish pass/fail/skip/timeout/crash/infrastructure outcomes; schema 1.2.0 JSON has effective run metadata, numeric timeout milliseconds, per-tier compilation status/diagnostics, structured causes, and bounded byte-array streams with independent truncation flags
timeout_units: bare values are seconds; ms and s suffixes are accepted
non_goals_initial: new test declaration grammar, decorators, parameterized-test syntax, automatic retry, snapshot rewriting, mock generation, matcher DSL
host_boundary: no Cargo test target, Rust #[test], or libtest semantics; Rust only for compiler CLI and irreducible process/filesystem/clock ABI
```

The compiler's own conformance, compile-fail, lowering-golden, diagnostic, and host implementation
tests remain distinct. `/core/testing` is the first-party framework for code authored in Terrane.

## DEBUGGING

```yaml
commands: terrane debug [--embed-sources] [--embed-generated-sources] FILE-OR-MANIFEST [-- ARGUMENTS] | terrane debug-adapter --stdio; debug --release is rejected
backend: one selected lldb-dap/LLDB implementation owns process control, unwind, registers, memory, and machine breakpoints; Terrane owns source translation
build: named terrane-debug-v1 profile shared by Cargo emission and provenance; full debug information, optimization 0, no stripping, compiler-default inlining at that optimization; optional --embed-sources and --embed-generated-sources independently record authored and generated build-time snapshots
provenance: deterministic schema 1.3 sidecar beside generated Rust and executable
identity: compiler/toolchain/exact rustc release/sysroot/target/toolchain-bound ABI recipe/profile, manifest/projection-lock/source/final-Rust hashes, executable identity, final-file associations, sequence points, functions, lexical scopes, bindings, object fields/privacy, and explicit source/build relocation roots
validation: translation requires matching sidecar, executable, generated files, source bytes, compiler/schema, selected target/toolchain-bound ABI recipe, and every named debug-profile property; mismatch reports native fidelity without disabling raw native debugging
breakpoints: every final-Rust location for an authored sequence point; relative paths resolve from the debugger invocation directory before the package root; non-executable lines adjust only forward within the same exact lexical scope; requested and resolved Terrane/generated locations are reported; unresolved requests remain pending
frames: stable session-local mapped-frame indices, explicit frame selection, Terrane namespace/function identities, logical source positions, and bounded source context; generated source rendering uses an explicitly embedded snapshot when its exact build-tree file is unavailable; generated/runtime/native frames remain available through explicit escape hatches
stepping: next targets a different authored sequence point in the same frame or its caller, including loop re-entry; step-out requires an exact depth decrease; a current-location user breakpoint is suspended while leaving that point, all other user breakpoints remain active, bounded temporary targets are installed and removed in batches, and oversized sets fall back to bounded native stepping without truncation
values: lexical binding selection uses the selected stop, scope ancestry, visibility range, and innermost shadow; raw values remain authoritative without a matching ABI recipe; exact adaptive integers use the recorded x86-64 Linux recipe; bounded strings/bytes, recursive focused-value expansion, lazy structured children, and explicit optimized-out/unavailable/truncated states; a distinct moved-value state is not claimed because current LLDB data does not prove source ownership transitions
privacy: secret object fields are redacted and stripped of child, memory, and evaluation references before ordinary frontend exposure; explicit raw native inspection is a separate opt-in path
limits: 100 variables plus an explicit continuation marker, 4096 display bytes, focused depth/cycle/count bounds, context radius up to 20, at most 512 temporary source targets per step, and at most 64 native fallback steps
protocol: standard framed DAP only on stdout; exactly one initialized event; correlated backend responses, including delayed launch/attach results, are retained and validated; variable handles are stop-local; cancel is not advertised; disconnect terminates launched debuggees by default and detaches from attached processes by default; launched debuggee exit status becomes CLI exit status
rust_formatters: imported only from the exact recorded Rust sysroot before client init commands; formatter absence is non-fatal
fidelity: terrane/fidelity reports source/native mode, target, toolchain-bound ABI recipe, artifact profile, exact rustc release, inlining state, and a reason for degradation
supported_host: 30 Linux x86-64 LLDB 22 integration scenarios exercise CLI/DAP launch, pre- and post-launch breakpoint ordering, source stepping, fatal native stops, relocated exact-build source maps, split generated support, shadowed locals, debugger-like debuggee output isolation, temporary-breakpoint cleanup, delayed launch failures, relative invocation paths, and more than 500 sequence points; attach reports the selected host policy outcome
experimental: CLI/DAP/provenance schema in 0.1; other hosts, attach regimes, optimized/stripped source fidelity, and unsupported layouts report limitations
excluded: direct isolated test-runner debugging pending a separate process-ownership/context/timeout/temp-directory/reporting contract; conditional breakpoints, logpoints, restart, Terrane expression evaluation, arbitrary debuggee formatting calls, DWARF rewriting, alternate native backends, time travel
```

## PROFILING

```yaml
commands: terrane profile record --cpu [--memory-timeline] [--embed-sources] [--retain-arguments] [--output FILE] FILE-OR-MANIFEST [-- ARGUMENTS] | terrane profile record --allocations [--memory-timeline] [--embed-sources] [--retain-arguments] [--output FILE] FILE-OR-MANIFEST [-- ARGUMENTS] | terrane profile record --memory-timeline [--embed-sources] [--retain-arguments] [--output FILE] FILE-OR-MANIFEST [-- ARGUMENTS] | terrane profile show FILE.trnprof [--focus PATH:LINE] [--generated] [--native] [--format text|json] [--limit N] [--compare BASELINE.trnprof] [--max-allocated-bytes N] [--max-retained-bytes N] [--source-root PATH] [--build-root PATH]
backend: Linux x86-64 perf CPU sampling; Heaptrack allocation/free interposition with normalized capture-local lifetimes; Linux procfs bounded process-memory sampling. CPU or allocations plus memory-timeline retains one primary unit and emits a separate auxiliary byte timeline.
build: named terrane-profile-cpu-v1 profile; optimization 3, line tables, no stripping, compiler-default optimized inlining, ThinLTO, one code-generation unit, package panic policy
artifact: bounded schema 1.2 typed .trnprof evidence; exact compiler/Rust/target/profile/input/executable/module identity; normalized CPU frames; allocation identities, sizes, collector timestamps, trace identities, matched frees, traffic/live/retained/peak accounting, and explicit process-memory interval/missed-sample/RSS fields
attribution: CPU samples use exclusive exact-authored|shared-or-ambiguous|runtime-associated|generated-only|native-only|unavailable buckets; allocation stacks preserve Heaptrack native/generated frames and separately weighted allocation-count, retained-byte, and peak-live-byte views
presentation: CPU source table/call tree/folded stacks; allocation traffic, churn, retained-at-exit and peak-live totals with folded allocation sites; process-memory timeline tables; bounded text and JSON. Allocation comparisons use aggregate evidence independent of inlining, and byte thresholds return status 1 after rendering the report.
relocation: copied-but-identical source and build roots remain attributable; changed, missing, or ambiguous identities preserve native evidence with explicit reduced fidelity
privacy: generated Rust and compiler-bundled source are retained for exact attribution; other authored source requires --embed-sources; workload arguments require --retain-arguments
unit: CPU captures use sample count; allocation captures use bytes and independent event counts; process-memory timelines use bytes and do not claim allocation ownership or deterministic wall time
unsupported: garbage-collector reachability, heap dumps, memory-access attribution, off-CPU/hardware counters, continuous-service capture, and non-Linux collectors
```

## CORE LIBRARY PRINCIPLE

```yaml
rule: public core facilities are written in TERRANE over a deliberately minimal Rust substrate
namespace_layers:
  /core: one public normative compiler-supplied surface; /core/types descriptors are implicit constructs, operational namespaces require explicit object or namespace-wide import
implementation_boundary: implementation language creates no public namespace layer
host_binding_rule: compiler seeds each bundled core package with only its own private host-* bindings; other packages cannot import them
protected_bridge_rule: protected declarations permit deliberate parent-to-child core package composition and remain absent from authored and namespace-wide imports
identity_rule: public wrappers retain /core/<facility>::<name> semantic identities; private host bindings carry separate compiler-only lowering keys
rust_visibility: generated Rust shims and ABI types may be public for split-module calls without publishing Terrane objects
no_internal_root: /internal does not exist as a language-owned namespace; /core/platform-* aliases do not exist
why_decisive: a Rust support crate is permanently opaque to the compiler - implementing a facility in Rust forecloses inlining, specialisation, and whole-program analysis for it forever
why_also: exercises lowering against real code; builds a corpus before a public one exists; failures surface as readable Terrane frames, which a Rust crate can never give
boundary: PER LAYER, not per facility - Rust owns the irreducible or audited layer, Terrane owns object model, policy, diagnostics, integration
example_json: Rust byte scanner beneath Terrane document model, descriptor mapping, data-path diagnostics, canonical output
example_tls: audited protocol implementation beneath Terrane stream integration, trust store, ALPN, connector policy, capability gating; NEVER reimplement TLS
rust_justified_only_if:
  - syscall/ABI boundary (fds, sockets, clocks, process control)
  - a guarantee the optimiser would destroy (constant-time compare, memory ordering, zeroisation) - not a perf judgement
  - large externally-audited security-critical implementation
  - data rather than code (Unicode tables, tz database), generated
rust_layer_rule: a layer claiming to be Rust states WHICH of the four applies
dependency_path: core facilities use the ordinary §23 manifest declaration and generated crossed-member projection; no privileged path
profiles: core facilities declare Rust dependencies explicitly so a profile may exclude them
consequence_build: package-level artifact caching becomes load-bearing, not an optimisation
consequence_profile: capabilities become which bundled core packages are present, not which support crates were compiled in
```

## DEPENDENCY PRINCIPLE

```yaml
rule: declarations name ECOSYSTEMS and PACKAGES, never APIs
truth: resolved manifest/lock/features/default-features/target/toolchain define the interface; nothing in the language predefines it
rust_declaration: package.toml [rust-dependencies] only; no source dependency declaration
rust_import: /deps/<manifest-name>/<rust-modules>; undeclared root is S2027
bridging: generated Rust shims for closed projected members; call-site-closed projected generics use one direct Rust call with the same conversion/panic/error boundary; no marshalling/runtime adapter
projection: one lock-resolved rustdoc artifact shared by compiler and LSP; verbatim names; module namespaces; functions, instance methods, associated functions as Class::function static members, concrete trait operations, opaque types, and enums; associated functions are never independent namespace functions; an operation supplied by a concrete Rust trait implementation is a namespace function beneath its concrete projected owner, with its receiver as the first parameter; a unique owner-local method keeps its name; owner-local collisions move every candidate beneath a deterministic `trait/<canonical-trait-path>` namespace, including normalized concrete trait arguments when one owner has several implementations of the same generic trait; only a collision that remains after owner and trait qualification declines; exact qualified Rust paths and reasons remain in projection artifacts
projection_schema: full rustdoc document deserialized through version-matched rustdoc-types; toolchain pin + FORMAT_VERSION + schema crate + cache schema 52 are one unit; malformed/mismatched input fails explicitly, representability limits are recorded declines
projected_generic_identity: full concrete Rust spelling including generic args is canonical identity; distinct instantiations stay distinct; aliases substitute before projection; an all-default declaration projects its default identity and resolves method Self; one admitted concrete instantiation keeps the readable Rust name, while multiple admitted instantiations append the full SHA-256 of each canonical path (never truncated/order-dependent); any duplicate (namespace,name) for distinct concrete identities fails projection explicitly; generated modules emit instantiated paths as Rust type aliases; lifetime/open generics remain explicit declines unless selected by the destination-directed rule or closed consistently from a projected call site
projection_artifacts: source order exact cache -> optional TERRANE_PROJECTION_ARTIFACT_URL trusted HTTPS -> local rustdoc; bundled source explicitly skipped pending a release artifact channel; published envelope requires exact dependency/features/defaults/target/build pin/rustdoc pin+format/projection-schema plus matching SHA-256 payload hash; every current attempt has an event/outcome; history format 3 persists top-level members, projected instance/static members, injected exact bound-owner dependencies, stable content-origin provenance/format/schema/cache identity/content hash without cache-hit churn; verified published result enters ordinary offline cache
projection_oracle: deterministic contained minimal crate; production destination-result questions are batched after written destination types are known; yes requires a compiler artifact for the exact concrete type/bound pair, only probe-local rustc bound failure is no, and all other failures are unknown; exact reports cache under projection identity and their evidence/wall time enter the shared projection artifact; transitive crates.io packages that own public bound traits are pinned from the resolved lock, recorded in projection history, and added as featureless default-disabled direct generated-crate dependencies so the rustdoc spelling is nameable without widening the resolved feature set
projection_destination_results: one Rust generic parameter used only by a projected result may be selected from exactly one explicit binding, argument, class-field, or declared-return destination; template unification must agree at every occurrence; supported closed owned shapes are scalars, string, bytes, optional, sequence, mapping, set, homogeneous tuple, and projected foreign object; missing, incompatible, conflicting, borrowed, source-object, no, unknown, and missing-oracle-answer cases are compile-time diagnostics; lowering emits an explicit concrete turbofish and applies recursive owned result conversion; Terrane source never writes Rust generic arguments
projection_call_site_generics: schema 46 retains generic templates and Rust bounds; every input-selected occurrence is unified across ordinary inputs, nested owned collections, and callback parameter/result shapes; each generic must resolve to one concrete projected type, fully concrete bounds are batch-checked by the projection oracle, and lowering specializes parameter/result conversions before one direct dependency call; Rust infers callback/future implementation parameters and bound-only parameters from the direct dependency call; static, consuming instance, and concrete external-trait calls use the same selection contract; Rust `AsRef` adaptation is restricted to canonical `str` and `std::path::Path`, and asynchronous alias unwrapping to canonical `futures_core::future::BoxFuture`
projection_native_input_obligations: a representable value-style Rust `impl Trait` input remains a caller-selected projected generic rather than forcing one concrete implementation or a user-visible Terrane interface; projection retains every canonical Rust bound, call analysis infers the exact projected Rust representation from the Terrane argument and substitutes all related generic parameters, the projection oracle proves the complete bound conjunction against the resolved graph, and lowering emits one specialized direct native call; no/unknown proof rejects that call with T0119; this uniformly covers contracts such as `Into<T>`, `Borrow<T>`, `TryInto<T>`, `IntoIterator<Item=T>`, auto traits, and their combinations without package- or operation-specific dispatch; canonical string/path `AsRef` keeps its established structural adapter
projection_deferred_callback_templates: direct Rust `Fn`/`FnMut`/`FnOnce` inputs may retain projected generic types recursively inside callback parameters and their exact result; dependency projection does not require those nested types to be concrete, while call analysis jointly binds them from the written callable, ordinary arguments, and destination before validating the callable and emitting its native adapter; a parameterized native callback-adapter trait emits no misleading Terrane interface and may select one unique blanket implementation whose generic `Self` has exactly one parenthesized `Fn`/`FnMut`/`FnOnce` bound; projection substitutes adapter trait arguments and retains every implementation-local result bound, call analysis infers and oracle-proves the complete closed recipe, and lowering passes a fresh concrete Rust closure instead of Terrane's reusable `Arc<dyn Fn...>` representation; zero recipes preserve the ordinary native obligation, multiple distinct recipes decline, and no/unknown result-bound proof is T0119; higher-ranked binders are retained through recipe metadata and proof, while a Terrane function result carrying the bound lifetime still requires a source-representable non-escaping type
projection_opaque_results: a producer-selected return-position `impl Trait` is retained as an owned opaque witness with its canonical bounds and associated-type equalities recursively nested in the named outer result; it has stable projected identity but deliberately no explicit Rust source spelling; Terrane admits it only as expression-local chain state ending in a terminal operation, and lowering emits the producer/member expression directly so rustc selects the hidden concrete type; generated aliases, explicit signatures, bindings, returns, fields, collections, captures, and suspension of that intermediate are forbidden
projection_namespace_overlays: a directly declared provider package may use package.metadata.terrane.namespace-overlays to attach one explicitly enabled projected module beneath another directly declared dependency namespace; presentation namespace changes but exact Rust owner/path and dependency provenance remain; declarations enter cache identity; undeclared, self, ambiguous, empty, overlapping, and colliding overlays fail projection; no upstream shadowing or crate-specific compiler dispatch
projected_reexports: prefer reachable non-prelude public Rust paths, then shortest path, then lexical order at equal depth; a prelude path is valid only when no substantive path exists; assign namespace-qualified identity from that canonical path; re-export and definition are one type; distinct same-named sibling items remain distinct; output ordering and generated imports are deterministic
projection_item_namespaces: Rust modules are structural namespace segments rather than projected values or unavailable declarations, so one segment may simultaneously name a module for `/deps/.../segment` traversal and a function or constant imported from its parent; value demand resolves the Rust value declaration, opaque result owners include retained witness bounds in their internal identity, owner import closure matches that exact identity rather than a same-base candidate, and public output aliases are expanded before Option/Result classification
projection_aggregates: Option<T> in parameters/results => T|none; Vec<T>, HashMap/BTreeMap, HashSet/BTreeSet, and homogeneous tuples recursively cross as matching Terrane collections when every component is representable; map keys/set items must be scalar (never optional/collection/tuple/foreign); shims elide identity-element Vec mapping and move uniquely-owned tuple items without clone/panic; Vec<u8> remains bytes; heterogeneous tuples decline
projection_payload_enums: a payload-bearing Rust enum remains one nominal projected class; each public unit variant is a zero-argument static constructor and each public single-field tuple variant whose payload has an owned boundary representation is a one-argument static constructor, both retaining the exact Rust variant name; every such enum has shared `variant-name` returning the exact public Rust variant name and a consuming `into-<Variant>` for each supported payload variant returning payload|none; inspection never clones a payload; stripped/non-exhaustive variants yield `unknown` from `variant-name`; named-field and multi-field variants, borrowed payloads, open generics, and payloads without one owned representation remain explicit per-variant declines rather than erased fields
projection_nested_outcomes: Rust Option<Result<T,E>> => T|none throws projected-E; None returns none, Some(Ok(value)) returns value, and Some(Err(error)) throws E; Result<Option<T>,E> has the same Terrane signature but records a distinct Rust wrapper order and lowers accordingly; no Result value is silently stringified or exposed as an optional sentinel
projection_typed_errors: each admitted Rust E in a projected Result has its canonical projected object identity and is catchable by that imported class; a catch binding uses the ordinary throwable protocol and `error.message` is the sole implicit boundary display supplied by projection, populated by Rust Display when E implements it; matching is identity-based, never display-text-based; arbitrary fields and the native Rust E payload are not preserved after constructing the throwable; a non-standard unprojectable E declines the call rather than collapsing to dependency-error, while an external std/core error that rustdoc does not expose as a projectable public identity retains the established dependency-error boundary
projection_callbacks: concrete or call-site-closable Fn/FnMut/FnOnce parenthesized bounds => shared/mutable/consuming Terrane function type at free-function and projected-method calls; generic value occurrences unify with surrounding projected inputs, including nested owned aggregates; Future<Output=T> return => async function; artifact records invocation mode, retained/'static, Send, Sync; lowering performs argument/result conversion in the exact Rust closure; retained callback rejects borrowed receiver/ref capture, transferable callback rejects local capture, mutable callables preserve repeatable owned state and separate by copying their current environment, consuming callables transfer once, throwable mismatch rejects unless Rust result represents it; conflicting/uninferable/HRTB/lifetime-generic shapes decline; Rust traits project toward Terrane interfaces because both are named contracts, while closures and functions remain structural callables
projection_into_future: exact canonical core::future::into_future::IntoFuture on a concrete non-async return => task of substituted associated Output; owner arguments close generic Output before call specialization; Result output retains projected success/failure; task construction is inert and first poll evaluates the Rust call once then converts once before ordinary async polling; similarly named traits and unresolved/borrowed/lifetime-escaping/unrepresentable outputs decline
projection_interfaces: eligible fixed-signature Rust traits => nominal Terrane interfaces; plain &self/&mut self/self => shared/mutable/consuming requirements; Arc<Self>/Box<Self>/Pin<&mut Self> and other wrapped receivers decline; required members demand class implementations; provided members supply callable defaults but Terrane overrides currently require owned non-Result signatures; required borrowed parameters and required Result methods decline; local classes adopt projected identities with implements and lower to foreign impls; projected error metadata contributes ordinary throwable protocol eligibility/conformance; owned concrete generic and impl-Trait inputs retain written lifetime/Send/Sync bounds and specialize from the written class argument; only owning Box<dyn Interface + Send/Sync> parameters project, retain every auto-trait bound, admit a conforming concrete class or an applied generated wrapper for that interface or a projected subinterface, and box in the shim; bare/borrowed dyn, Box<T>, multiple principal traits, boxed dyn results, and wrappers lacking the requested auto traits decline; the oracle records closed foreign Send and Sync independently and semantic checks recursively traverse the complete effective local-class field graph; canonical Drop direct imports decline with consuming-destruct guidance, while a projected Drop supertrait requires consuming destruct and reuses one lineage-aware Rust Drop; projected async methods require a Send interface and async main context, resource classes require consuming receivers, dropped Rust futures detach cancellation cleanup, nested finally completes before receiver release, and executor shutdown drains outstanding cleanup; exactly one non-generic associated slot may be closed explicitly with Interface of ConcreteType, where ConcreteType is currently a closed boundary-representable scalar, aggregate, or projected foreign type and source-class arguments remain deferred pending an explicit Rust boundary conversion contract; the applied nominal identity is recursively substituted through annotations, reflected descriptors, inherited methods, concrete and erased Rust crossings, and generated associated-type declarations; unapplied identities are rejected recursively even when nested inside an aggregate or callable annotation; independently projectable supertraits form a complete recursive, diamond-safe conformance and foreign-implementation closure; final oracle admission removes every generic function whose projected interface bound was declined; bare/inferred applications, source-class arguments, multiple or generic associated slots, failed associated bounds, incoherent erased bindings, static or generic methods, higher-ranked lifetimes, unsupported owning containers, and unprojectable members or supertraits decline with stable reasons; Rust traits project to Terrane interfaces, never Terrane traits, because the latter are source implementation composition through uses
applied_type_annotations: constructor arguments are recursively resolved type expressions, so a closed projected interface may appear inside lists, tuples, map/entry values, channels and other aggregate positions that admit object elements; ordinary scalar-key and scalar-set restrictions remain; `|` belongs to the current `of` argument, while grouping makes the completed application optional, as in `(sequence of int)|none`
projected_interface_inheritance: a subclass inherits the complete effective projected-interface set of every base; semantic generic-bound selection and lowering consume that same closure, including its closed associated binding
projection_foreign_ownership: projected foreign types record Rust Clone plus separately oracle-proven Send and Sync support; a Rust struct with a complete public, non-hidden, exhaustive, named field set projects representable fields as ordinary Terrane class fields, and owned native structs construct directly; a lifetime-parameterized struct is admitted only when every field has an exact owned conversion recipe, with borrowed/optional strings and slices stored as owned strings/lists and reconstructed as a temporary native view immediately around shared-borrowed or consuming calls; Terrane values retain ordinary ownership and expose no Rust lifetime; mutable borrowed inputs and borrowed outputs remain unavailable; private, hidden, tuple, non-exhaustive, or incompletely convertible non-lifetime structs remain opaque, while equivalent lifetime-bearing structs decline; a source class containing a non-Clone foreign field becomes resource-owning, transfers on assignment, and never receives an invalid generated Clone implementation; projected interface and retained-bound obligations recurse through the complete effective source-class field graph and closed foreign auto-trait facts; persistent-list iteration or indexing that would copy an item containing a non-Clone projected foreign value is rejected with T0135 and tracking key `collections/consume-non-clone-foreign-items`
projection_transitive_owners: terrane-dependencies.lock preserves the complete exact resolved Rust graph; a public signature exposed through a declared package may privately name one unique recursive registry owner through an exact generated Cargo edge with no feature widening, while Terrane source visibility remains restricted to declared /deps aliases; multiple locked versions or an unrepresentable source decline explicitly rather than selecting heuristically; direct declaration remains the explicit unification mechanism
projection_inventory: ordinary package analysis refreshes the ignored terrane-projection.generated.trn beside package.toml; it orders currently required unavailable declarations plus the required-admitted index first, active projected source units second, and unreachable unavailable declarations last; report comments retain exact paths/reasons/demand sites, partial function/callback contracts, and transitive requirements; deterministic Generated source unit markers introduce separately syntax-validated, one-namespace compiler-owned Terrane units, allowing multiple and cyclic /deps namespaces in the physical artifact without weakening S2002 or producing S0005 for the container in editor diagnostics; native identity and Terrane source naming are separate: canonical public re-exports and repeated observations of one exact Rust type share one readable nominal declaration, a unique closed generic uses its constructor name, actual collisions first gain deterministic path-derived qualifiers and use a short digest only if those qualifiers still collide, and closed numeric specializations of one Rust type constructor share one public declaration while exact native arguments and provider paths remain compiler-owned semantic metadata for specialization-specific member selection and generic Rust lowering; universal projected-call panic translation is implicit and omitted from generated declarations, while projected Rust Result errors retain an explicit throws dependency-error contract; admitted nominal classes contain the union of members whose rendered declaration passes parser validation independently of imports, while admitted members whose rendering is not syntax-valid remain inert comments with exact parser rejections; unsupported public members remain inert comments inside the owning class with native path, recoverable signature, and exact gap, while complete unavailable records remain in the report; a demanded unavailable struct, enum, alias, or trait with recoverable nominal shape receives an active class/interface skeleton under its public dependency namespace, while unsupported native generics/lifetimes remain adjacent residual obligations and prevent lowering registration; only source demand for an unresolved operation fails semantic analysis
types: Option<T> => T|none; Result<T,E> => T throws projected-E; &self => shared receiver; &mut self => mutable-receiver contract; self => move. Borrowed receivers use ordinary member-call syntax; the contract drives Rust borrowing and mutable binding
panic: unwinding profile implicitly converts a crossing projected-call panic to dependency-panic without repeating that universal effect on every generated declaration; abort profile emits no catch boundary and generated Cargo uses panic=abort. Receiver crossings use an explicit AssertUnwindSafe boundary because foreign receiver state is the captured logical invariant; receiver-free crossings retain Rust's UnwindSafe proof
async_projection: Rust async fn => async Terrane callable returning a task; generated async shim awaits before result conversion and Result/panic mapping; arguments and the Rust future are constructed at call evaluation; Terrane-driven projected interface calls run in the generated async-entry runtime, while an arbitrary Rust caller must poll the returned method future on a thread entered into that runtime because bare-thread polling is outside the projected contract
async_sequence_projection: concrete owned producer with async borrowed next()->Result<Option<Item>,E> plus consuming close() => resource-owning linear endpoint; await next => async-iteration-step of Item with item/end/value, Rust Err => dependency throwable, cancellation => enclosing task outcome; borrowed operation must be awaited immediately and future is constructed before wrapper so its mutable borrow spans one suspension; close consumes; Drop releases but is not graceful close; borrowed/open-associated/lifetime-dependent items decline
async_sink_projection: concrete owned endpoint with async borrowed send(Item)->Result<bool,E> => resource-owning linear sink; await send => async-sink-outcome with accepted=bool and closed=!bool, Rust Err => dependency throwable, cancellation/deadline => enclosing task outcome; borrowed operation must be awaited immediately so its mutable borrow spans one suspension; sync/async flush retains failure contract; close and split consume; split transfers whole duplex endpoint into independent halves; Drop releases but is not graceful close; borrowed/open-associated/lifetime-dependent payloads decline
chain_only_projection: introduced in schema 20; root/continue/terminal roles for concrete otherwise-unnameable Rust intermediates, including one retaining a borrow from a named input that outlives the expression; intermediate may exist only as receiver subtree inside one nested Terrane expression, never binding/store/return/capture/Terrane argument/await; terminal output must be owned/projectable; lowering emits one Rust chain and applies argument conversion plus panic/error/async/result boundaries only at terminal; tooling visibly says chain-only/non-escaping; non-continuing or non-owned-terminal members decline; concrete adapters may execute open generic APIs such as SQLx Query internally, but the open generic itself remains declined
async_closure: 'value = async function R; args' => async function value; each invocation owns a fresh future and invocation-local copies of captured shareable state; transferable closure captures only transferable values
tooling: completion/signature/hover and declined reasons are ADVISORY; Cargo/rustc remain authoritative
execution: Rust inspection and generated-crate compilation use the build capability policy; fetch may be online, then compilation is offline/frozen; injecting exact bound-owner edges rewrites only the generated projection manifest and resolves offline without `--locked`, with graph integrity supplied by the pre-injection manifest+lock identity and content-hashed exact dependency list
cache_identity: manifest + lock checksum + features/default-feature policy + effective target (CARGO_BUILD_TARGET, then Cargo build.target, then pinned-stable host) + toolchain + package source checksums + sandbox tier; project-local cache keeps current + at most 3 prior projections
build_toolchain: each Terrane version selects one stable Rust release (currently 1.98.1), shared by the compiler workspace rust-toolchain.toml/rust-version and BUILD_TOOLCHAIN/generated-crate rust-version/rust-toolchain.toml; package rust-toolchain = "system" uses a compatible ambient compiler and is recorded
lint_policy: user builds inherit RUSTFLAGS; generated manifests forbid compiler-guaranteed unsafe_code, while packages containing maintained authored Rust modules use deny so only an explicitly annotated module can own a required unsafe boundary; conformance denies all warnings
cargo_cache_wrapper: sccache only when TERRANE_SCCACHE=1; choice participates in generated-crate cache identity
toolchain_report: terrane toolchains lists only stable pins Terrane requested; reports current/not-current use, never removes or says safe
containment: bwrap-capable hosts contain compilation; other hosts report the unavailable tier and continue under declared host policy
lock_change_diagnostic: machine-independent terrane-projection.lock history records top-level names, static Type::member, and instance Type.member; after declared resolution fails, matching removal history emits S2031 at the import/member selection with the dependency version change, while a never-present name keeps its ordinary missing-member diagnostic
```

## RUST

- Rust is native lowering, not foreign runtime.
- Generated identifiers use exact deterministic injective encoding; punctuation never normalized away.
- Source name, generated Rust name, native/link symbol are independent reflected identities.
- Inline Rust block/expression and maintained `.rs` files are first-class escape hatches with explicit safety/source mapping.
- Generated/handwritten Rust may call each other within one Rust crate graph.
- Rust errors/diagnostics map back to Terrane spans without hiding originals.
- Ejection tooling can produce maintainable generated Rust/Cargo artifacts.
- Lowering itself emits canonical Rust. A bundled pinned formatter may validate an untouched
  generated artefact, but its formatted copy is discarded; mismatch is a compiler defect, never a
  silent rewrite.
- Artifact layout: streamed Rust is one complete standalone unit; named-file output and Cargo builds
  share a split renderer in which the requested entrypoint contains authored lowering plus one
  relative include, while `<entrypoint-stem>.support.rs` contains compiler prelude, runtime,
  structured-error and source-site infrastructure, selectively included bundled `/core`
  implementation code, and projected `/deps` lowering; user-authored package modules remain in the
  entrypoint. The support sidecar is emitted even when empty, preserving a uniform
  two-file artifact contract.

## NATIVE INTEROP

- System/C crosses an explicit ABI boundary.
- Rust remains native lowering rather than a foreign runtime transition.
- Foreign-runtime adapters are deferred until after version one.
- Any later adapter must define conversion, ownership, lifetime, thread, exception, deployment, and tooling contracts without weakening Terrane semantics.
- C++ initially crosses through C-compatible shims or Rust bridges; arbitrary C++ ABI integration is deferred.

## SOURCE INTELLIGENCE

```yaml
authority: one compiler-owned immutable snapshot/query engine; no public internal Rust structs and no second parser/resolver
schema: version 1.0 additive public projection; canonical UTF-8 half-open byte spans
identity: compiler version + schema version + content-derived snapshot ID + logical URI/source hash; semantic options include manifest/lock hashes, target, profile and capabilities; generated spans require exact build ID
syntax: exact tokens/trivia + snapshot-local node IDs + named child fields + complete/error/recovery/unsupported state; remains available after syntax errors
semantics: compiler resolution supplies canonical symbol/descriptor identity, definition/references, type, ownership and capability facts; availability is known|unresolved|invalid|not-yet-analyzed|unsupported
query: locate | definition | references | bounded structural find | exact-build generated association; deterministic URI/byte ordering, bounded pages, opaque snapshot/query-bound continuations, recovery matching opt-in
transport: terrane tooling --stdio JSON-lines; terrane query --request <json-file> one-shot; request IDs, schema errors, cancellation and expiry explicit; protocol frames only on stdout
edits: proposals carry snapshot + all affected hashes + non-overlapping byte replacements + preview diagnostics + semantic reanalysis state; apply preflights every hash and uses same-directory temporary replacement; rename follows canonical identity and rejects capture
format: compiler lossless tree; terrane fmt [--check] and LSP formatting; canonical parser-proven assignment/infix spacing, safe trailing whitespace, idempotence, comments/multiline/newline style and malformed-region preservation
lsp: shared snapshot IDs for diagnostics, semantic hover, definitions, references, rename, symbols and formatting; versioned workspace edits; negotiated UTF-8/UTF-16/UTF-32 boundary
reference: docs/tooling-schema.md
```

## COMPILER

Pipeline:

```text
manifest/source set
-> UTF-8 source files + stable file IDs/spans
-> lossless tokens/trivia/layout
-> lossless CST
-> compact semantic AST
-> namespace assembly/import resolution
-> names/types/callable contracts/ownership/control-flow
-> typed semantic IR
-> Rust-oriented lowering IR
-> deterministic Rust + Cargo
-> rustc/Cargo
-> source-mapped diagnostics/artifacts
```

Contracts:

- `check`, `rust`, `build`, `run` share pipeline.
- `build --release` and `run --release` select Cargo's optimized release profile; development and release artifacts are cached separately.
- Parse recovery never promotes recovered invalid nodes to lowering.
- Diagnostic: stable code, primary source span, labels/notes/help; originating bytes including UTF-8.
- Generated output deterministic for compiler version, target, declared inputs.
- No universal boxed `Value` shortcut; finite dynamic alternatives use closed representations when sound.
- Direct native lowering only when Rust operation exactly matches complete Terrane semantics.
- Reflection exposes semantic descriptors, source/generated/native identities, compilation artifacts subject to profile.
- Development compilation explains lowering/cost/copies/COW/ref/move/foreign transitions.
- Cache keys include source set, compiler version, target, dependencies, import/modifier plans, build selections, relevant options.
- Conformance cases are implementation truth. Accepted compile cases compile generated crates; runtime changes execute; generated-Rust goldens reviewed.
- Source warnings do not fail `check`/`rust`/`build`/`run`; generated/compiler Rust warnings remain denied. Warning conformance files match code, source-relative span, severity, message, order, and multiplicity exactly.
- Binding and function usage is resolved by declaration identity. `W4001`: initialized local value is never read; suppressed for a name beginning with `_`. `W4002`: initial/later store cannot reach a read before definite replacement; conditional stores do not kill incoming values. Bare `_` creates no binding and receives neither warning. `W4006`: the first read of an underscore-prefixed retained binding reports that its intentionally-unused name is stale and suggests removing the leading underscore. Parameters and loop targets are excluded from `W4001`; parameter-name linting is deferred to an explicit policy. Lowering consumes warning-only locals so generated Rust stays warning-free. `W4003`: authored union arm repeats an earlier canonical semantic identity; lowering normalizes it away. `W4004`: namespace-wide import shadows, replaces, or is skipped for a different visible object. `W4005`: authored top-level function is never referenced.
- See `docs/compiler-plan.md` for milestone sequencing; do not infer implementation status from this design reference.

## DIAGNOSTIC HOTSPOTS

Must reject with source-oriented help:

```text
print render             -> adjacency is not invocation; suggest member attachment or `print; render`
count-1                   -> lexical attached digits-only suffix; suggest `count - 1`
a+ b                      -> undeclared left-attached/postfix operator
nested; other; value      -> nested call must be parenthesized
for x=(call; a);...       -> calls in for clauses grouped
foo                       -> unresolved name when not imported or declared
value .member             -> invalid; whitespace before a member dot
list<string>              -> angle generic spelling invalid; canonical `list of string`
function f of T ...       -> source type parameters unsupported
===                       -> invalid; choose `==`, `is`, or `is a`
const                     -> invalid declaration word; use `constant`
```

## INVARIANT

Priority: these override examples/lowering sketches/plans. Condensed from full spec §41:

1. Everything semantic is object; representation can specialize invisibly.
2. Values are typed once context fixes them; dynamic != weak coercion; constraints optional/local/real. Numeric constants take destination/operand arithmetic, numeric destinations preserve the exact value or throw, and written coercion selects alternative object-driven policies.
3. Assignment value-semantic; COW allowed; `ref` shared identity; `move` ownership transfer.
4. One lookup view; imports bind ordinary names scoped to the containing block, function, or namespace.
5. Namespace segments `/`-separated, lowercase `[a-z]([a-z0-9]|-[a-z0-9])*`; `/` is both root anchor and separator, and is never an identifier character.
6. Compact operator-bearing names differ lexically from spaced operators.
7. `foo.bar` member; `.bar` object; `foo; bar` explicit argument; adjacency never call.
8. Compile-time structural slots never depend on same-spelled ordinary bindings.
9. Empty blocks legal; conventional control flow.
10. Public/dynamic permissive defaults; explicit private/protected/strict available.
11. Rust canonical; output deterministic/readable/source-mapped; name encoding injective.
12. Native/Rust/system/foreign dependencies share inspectable graph; foreign boundaries explicit.
13. Reflection/debugging/performance explanation compiler contracts.
14. Missing target capabilities diagnose; never silently weaken semantics.
15. Equality, identity, membership distinct.
16. Non-owning reference/shared owner/address/ABI contracts are distinct; never silently convert or weaken.
17. `void` no value; `opaque` hidden representation.
18. Derived reference provenance never widens.
19. Source/generated/native names independent.
20. Destruction is deterministic only for lexical ownership and acyclic final shared-owner release.
21. `int` exact arbitrary precision with adaptive promotion/normalization; fixed widths expose arithmetic bounds/overflow policy; numeric destination conversion is exact-or-throw.

## OPEN

Validation/prototype points, not permission to invent semantics:

- zero-argument invocation shorthand beyond the required explicit `;` remains a possible future ergonomic study; current grammar requires `;`;
- map literal syntax;
- exact COW split policy;
- conversion-declaration coherence: conflicting declarations for one source/destination pair, and whether a declaration may be added for a type the author does not own;
- numeric arrival diagnostics: final spelling of the typed-value exact-arrival predicate (proposed `value.fits; Destination`), wording/severity of the typed false-`is a` and lossy constant-division lints, stable `T00xx` codes for contextual-constant rejections, and whether `integer-conversion-overflow` keeps its name now that it covers every exact-or-throw arrival failure;
- the version-one async surface: task identity/linearity, un-awaited task disposal, scope failure semantics for surviving siblings, defined cancellation points, and the executor boundary the language fixes versus the profile selects;
- dynamic finite-union representation;
- reference representation thresholds after source validation (borrow/stable handle for non-owning `ref`; Rc/Arc/custom owner for owning `shared ref`);
- public-by-default API lint/strict policy;
- reflection artifact embedding policy;
- importer composition/evaluation ergonomics.

## DEFERRED

Not version-one; no private incompatible syntax:

- core constructs supplied/replaced as scoped objects (including `function`); version one keeps core constructs structural;
- source-declared generics;
- compact map literals;
- stateful hot-code replacement;
- arbitrary C++ ABI integration;
- multimethod/generic-function dispatch;
- foreign-runtime adapters;
- `when build` compile-time selection; its settled design is documented under future compiler internals and scheduled for Milestone 32;
- open package-supplied `with` decorative modifiers; version one defers custom declaration modifiers, while its settled design is documented under future compiler internals.

## AUTHORING CHECKLIST

Before writing Terrane:

1. Determine implemented subset from conformance cases, not this design.
2. Declare namespace segments with `/` separators, lowercase only; never whitespace tiers.
3. Import explicitly; `as` renames. Imports bind ordinary names directly.
4. Preserve compact punctuated identifiers; put spaces around infix operators.
5. Use `;` for every call, including zero args.
6. Prefer one call per statement. A single parenthesised call in an argument list is ordinary; two or more should be bound to named intermediates. Nesting is legal but obscures evaluation order, accumulates meaningless parentheses, and leaves diagnostics and traces pointing at an anonymous subexpression.
6. Parenthesize nested calls and calls in three-clause `for` clauses.
7. Use indentation; empty block is legal.
8. Write type after name; use `T|none`; canonical constructors use `of`.
9. Let a single numeric destination perform its exact-or-throw conversion; write `coerce` or a rounding member when selecting a different policy. Never assume foreign conversion.
10. Choose value assignment vs `ref` vs `move` deliberately.
11. Use `constant`, not `const`; distinguish `void`/`opaque`.
12. Do not use source generics, `===`, adjacency calls, or implicit object imports.

## MAINTENANCE CHECKLIST

When full spec changes:

1. Update affected keyed section(s) here in same work unit.
2. Update `Retrieval map` if a topic/key moved or was added.
3. Keep `INVARIANT` synchronized with full spec §41.
4. Keep `OPEN` synchronized with §40 and `DEFERRED` with §42.
5. Keep grammar/call precedence synchronized with §34.
6. Keep diagnostic hotspots synchronized with normative diagnostics/acceptance tests.
7. Never promote planned behavior to implemented; conformance remains implementation truth.
8. Search this file for superseded terms/decisions after editing.
9. Treat a forced fallback to the full spec as a retrieval defect when the answer can be captured compactly: repair the smallest relevant key/rule in the same work unit.
10. Keep size bounded: prefer replacing vague text, deduplicating, or adding a precise pointer over accumulating explanatory prose.
