# Terrane object surface — implemented today

This map describes the language surface implemented by the compiler today. It is not a map of every object proposed by the language draft.

Compiler-only host bindings and generated Rust ABI details are documented separately in
[`internal-surface.md`](internal-surface.md); they are not part of this public map.

Status labels:

- **implemented** — checked and lowered by the current compiler pipeline.
- **name only** — reserved in the compiler-owned namespace, but has no implemented value semantics or operations yet.
- **source-declared** — supplied by a Terrane program rather than the prelude.

Source-declared and projected class, interface, and trait declarations have namespace-qualified
identity, but their roles differ. Interfaces are named nominal contracts adopted with `implements`;
traits are source implementation composition adopted with `uses` and are not types; protocols are
unnamed structural operation shapes and therefore have no declaration identity. Named built-ins use
canonical `/core` identities; synthesized composed descriptors such as references, callables,
optionals, and unions retain canonical source-shaped spelling. Import aliases preserve identity, and
same-named declarations from different namespaces remain distinct.
Built-ins and source declarations share `DescriptorContract`, with immutable built-in templates
kept separate from per-unit source contracts. Canonical contracts carry identity, kind, category
conformance, complete instance and static member inventories, their callable subsets, stable
operation IDs, invocation-only constraints, fields, nominal bases, interfaces, traits, and
reflection data. Member lookup, nominal relations, compatibility, dispatch, reflection, and
structural protocol checks query those contracts for built-in and source-declared receivers.
Compiler descriptor operations are internal machinery, not another source construct. The
implemented non-iteration protocol example is `truth`: a source class with a synchronous,
non-throwing, shared, parameterless `truth bool` method may be used directly as an `if` or `while`
condition.

Every `/core/types` descriptor is available as an implicit language construct. Operational core
tooling is not implicit: authored and bundled source must import an individual object or use
`import /core/<facility>` to bind every public importable object in that namespace. Core tool
symbols retain their `/core` identities; compiler-only lowering keys are not namespace objects.

## Tree

```text
Terrane package
├── public compiler-shipped core surface
│   ├── /core
│   │   ├── /core/output
│   │   │   └── print                          function
│   │   ├── /core/types
│   │   │   ├── bool                           type descriptor
│   │   │   ├── int                            type descriptor
│   │   │   ├── abstract category descriptors
│   │   │   │   ├── number
│   │   │   │   ├── integer
│   │   │   │   ├── fixed-integer
│   │   │   │   ├── signed-fixed-integer
│   │   │   │   ├── unsigned-fixed-integer
│   │   │   │   └── floating
│   │   │   ├── signed fixed-width descriptors
│   │   │   │   ├── int8
│   │   │   │   ├── int16
│   │   │   │   ├── int32
│   │   │   │   ├── int64
│   │   │   │   └── int128
│   │   │   ├── unsigned fixed-width descriptors
│   │   │   │   ├── uint8
│   │   │   │   ├── uint16
│   │   │   │   ├── uint32
│   │   │   │   ├── uint64
│   │   │   │   └── uint128
│   │   │   ├── floating-point descriptors
│   │   │   │   ├── float                     spelling of float64
│   │   │   │   ├── float32                   canonical descriptor
│   │   │   │   └── float64                   canonical descriptor
│   │   │   ├── string                         type descriptor
│   │   │   ├── none                           type descriptor
│   │   │   ├── bytes                          implemented type descriptor
│   │   │   ├── overflow-result                compiler-supplied result type
│   │   │   └── div-rem-result                 compiler-supplied result type
│   │   ├── /core/errors
│   │   │   ├── throwable                      catch-all throwable interface
│   │   │   ├── arithmetic-overflow            compiler-owned throwable object
│   │   │   ├── division-by-zero               compiler-owned throwable object
│   │   │   ├── integer-conversion-overflow    compiler-owned throwable object
│   │   │   ├── negative-shift-count           compiler-owned throwable object
│   │   │   ├── resource-error                 compiler-owned throwable object
│   │   │   ├── coercion-error                 compiler-owned throwable object
│   │   │   ├── decode-error                   compiler-owned throwable object
│   │   │   ├── index-error                    compiler-owned throwable object
│   │   │   ├── missing-key                    compiler-owned throwable object
│   │   │   ├── dependency-error               dependency `Result::Err` throwable object
│   │   │   └── dependency-panic               unwinding dependency-panic throwable object
│   │   ├── /core/encodings
│   │   │   ├── utf8                           encoding object
│   │   │   ├── utf16-le                       encoding object
│   │   │   ├── utf16-be                       encoding object
│   │   │   ├── utf32-le                       encoding object
│   │   │   └── utf32-be                       encoding object
│   │   ├── /core/collections
│   │   │   ├── iterator                       typed linear iterator constructor
│   │   │   ├── iteration-step                 typed item/end result constructor; `.end` constructs exhaustion
│   │   │   ├── list                           insertion-ordered sequence constructor
│   │   │   ├── map                            insertion-ordered key/value constructor
│   │   │   ├── set                            insertion-ordered unique-value constructor
│   │   │   ├── tuple                          fixed-length sequence constructor
│   │   │   ├── range                          half-open range constructor; `.through` is inclusive
│   │   │   ├── entry                          key/value pair constructor
│   │   │   ├── unordered-map                  deterministic unordered map constructor
│   │   │   └── unordered-set                  deterministic unordered set constructor
│   │   ├── /core/codecs                       public facility namespace
│   │   ├── /core/compression                  public facility namespace
│   │   ├── /core/concurrency                  public facility namespace
│   │   ├── /core/documents                    public facility namespace
│   │   ├── /core/filesystem                   public facility namespace
│   │   ├── /core/documents/json               public facility namespace
│   │   ├── /core/networking                   public facility namespace
│   │   ├── /core/process                      public facility namespace
│   │   ├── /core/random                       public facility namespace
│   │   ├── /core/streams                      public facility namespace
│   │   ├── /core/networking/tls               public facility namespace
│   │   ├── /core/urls                         public facility namespace
│   │   ├── /core/random/uuid                  public facility namespace
│   │   ├── /core/documents/yaml               public facility namespace
│   │   └── /core/async
│   │       └── task-scope                     structured task-scope constructor
├── default prelude
│   ├── print                                  binding to /core/output::print
│   ├── utf8                                   encoding name for /core/encodings::utf8
│   ├── utf16-le                               encoding name for /core/encodings::utf16-le
│   ├── utf16-be                               encoding name for /core/encodings::utf16-be
│   ├── utf32-le                               encoding name for /core/encodings::utf32-le
│   ├── utf32-be                               encoding name for /core/encodings::utf32-be
│   └── task-scope                             binding to /core/async::task-scope
└── public core facility details
    ├── /core/streams                          standard streams; requires `process`
    │   ├── stream-operation-result            failed / message
    │   ├── read-result                        bytes / completed / end / failed / message
    │   ├── text-read-result                   text / completed / end / failed / message
    │   ├── write-result                       encoded bytes / completed / failed / message
    │   ├── byte-reader                        inferred resource-owning process-byte input
    │   ├── byte-writer                        inferred resource-owning process-byte output
    │   ├── text-reader                        inferred resource-owning encoded input adapter
    │   ├── text-writer                        inferred resource-owning encoded output adapter
    │   ├── stdin                              byte-reader factory
    │   ├── stdout                             byte-writer factory
    │   └── stderr                             byte-writer factory
    ├── /core/filesystem/paths                 lexical filesystem paths
    │   ├── path                               platform-neutral lexical component value
    │   ├── normalise-path                     lexical `.` / `..` resolution, root-bounded
    │   ├── join-path                          lexical base/child resolution
    │   ├── path-components / path-is-absolute
    │   └── path-name / path-parent / path-stem / path-extension
    ├── /core/filesystem                       filesystem operations; requires `filesystem`
    │   ├── filesystem                         unforgeable capability, acquired via filesystem-capability
    │   ├── filesystem-capability() -> filesystem
    │   ├── filesystem-operation-result        failed / message
    │   ├── existence-result                   exists / failed / message result object
    │   ├── file-handle                        inferred resource-owning file stream
    │   ├── directory-handle                   inferred resource-owning directory anchor
    │   ├── open-file(filesystem, path, …) -> file-handle
    │   ├── file-read(filesystem, ref file-handle, limit) / file-write(filesystem, ref file-handle, data, offset)
    │   ├── file-flush(filesystem, ref file-handle) / file-sync-data(filesystem, ref file-handle)
    │   ├── file-sync-all(filesystem, ref file-handle) / file-close(filesystem, file-handle)
    │   ├── filesystem-open-beneath(filesystem, directory, relative, cross-filesystem) -> directory-handle
    │   ├── open-file-beneath(filesystem, ref directory-handle, relative, …) -> file-handle
    │   ├── filesystem-exists / filesystem-metadata / filesystem-symlink-metadata
    │   ├── filesystem-canonical / filesystem-realpath / filesystem-read-link
    │   ├── filesystem-read-bounded / filesystem-write-atomic
    │   └── filesystem-rename / filesystem-remove
    ├── /core/process                          process and host environment; requires `process`
    │   ├── native-string                    lossless text-or-raw platform value
    │   ├── arguments / environment            explicit process snapshots
    │   ├── cli-schema / parse-command-line    schema-driven options and structured diagnostics
    │   ├── exit-status / make-exit-status / exit explicit validated termination
    │   └── process-host-name-result / process-host-name lossless platform host name or translated host failure
    ├── /core/documents                        structured document values and mappings
    │   ├── document-integer                   exact integral value; text uses canonical exact number spelling
    │   ├── document-decimal                   coefficient / exponent / canonical exact text value
    │   ├── document-value                     none / bool / integer / decimal / string / list / map
    │   ├── document-result                    value or failed / message / path / expected diagnostic
    │   ├── document-mapping                   descriptor name, expected kind, fields, defaults, unknown-field policy
    │   ├── serializable / deserializable      explicit manual typed conversion interfaces
    │   ├── document-decodable                 explicit compiler-derived fieldwise decoding opt-in
    │   ├── document-validatable               optional post-decode `string|none` validation contract
    │   ├── document-diagnostic                path / expected / actual kind / reason / message / decode and field source
    │   ├── document-decode-outcome of T       initialized T value plus deterministic typed diagnostic list
    │   ├── exact scalar/list/map constructors programmatic document construction with duplicate rejection
    │   └── decode-document                    descriptor-driven document-shape validation
    ├── /core/documents/json                   JSON policy and document integration
    │   ├── json-options / default-json-options depth and byte limits; duplicates always rejected
    │   ├── parse-json / stringify-json / canonical-json
    │   │                                       JCS key ordering/escaping with exact, ECMAScript-shaped numbers
    │   ├── decode-json / encode-json
    │   └── decode-typed-json                  opted-in concrete class decoding with explicit unknown policy
    ├── /core/documents/yaml                   YAML policy and document integration
    │   ├── yaml-options / default-yaml-options / make-yaml-options
    │   │                                       depth (capped at 255), byte, and alias-expanded-node limits
    │   ├── parse-yaml                         JSON-shaped safe scalars; tags and duplicate keys rejected
    │   ├── stringify-yaml                     emits canonical JSON, a valid YAML 1.2 document
    │   ├── decode-yaml / encode-yaml
    │   └── decode-typed-yaml                  same typed conversion and diagnostics after safe YAML parsing
    ├── /core/logging                          structured observability; requires `logging`
    │   ├── log-level / *-level                trace through critical ordered severities
    │   ├── log-value                          lazy document-value rendering protocol
    │   ├── log-field / field / secret-field   key, renderer, secrecy, and compiler-injected field source
    │   ├── log-sink / memory-sink / console-sink / failing-sink
    │   │                                       explicit deterministic, host-console, and failure-witness sinks
    │   ├── logger / logger-options             explicit sink, target hierarchy, limits, fields, and spans
    │   ├── default-logger / named-logger       explicit nonambient construction helpers
    │   ├── with-field / with-span              immutable context enrichment
    │   ├── emit / debug / info / warning / error compiler-injected call source; filter before render
    │   ├── discarded-count / drain-memory / drain-fallback observable drops, deterministic records, nonrecursive fallback
    │   ├── install-dependency-bridge           explicit `log`/`tracing` event, key-value, and span routing with foreign provenance
    │   └── /core/logging/async::send-event / consume-events existing typed-channel producer and sink consumer; requires `threads`
    ├── /core/urls                             URL and ordered-query model
    │   ├── url                                serialized / display / components / query / origin
    │   ├── url-query                          ordered duplicate-preserving query entries (read-only after parsing)
    │   ├── url-result                         value or failed / message result
    │   └── parse-url / parse-url-relative
    ├── /core/random                           random and digest policy; requires `entropy`
    │   ├── secure-random / pseudo-random      incompatible source types; ChaCha20 is selected explicitly
    │   ├── random-int-result                  failed / message plus bounded integer value
    │   ├── pseudo-bytes / pseudo-bounded-int / split-pseudo
    │   ├── secure-bytes / secure-bounded-int
    │   ├── secret-buffer / destroy-secret     opaque key material with explicit best-effort zeroisation
    │   ├── sha256 / sha512                    distinct digest algorithms
    │   └── digest-bytes / sign-hmac / digest-equals / signature-equals
    ├── /core/codecs                           strict codec policy
    │   ├── decode-result                       failed / message plus decoded bytes
    │   ├── hex / hex-codec                     reusable hexadecimal codec with encode / decode
    │   ├── base64 / base64-url / base64-codec reusable alphabet policy with encode / decode
    │   ├── encode-hex / decode-hex
    │   └── encode-base64 / decode-base64      direct standard or URL-safe operations with explicit padding
    ├── /core/compression                      bounded compression policy
    │   ├── gzip / zlib / deflate-raw / zstd   explicit codecs; no auto-detection
    │   ├── compression-options                level and deterministic-output policy
    │   └── decompression-limits               mandatory output, ratio, and work limits
    ├── /core/random/uuid                      UUID values; requires `entropy`
    │   └── parse-uuid / random-uuid / time-uuid strict canonical parsing plus v4 and v7 generation
    ├── /core/networking                       sockets and DNS; requires `networking`
    │   ├── ip-address / socket-address / network-host-name validated value objects with structured parse results
    │   ├── network-operation-result           explicit failure / deadline / message
    │   ├── network-cancellation-token / network-operation-options shared observable cancellation and positive deadlines
    │   ├── tcp-stream / tcp-listener / udp-socket
    │   ├── ip-address-from-string / socket-address-from-ip / socket-address-from-string / parse-host-name
    │   ├── connect-tcp / connect-host / bind-tcp / bind-udp
    │   └── lookup-dns                         ordered candidates with TTL and explicit failure results
    ├── /core/networking/tls                   TLS; requires `networking` and `tls`
    │   ├── tls-stream                         negotiated-version plus deadline-aware read, write, shutdown, and close
    │   └── connect-tls                        validated TLS 1.3/1.2 client connection; no insecure ordinary option
    ├── /core/concurrency                      synchronization objects; requires `threads`
    │   ├── concurrency-operation-result / concurrency-int-result explicit failure and integer value results for synchronization cells
    │   ├── channel                            typed sender/receiver pair; compile-time capacity; zero-capacity block rendezvous whose accepted handoff completion outranks simultaneous cancellation
    │   ├── channel-block / channel-fail-send / channel-drop-newest / channel-drop-oldest explicit overflow policies; non-block policies require positive capacity
    │   ├── channel-pair / channel-sender / channel-receiver compiler-owned generic linear endpoint families; receiver close returns accepted buffered values
    │   ├── channel-send-outcome / channel-receive-outcome compiler-owned accepted/dropped/closed, rejected/evicted item, and available/value/closed state
    │   ├── mutex / read-write-lock / shared-cell diagnostic-only names directing typed state to owner tasks and channels
    │   ├── int-mutex                          individually synchronized integer load / store / increase cell
    │   ├── int-read-write-lock                integer shared read / exclusive write cell; no exposed guards
    │   ├── memory-order / five order factories typed atomic policy with operation-specific validation
    │   ├── atomic-int64                       load / store / increase with validated memory order
    │   └── thread-local-int                   per-existing-host-thread get / write; shared identity, stale-owner sweep
    ├── namespace                              hierarchical object container
    │   ├── variable                           namespace-local value
    │   ├── constant                           namespace-local or program-global value
    │   ├── function                           callable value
    │   ├── nested namespace                   hierarchical name
    │   └── import                             selected names or namespace binding
    ├── function
    │   ├── invocation mode                    shared / mutable / consuming
    │   ├── parameter                          positional or named
    │   ├── optional parameter                 has a default expression
    │   ├── return type                        declared value type
    │   ├── anonymous function                 value-capturing closure
    │   └── bound method                       receiver captured once
    ├── class
    │   ├── instance field / method             selected with `.`
    │   ├── static field / method               selected with `::`
    │   ├── instance                            explicit construction operation
    │   ├── self / this                         class / instance implicit receivers
    │   └── construct / destruct                compiler-recognized lifecycle methods
    ├── interface                              named nominal contract and dispatch type (`implements`)
    ├── trait                                  reusable source fields and methods (`uses`), not a type
    └── lexical block
        └── binding                            local typed value, ref, or shared ref
```


Version one intentionally has no generic shared mutable cell. Application state belongs to one
owner task, and peers communicate through bounded typed channels. `mutex`, `read-write-lock`, and
`shared-cell` are compiler-owned diagnostic-only names: invoking one emits `T0111` with that
guidance. The integer-specialized synchronization objects below remain implemented low-level
facilities rather than generic type constructors.

## Implemented value types

### `bool`

```text
bool value
├── property
│   └── .type -> bool
├── unary operation
│   └── not bool -> bool
├── logical operations
│   ├── bool and bool -> bool
│   └── bool or bool -> bool
├── equality operations
│   ├── bool == bool -> bool
│   └── bool != bool -> bool
└── descriptor relation
    └── value is a bool -> bool
```

`and` and `or` short-circuit. A descriptor comparison through `.type` uses canonical descriptor identity.

### `int`

`int` is an adaptive, exact signed integer. Its representation may widen, but that representation is not part of the Terrane object surface.

```text
int value
├── property
│   └── .type -> int
├── unary operations
│   ├── -int -> int
│   └── ~int -> int
├── arithmetic
│   ├── int + int -> int
│   ├── int - int -> int
│   ├── int * int -> int
│   ├── int / int -> int      Euclidean quotient
│   └── int % int -> int      Euclidean remainder
├── bitwise and shift operations
│   ├── int & int -> int
│   ├── int | int -> int
│   ├── int ^ int -> int
│   ├── int << integer -> int
│   └── int >> integer -> int
├── comparisons
│   ├── int == int -> bool
│   ├── int != int -> bool
│   ├── int < int -> bool
│   ├── int <= int -> bool
│   ├── int > int -> bool
│   └── int >= int -> bool
├── coercion family
│   ├── .coerce; Destination -> Destination
│   ├── .coerce.checked; Destination -> Destination or none
│   └── .coerce; Destination, function from int to Destination -> Destination
└── descriptor relation
    └── value is an int -> bool
```

For an `int` source, the destination may be any integer or floating-point descriptor. Integer destinations use exact checked conversion. Floating destinations use IEEE round-to-nearest, ties-to-even; a rounded magnitude outside the destination's finite range throws `coercion-error`. `.coerce.checked` returns `none` instead of throwing. `.coerce.wrap` and `.coerce.saturate` require a fixed-width integer source and destination and therefore are not available from `int`.
`int` also exposes the compiler-owned `add`, `subtract`, `multiply`, `divide`,
`remainder`, `div-rem`, `negate`, `shift-left`, and `shift-right` families. Their default
children retain exact adaptive arithmetic; fixed-width-only `wrap`, `saturate`, and
`overflowing` children are absent, while `checked` exists only for genuinely fallible
operations. `div-rem` returns one compiler-owned result containing `.quotient` and
`.remainder`.


### Fixed-width integers

The members below exist uniformly on:

```text
int8, int16, int32, int64, int128,
uint8, uint16, uint32, uint64, uint128
```

```text
fixed-width integer value T
├── property
│   └── .type -> descriptor T
├── unary operations
│   ├── -T -> T               signed types only
│   └── ~T -> T
├── arithmetic
│   ├── T + T -> T
│   ├── T - T -> T
│   ├── T * T -> T
│   ├── T / T -> T
│   └── T % T -> T
├── bitwise and shift operations
│   ├── T & T -> T
│   ├── T | T -> T
│   ├── T ^ T -> T
│   ├── T << integer -> T
│   └── T >> integer -> T
├── comparisons
│   ├── T == T -> bool
│   ├── T != T -> bool
│   ├── T < T -> bool
│   ├── T <= T -> bool
│   ├── T > T -> bool
│   └── T >= T -> bool
├── coercion family
│   ├── .coerce; Destination -> Destination
│   ├── .coerce.checked; Destination -> Destination or none
│   ├── .coerce.wrap; Destination -> Destination
│   ├── .coerce.saturate; Destination -> Destination
│   └── .coerce; Destination, function from T to Destination -> Destination
└── descriptor relation
    └── value is a descriptor T -> bool
```

All integer and floating-point descriptors are valid destinations for the bare and checked policies, except that no floating-to-integer pair is declared. `.coerce.wrap` and `.coerce.saturate` require fixed-width integer sources and destinations and do not accept `int` or a floating destination. The family is compile-time only: a selection must be invoked in the same expression, so `family = value.coerce` is rejected, and the destination must resolve statically to a canonical descriptor. The flat `.checked-coerce`, `.wrapping-coerce`, and `.saturating-coerce` spellings are rejected with a migration diagnostic and no aliases remain. Default integer-to-integer coercion is exact and checked; integer-to-floating coercion rounds to nearest with ties to even and throws `coercion-error` only when the rounded magnitude is outside the destination's finite range. `.coerce.checked` returns `T or none`; `.coerce.wrap` and `.coerce.saturate` return `T`.
The same nine named arithmetic families are implemented on fixed-width integers. Their
`checked`, `wrap`, `saturate`, and `overflowing` children select explicit policies instead
of inheriting Rust build-mode behavior. `overflowing` returns `.value` and `.overflowed`;
`div-rem` computes and returns both results through one backend operation. Postfix `++`
and `--` remain statement-only spellings of the default add/subtract policy.


Declared numeric binding, assignment, parameter-default, argument, and return destinations admit numeric values exactly or fail with `integer-conversion-overflow`. Range-contained fixed-width widening emits only a representation change; other typed numeric pairs retain a runtime representability check. Integer values of different concrete types promote to the smallest implemented integer type containing both source ranges, or to `int`. Local adaptive-`int` bindings proven to remain in `int64` range lower directly to `i64`; conversion to the erased adaptive ABI occurs only where an operation or call requires it.

Any statically typed source value may use bare `.coerce; Destination, converter` when the
compiler declares no conversion for that source/destination pair. The second positional argument
must be one synchronous function from the source's exact static type to the concrete destination;
it is invoked once after the source evaluates once. Named or multiple callbacks, signature
mismatches, asynchronous callbacks, and callback use on a policy child are rejected. A callback's
declared throwables propagate through the ordinary call ABI.

Checked fixed-width integer-to-floating arrivals use allocation-free native magnitude and bit
checks before the primitive conversion. Only adaptive `int` enters the arbitrary-precision
conversion path.

Numeric union bindings retain their declared arms in the semantic model and lower to compiler-owned tagged Rust enums. An exact typed arm wins; otherwise the value must be admitted by exactly one arm. Ambiguous constants are rejected, later assignments are checked against the original arm set, and `is a` inspects the current runtime arm rather than the initializer's selected type. Union destinations are currently implemented only for bindings and their later assignments; parameter and return annotations remain unsupported.

Numeric constant expressions are evaluated in their destination context. Integer destinations use exact unbounded intermediates and check only the final result; floating destinations evaluate at destination precision. This applies to typed bindings and assignments, parameter defaults, declared arguments, and declared returns. A constant used with a typed numeric operand takes that operand's type, except for shift counts.

### Floating-point values

Implemented types are `float32` and `float64`. `float` is the default-precision spelling of `float64` in this compiler version: both resolve to one canonical `float64` descriptor and lower as binary64.

```text
floating-point value T
├── property
│   ├── .type -> descriptor T
│   ├── .finite / .infinite / .not-a-number -> bool
│   └── .negative-sign / .zero / .normal / .subnormal -> bool
├── unary operation
│   └── -T -> T
├── arithmetic and comparisons
│   ├── T + T / T - T / T * T / T / T / T % T -> T
│   └── == / != / < / <= / > / >= -> bool
├── roots and powers
│   ├── .square-root; / .cube-root; -> T
│   ├── .hypotenuse; T -> T
│   ├── .power; T -> T
│   └── .integer-power; int32 -> T
├── exponentials and logarithms
│   ├── .exponential; / .binary-exponential; -> T
│   ├── .exponential-minus-one; -> T
│   ├── .natural-log; / .natural-log-one-plus; -> T
│   ├── .binary-log; / .decimal-log; -> T
│   └── .logarithm; T -> T
├── trigonometry
│   ├── .sine; / .cosine; / .tangent; -> T
│   ├── .sine-cosine; -> tuple of T, length 2
│   ├── .arc-sine; / .arc-cosine; / .arc-tangent; -> T
│   └── .arc-tangent-two; T -> T
├── scalar utilities
│   ├── .absolute; / .fractional-part; -> T
│   ├── .copy-sign; T -> T
│   ├── .minimum; T / .maximum; T -> T
│   ├── .clamp; T, T -> T
│   └── .multiply-add; T, T -> T
├── algorithm utilities
│   ├── .next-up; / .next-down; -> T
│   ├── .decompose; -> float-decomposition of T
│   │   ├── .mantissa -> T
│   │   └── .exponent -> int32
│   └── .scale-binary; int32 -> T
├── integer rounding methods
│   ├── .round; -> int          ties to even
│   ├── .floor; -> int
│   ├── .ceiling; -> int
│   └── .truncate; -> int
├── coercion family
│   ├── .coerce; FloatingDestination -> FloatingDestination
│   └── .coerce.checked; FloatingDestination -> FloatingDestination or none
└── descriptor relation
    └── value is a descriptor T -> bool

floating descriptor T
├── .radix / .significand-digits -> int
├── .epsilon -> T
├── .minimum-positive-normal / .minimum-positive-subnormal -> T
└── .minimum / .maximum -> T
```

Computational members lower directly to Rust primitive operations or to the compiler-owned scalar support used for exact decomposition and binary scaling, so they require no scientific library. Classification and `negative-sign` are properties; zero-argument computations remain methods. Floating results preserve receiver precision. The operations follow the specification's IEEE NaN selection, infinity, signed-zero, domain, overflow, underflow, rounding, accuracy, and target-reproducibility contracts. `sine-cosine` evaluates the receiver once; `multiply-add` is fused; `decompose` and `scale-binary` round-trip normal and subnormal values without avoidable intermediate rounding.

Floating values implement bare and checked coercion to floating destinations. Same-width coercion is identity, `float32` to `float64` is exact, and `float64` to `float32` rounds to nearest with ties to even. A finite source that rounds outside the `float32` finite range throws `coercion-error`, while `.coerce.checked` returns `none`. IEEE infinity and NaN retain their categories across written floating conversion. No floating-to-integer pair is declared on `coerce`; use `round`, `floor`, `ceiling`, or `truncate` to choose the fractional policy before an integer destination.

### `string`

```text
string value
├── properties
│   ├── .length -> int        Unicode extended grapheme-cluster count
│   └── .type -> string
├── views
│   ├── .bytes -> iterable byte sequence
│   ├── .scalars -> iterable list of scalar strings
│   └── .graphemes -> iterable list of grapheme strings
├── transformation and search families
│   ├── .trim[.start|.end]; pattern? -> string
│   ├── .contains[.start|.end]; pattern -> bool
│   ├── .find; pattern -> text-range or none
│   ├── .find.all; pattern -> iterable read-only list of text-range
│   ├── .find.count; pattern -> int
│   ├── .upper[.first|.words]; / .lower[.first]; / .case-fold; -> string
│   ├── .normalise.nfc|nfd|nfkc|nfkd; -> string
│   ├── .split; pattern -> iterable read-only list of string
│   └── .replace; pattern, replacement -> string
├── encoding
│   └── .encode; encoding -> bytes
├── methods
│   ├── .concat; values... -> string
│   └── .join; values... -> string
├── iteration
│   └── for item in string    item is one owned grapheme string
├── comparisons
│   ├── string == string -> bool
│   ├── string != string -> bool
│   ├── string < string -> bool
│   ├── string <= string -> bool
│   ├── string > string -> bool
│   └── string >= string -> bool
└── descriptor relation
    └── value is a string -> bool
```

`.concat` accepts zero or more values, converts each through Terrane's canonical scalar display, and appends them without a separator. `.join` accepts the same values but interleaves the receiver as the separator; an empty call yields the empty string and a singleton call adds no separator. String transformation, search, normalization, and case folding lower through the pinned support runtime. Empty-pattern search, split, and replacement use logical extended-grapheme boundaries: `find.all` includes both ends, `split` returns the graphemes without synthetic empty strings, and `replace` inserts at every boundary. The compiler-owned read-only lists returned by `split` and `find.all` expose `.length` and support `for` iteration; `split` results additionally support indexing.

### `none`

```text
none value
├── property
│   └── .type -> none
└── descriptor relation
    └── value is a none -> bool
```

`none` is also the absent arm of `.coerce.checked`. No other operations on `none` are implemented.

### `bytes`

`bytes` is an implemented sequence value with `b'...'` literals and `.length`. It has no
scalar display. Integer indexing yields `uint8`; range indexing yields new `bytes` while honoring
half-open or inclusive endpoints and authored steps. Invalid selected indices throw
`index-error` with the authored adaptive integer retained in its message. Iteration yields
`uint8` values.

### Collection types

The collection constructors also define applied value types for binding annotations, function
parameters, and function returns:

```text
list of Item
map of Key, Value
set of Item
tuple of Item
unordered-map of Key, Value
unordered-set of Item
entry of Key, Value
```

Collection type application is recursive, so an item or value may itself be an applied collection
type. Map and set keys must be immutable scalar values. A bare constructor name such as `list` is a
value constructor, not a type: every collection type carries its `of` argument or arguments.
Tuples are homogeneous and fixed-length after construction. Their runtime length is not part of
`tuple of Item`, so differently sized tuples with the same item type share binding and function
boundaries.
Iteration takes a value snapshot of its source collection. Mutating or replacing the source binding
inside a `for` does not change the items remaining in that traversal; copy-on-write separates the
mutated value while the iterator retains the original shared storage.
Only lists currently expose `.clear`; map and set mutation intentionally use their documented
member sets rather than inheriting a speculative uniform clear operation.

## Type descriptor objects

Every implemented scalar type has one canonical descriptor object:

```text
bool
int
int8  int16  int32  int64  int128
uint8 uint16 uint32 uint64 uint128
float32 float64 (`float` resolves to `float64`)
string
none
```

Descriptor behavior:

```text
descriptor object D
├── identity
│   ├── D is D -> true
│   └── D is other-D -> false
└── use as a type
    ├── binding annotation
    ├── function parameter annotation
    ├── function return annotation
    ├── integer coercion destination
    └── right operand of `is a`
```

For a scalar value `value`:

```text
value.type is D
value is a D
```

For an ordinary typed scalar, both forms compare its resolved canonical Terrane type with `D`. For a numeric constant, `value is a D` tests whether the constant is exactly admissible by `D`; for a numeric union binding, it tests the current runtime arm. The right-hand descriptor is resolved statically, and an unresolvable name fails with `T0001`. Scalar values themselves are identity-less: `is` between ordinary scalar values is false even when their values and types are equal. Operand expressions are still evaluated for their effects.

Descriptor names remain compile-time identities in ordinary type positions. When reflection or
dynamic descriptor observation requires a value, the compiler materializes the canonical
descriptor object, including for an inline `.type` expression. Source-declared object descriptors
retain namespace-qualified identity. A declared object field or method named `type` takes
precedence over the universal reflection property. Every materialized descriptor exposes
`inherently-identity-bearing`: it is true for reference and resource-owning type contracts and
false for ordinary value types, including collections.

Source-declared class and trait fields with declared scalar, optional, bytes, or collection types
may omit their initializer and receive the type's canonical false, typed-zero, empty, or absent
value. Plain `none`, tuples, source objects, references, callables, resources, and other runtime
contracts are nondefaultable. Every effective class field must receive either its canonical default
or an explicit initializer from the class, a base, or a reused trait; inherited initializers retain
their declaring source/object context. Instance fields also accept one trailing `metadata (...)`
clause. The implemented metadata names are string `external-name` and boolean `secret`; inherited
canonical and explicit defaults plus `T|none` derive the effective field descriptor's `defaulted`
and `optional` flags. Class descriptors expose declaration-ordered field names, external names, and
all three flags as parallel reflected lists. Malformed, duplicate, static-field, non-field, and
externally conflicting metadata is rejected.

## Functions

### Built-in `print`

Canonical object and default-prelude spellings:

```text
/core/output::print
print
```

```text
print; values... -> none
```

- Accepts zero or more arguments whose types implement canonical text display; this is checked
  semantically rather than deferred to generated Rust.
- Converts each argument with canonical text display, concatenates the results without separators,
  and writes one trailing newline.
- Every usable scalar type and `none` implements that display contract. `bytes`, member-family
  objects, and result objects do not.

### Source-declared functions

```terrane
function name ReturnType; required Type, optional Type = default
```

Implemented callable contract:

```text
callable value
├── written invocation mode               function / mutable function / consuming function
├── exact invocation mode                 inferred receiver/capture authority
├── sync or async independently
├── optional throws upper bound and exact escaping set
├── typed parameters and return
├── source function or anonymous closure
├── stored bound method
└── value capture
    └── captures resolver-selected outer bindings once when the closure is created
```

Shared callables observe repeatable state. Mutable callables may update repeatable receiver or
captured state, and separation copies the current environment rather than aliasing it. Consuming
callables may transfer from their environment and are callable once. A body may require less
authority than the written mode but never more; substitution proceeds from shared to mutable to
consuming destinations, never in reverse. `async` and `throws` compose orthogonally, for example
`mutable async function from int to int throws E`.

Function values use these `function from ... to ... [throws T]` annotations and may cross bindings,
parameters, returns, and object-member boundaries. `throws T` is a callable upper bound; nested
function results associate right and each postfix clause binds to its nearest function type.
Anonymous functions use ordinary function syntax without a declaration name. The compiler checks
duplicate, unknown, missing, and excess arguments, rejects positional arguments after named
arguments, and checks callable invocation and throwable compatibility at every typed destination.
Variadic functions, overloads, and generic functions are not implemented.

## Source object and name model

Terrane resolves every bare name through one ordered view:

```text
lexical scope -> namespace -> program-global -> default prelude
```

The first matching name may denote a value, function, canonical descriptor, namespace, or imported entity. There is no leading-dot object form: `.` appears only between a receiver and a member, as in `value.name`. Namespace qualification uses `namespace::name`.

Namespaces form a package-wide tree assembled before reference resolution. Paths use `/` between canonical lowercase segments, with root `/` and parent `..` anchoring. Authored manifests bound sorted recursive source discovery through namespace-root-to-directory mappings, and every discovered declaration is checked against its longest-prefix directory correspondence. Generated Cargo projects live under the package root, and `terrane-build.toml` records the resolved package-relative source set. Direct `.trn` input remains an exempt implicit one-unit package. Selected imports, namespace bindings, visibility, lexical shadowing, program globals, and explicit `global`/`constant` binding rules are implemented.

A top-level plain assignment creates a namespace variable. Functions cannot read or write namespace variables across that boundary; mutable state must cross as an explicit `global`, parameter, or return value. Namespace variables cannot be `public`.

`constant` declarations are non-rebindable at every supported identity tier. In one lexical scope, an ordinary assignment to an already initialized local creates a replacement binding; its initializer sees the earlier binding, and its inferred type may change. Assignment to an uninitialized local, an enclosing-scope binding, a parameter, or a `for` target remains mutation. Generated Rust marks only genuinely mutated storage mutable.

## Classes, interfaces, traits, and references

Classes provide typed, definitely initialized instance fields and methods, class-level static fields
and methods, single inheritance, explicit `instance class; arguments` construction through
`construct`, and one deterministic invocation of each applicable `destruct` hook per independently
owned source value. `.` selects only instance members and `::` only static members. `this` denotes
the current instance in instance methods; late-bound `self` denotes the effective class in instance
and static methods, including inherited static factories using `instance self;`. Instance state is
independent for every constructed value. Static state is shared by one effective class but separate
between a base class and each subclass; nested member writes mutate that shared storage directly.
Return validation recognizes one static-only postcondition used by lazy initialization: an exact
`T|none` static member returned as `T` is proven present only after a preceding sibling absence
guard with no `else`, exactly one direct compatible assignment to that same textual target, no
other branch write, and no intervening write. This does not narrow the member inside the block,
does not apply to instance members, and falls back to the ordinary optional-type diagnostic when
any precondition is absent.
Destruction is ordered from the most-derived class toward the root base. Value separation copies
class and interface-typed state into a fresh lifecycle lineage, while compiler-introduced Rust
clones remain within one lineage and cannot multiply the hook. Subclass values retain inherited and
directly declared state at arbitrary inheritance depth; methods access their flattened storage
directly, while nested base wrappers recursively forward inherited instance-field reads and writes
and overridden methods to the preserved concrete value. Static fields are never forwarded through
instances. Subclasses inherit their bases' declared interface conformance; projected generic calls
and generated foreign implementations use that same complete effective interface closure. Declared
named interfaces state complete method signatures and written invocation modes, require explicit
`implements`, and lower as typed dispatch contracts. Traits reuse source fields and methods through
`uses`, are not type objects or Rust-style contracts, and reject unresolved multi-trait conflicts.

`ref T` values carry compiler-owned whole-path provenance: their originating owner, selected
external-lender parameter, field/element/call-result projections, and first lifetime-ending
mutation, move, or replacement. Parentheses, member access, supported list/map/unordered-map
indexing, uniquely selected lender calls and returns, reference bindings, borrowed collection
iteration, captures, and async liveness preserve or narrow that proof. Lender selection follows
actual return flow through calls to a fixed point; parameter count never chooses it. Returning a
reference is accepted only when it reaches exactly one reference parameter. Local or by-value
parameter return, ambiguous lender flow, unsupported indexed borrow, and post-lifetime-end use are
rejected in source terms.

A provenance-bounded native `ref T` lowers to a Rust borrow, including explicit result lifetimes
for reference-returning functions. Reads do not upgrade, lock, or clone the owner. `shared ref T`
remains explicit synchronized reference-counted ownership; an ordinary observer of that same
explicitly shared identity uses a non-owning weak handle. Prefix `ref`, `shared ref`, and `move`
construct those respective ownership forms, and an ordinary reference cannot be promoted into
shared ownership. Transparent observation does not erase the distinction among `T`, `ref T`, and
`shared ref T` at storage, parameter, or return boundaries.

The native target rejects statically provable strong `shared ref` cycles in descriptor fields,
including through collection element types, while admitting acyclic shared fields. Later cycles
assembled outside that descriptor proof are not traced and must be broken explicitly; ordinary
`ref` back-edges are excluded from ownership edges. References may cross async suspension only
while the owner proof remains complete.

## Callable contracts and reflection

Callable contracts are modelled by the rule each one enforces rather than as permissions from one
generic effect system. The compiler retains a callable declaration's written throwable upper bound
separately from its exact escaping set, and its written invocation mode separately from the exact
receiver/capture authority its body uses. Infallible and narrower implementations satisfy a broader
throwable destination. Shared callables satisfy all invocation destinations, mutable callables
satisfy mutable or consuming destinations, and consuming callables satisfy only consuming
destinations. Omitting `throws` declares an infallible callable type; omitting an invocation prefix
declares shared invocation.

Named functions, closures, bound methods, and inferred aliases keep exact metadata; explicitly typed
bindings and class fields retain both the exact initializer summary for reflection and their written
storage ABI and invocation bound. Source callables never acquire an implicit broad throwable
fallback. ABI selection follows written contracts independently. `async` remains orthogonal and
requires explicit `await`. `awaits`, `mutating`, `mutates`, `unsafe`, and bare `foreign` are not
function qualifiers. Concrete unsafe Rust and foreign interoperability belong to explicit Rust,
runtime, adapter, import, or ABI constructs. Callable reflection exposes retained `.contracts`,
`.throwable-contract`, `.escaping-throwables`, `.invocation-mode`, and
`.exact-invocation-mode`; descriptor values retain canonical identity and `.name`.

I/O and blocking are not source qualifiers, ordinary operations require no compiler-issued
capability value, and manifests do not inject authority into entrypoints. `pure` is not a function
qualifier; no empty generic effect set is presented as a stronger semantic purity guarantee.

## Async tasks and scopes

An `async function` has a distinct callable type and invocation produces a linear task. `await` is
accepted only in an async function and consumes that task; leaving a task unconsumed is a
source diagnostic. The compiler rejects sync/async callable substitutions and non-owning references
whose owner is not proven across suspension.

`select` waits for the first ready operation among two or more statically written cases. Each header
contains exactly one top-level `await`; a case may bind the completed result, and that binding exists
only in its case body. Cases may return unrelated result types:

```terrane
select
  case count int = await (next-count;)
    print; count
  case message = await receiver.receive;
    print; message.value
```

Operations are constructed once in source order. Polling starts at an activation-local rotating
cursor, so simultaneously ready cases alternate fairly across repeated execution of the same
statement. Once a winner is fixed, the cursor advances, losing operations are cancelled and drained
in reverse source order, projected cleanup completes, and only then does the selected body begin.
Selected errors propagate as an ordinary `await` error. Loser cleanup continues after an error; a
later cleanup error replaces an earlier failure. External cancellation and deadlines use the same
drain path, while a request arriving after a winner is fixed cannot interrupt loser cleanup.

All case futures and result slots are stack-local to the generated async state. A plain source-only
selection keeps the dependency-free cooperative runtime; selections involving task scopes,
asynchronous finalization, or projected async work use the already selected native runtime. Channel
waiters are removed when their losing receive/send future is released.


Async callable, task, and scoped-task types retain compiler-owned local-versus-transferable
metadata. Authored callables infer it conservatively from parameters and values live across
suspension; projected Rust async members are currently local because rustdoc alone does not prove
their returned future transferable. The threaded scope rejects local callables, while the
cooperative scope accepts them. Direct invocation and immediate await keep the concrete Rust future
type; generated boxing/pinning remains only at erased callable or task ABI boundaries, with `Send`
present only for transferable futures.

The manifest's two executor profiles map to internal `local` and `parallel` strategies. Semantic
analysis aggregates runtime-context, wake-support, task-mobility, and blocking-delegation
requirements, including explicit requirements on referenced projected async members. Runtime
selection is downstream of this generic model; compiler-owned semantic contracts do not contain
Tokio or another executor crate name.

An asynchronous entrypoint creates one selected wake-driven runtime and tears it down after the
entry task and all linearly owned scopes finish. Projected Rust futures are constructed on first poll
inside that context for Terrane-driven calls, so dependency timers, sockets, and other reactor-backed
futures can suspend without a busy loop. Rust callers of projected interface methods must likewise
poll the returned future on a thread entered into the generated runtime; polling it from a bare Rust
thread is outside the supported boundary. The generated Cargo manifest includes the pinned runtime
dependency only when semantic lowering requires async support. There is no fallback that catches
missing runtime context and blocks instead.
Cancellable legacy scope polling parks on real wakeups with bounded cancellation/deadline
observation; concurrent runtime-native scope scheduling is not implemented yet.

Projected async metadata records runtime-context, wake-support, and transfer knowledge separately.
Rust `async fn` items currently mark wake support `required` and runtime context and transfer
`unknown`; hover and completion expose those Terrane terms. Semantic lowering conservatively keeps
such work local and never treats `unknown` as permission.

`task-scope; deadline?` constructs a scope using the selected threaded or cooperative executor
profile. `.spawn; callable` consumes an async callable invocation into a linear scoped task;
`.join; task` consumes it and returns a task of its outcome. `.child-scope; deadline` creates a child
whose runtime effective deadline is the earlier of parent and requested deadlines; statically
resolvable extension through local aliases and nested constant expressions is rejected. `.cancel;`
records cancellation, and join waits for the selected executor's child operation.
`task-scope.join; child` consumes the statically non-copyable scoped child automatically and returns
an async task for its outcome, so callers write `await scope.join; child` in an async function.
Lowered Rust still passes the task value by ownership; no source-level `move` is needed to express
that compiler-owned representation detail. Native scope children are spawned onto the selected
local or parallel runtime strategy; sibling work can make progress while a join is pending, and a
failed child requests cancellation of its surviving siblings.

The implemented task outcome exposes `completed bool`, `cancelled bool`, `value T or none`, and
`error throwable or none`. Successful completion retains `value` even when cancellation was
requested; failure sets `completed` false, leaves `value` absent, retains the typed child error, and
requests cancellation of surviving siblings. Native scopes wake a suspended child when cancellation
is requested or its deadline expires; they do not poll cancellation on a timer. Observation drops
the in-flight operation, runs active `finally` regions exactly once in innermost-first order, and
shields asynchronous cleanup from the initiating request before join completes. Scoped tasks remain
linear, so every child must be joined before function exit; no implicit detach or abandoned child
path exists.

`/core/streams` `read-async` performs its standard-input host read through the selected runtime's
explicit blocking-delegation path and awaits that delegated operation. The source contract remains
a task of the same read result; the current standard-stream handle is not readiness-native.

TCP connect, listener accept, stream read/write, and UDP send/receive register nonblocking socket
descriptors with the selected runtime and await readiness directly. Host-name connection delegates
only DNS resolution before asynchronously racing socket candidates; standalone DNS lookup remains
explicitly delegated. TLS handshake/read/write/shutdown uses the same readiness-native transport.
A scope may spawn either an async callable or an unpolled task moved into it, permitting
resource-owning arguments to enter a child without borrowing them across suspension. The TCP
loopback conformance witness proves a pending standard-input read and readiness-native socket accept
do not prevent client progress on a single executor worker. Separate outcome evidence covers socket
deadlines and cancellation.

Task runtime support and its Cargo dependencies are selected from semantic lowering metadata, not
from generated source-text searches. Merely spelling a runtime crate path in source text cannot
change the generated manifest.

## Properties and methods index

| Receiver | Member | Kind | Result / effect |
|---|---|---|---|
| any implemented scalar value | `.type` | property | canonical scalar descriptor |
| class descriptor | `.field-count`, `.field-names`, `.field-external-names`, `.field-defaulted`, `.field-optional`, `.field-secret` | properties | declaration-ordered resolved field metadata |
| `string` | `.length` | property | adaptive `int` grapheme count |
| `string` | `.concat; values...` | method | concatenated `string` using canonical display |
| `string` | `.join; values...` | method | canonical displays interleaved with receiver separator |
| any integer | `.coerce; D` | family default | exact coercion or runtime failure |
| any integer | `.coerce.checked; D` | family child | destination value or `none` |
| fixed-width integer | `.coerce.wrap; D` | family child | destination value with wrapping policy |
| fixed-width integer | `.coerce.saturate; D` | family child | destination value with saturation policy |
| `string` | `.parse; callback` | family default | callback's declared return |
| `string` | `.parse.checked; callback` | family child | callback's declared return or `none` |
| `string` | `.radix; base` | method | adaptive `int` interpretation |
| any integer | `.radix; base` | method | lowercase base-N `string` |
| any integer | named arithmetic families | family | explicit default/checked/wrap/saturate/overflowing policy |
| `string` | `.bytes`, `.scalars`, `.graphemes` | properties | explicit text views |
| `string` | `.trim`, `.contains`, `.find`, case/normalization, `.split`, `.replace` | families | Unicode text operations |
| `string` | `.encode; encoding` | method | encoded `bytes` |
| `bytes` | `.length` | property | byte count |
| `bytes` | `.decode; encoding` | method | validated `string` or deterministic decode error |
| `bytes` | `[index]`, `[range]` | lookup/slice | `uint8` or new `bytes`; invalid selected index throws `index-error` |
| collection iterator | `.next` (compiler protocol) | method | typed item or dedicated sticky `end` step, distinct from `none` |
| source-defined iterable | `.iterator`, iterator `.next` | structural protocol | authored typed `iteration-step` state machine with targeted malformed-contract diagnostics |
| list / tuple | `[index]` | lookup | value or `index-error`; `.get.checked; index` returns value or `none` |
| map / unordered map | `[key]` | lookup | value or `missing-key`; `.get.checked; key` returns value or `none` |
| list / map / set / tuple / unordered variants | `.length` | property | adaptive `int` count |
| list | `.append`, `.set`, `.remove`, `.clear`, `.sort`, `.sort.descending` | methods | copy-on-write mutation with observable release points; stable scalar sorting returns the resulting list; `.clear` is intentionally list-only |
| map / unordered map | `.set`, `.remove`, `.remove.checked`, `.keys`, `.values`, `.entries` | methods | deterministic mutation/views; removal returns the stored value, or `none` only for the checked child; no `.clear` in the current surface |
| set / unordered set | `.contains`, `.add`, `.remove` | methods | deterministic membership/mutation; no `.clear` in the current surface |
| entry | `.key`, `.value` | properties | cloned key/value |
| byte reader | `.read`, `.read-exact`, `.read-all`, `.read-async` | methods | partial/exact/bounded/async byte read results |
| byte writer | `.write`, `.write-all`, `.resume`, `.write-async` | methods | partial/complete/resumed/async byte write results |
| byte reader / writer | `.text; encoding` | method | consuming explicitly encoded text adapter |
| text reader | `.read`, `.read-exact`, `.read-all`, `.read-async` | methods | decoded text result or `decode-error` |
| text writer | `.write`, `.write-all`, `.resume`, `.line`, `.write-async` | methods | encoded write result; `.line` alone appends newline |
| writer | `.flush` | method | observable buffering result |
| byte / text writer | `.sync-data`, `.sync-all` | methods | distinct observable durability results |
| any stream | `.close` | method | consuming idempotent release with observable result |

Stream classes become resource-owning transitively from their compiler-owned process handle field.
There is no source `linear class` qualifier; assignment transfers these values automatically.

The compiler represents callable families as bound methods with a distinguished default,
typed children, signatures, and availability constraints. Semantic analysis resolves the
family before lowering; generated Rust erases it to a direct function or support operation.
Family selections must be invoked in the same expression.

The `/core/errors::throwable` interface and compiler-owned standard throwable objects are runtime
identities used by `throw`, `try`, `catch`, and `finally`. Ordinary source-declared classes may
implement `throwable`; a conforming class supplies `message string` and a synchronous, non-throwing,
zero-argument `render string` method, while `cause` is compiler-managed in the runtime envelope.
The class may retain its own additional declared fields. A typed or catch-all `catch ... as name`
binding exposes the common runtime envelope:

```text
throwable value
├── .message -> string
├── .cause -> throwable or none
└── .render; -> string
```

Arithmetic, coercion, decoding, and collection failures enter the same typed result-propagation
path and are catchable. Exact escaping throwable alternatives are inferred transitively after
catches and `finally` replacement. A postfix `throws T` clause is an optional upper-bound contract:
every escaping throwable must implement `T`. Reflection exposes the declared bound separately from
the inferred escaping set. Throwing a caught throwable value preserves its runtime kind and existing
cause chain while adding the explicit rethrow site.

## Projected Rust dependencies

`package.toml` accepts a top-level `rust-toolchain = "pinned" | "system"` selection and
lock-resolved `[rust-dependencies]` entries with version, features, default-feature policy, package
alias, target condition, and declared effects. The default `pinned` selection emits Terrane's exact
stable toolchain; `system` explicitly opts into the caller's active Rust toolchain. An optional
`[profile]` selects an effect allowlist and unwind or abort panic policy; a forbidden dependency
effect is rejected during manifest resolution.

Declared crates are projected from typed rustdoc metadata into reserved
`/deps/<manifest-name>/...` namespaces. Canonical public-path selection prefers substantive paths
over convenience paths beneath `prelude`, then shortest depth and lexical ordering. The shared
projection records verbatim public names, canonical Rust paths, documentation, representable free
functions, associated functions as `Class::function` static members, inherent instance methods,
eligible fixed-signature Rust traits as nominal Terrane interfaces, receiver ownership, opaque
foreign types, data-free enum variant constructors, directly
representable `Result` returns, arbitrary projected `Option<T>` values, all Rust integer widths,
`f32`, `char`, concrete representable type aliases, recursive standard sequence, map, set, and
homogeneous tuple shapes, monomorphic concrete `Fn`, `FnMut`, `FnOnce`, and future-returning
callback bounds, concrete owned asynchronous producers with typed item/end steps, and concrete
owned asynchronous sinks with accepted/close outcomes. Callback metadata maps `Fn`, `FnMut`, and
`FnOnce` to shared, mutable, and consuming invocation modes while retaining retention and
`Send`/`Sync` independently. Generated shims cover free-function and projected-method arguments.
Mutable callback state is repeatable and independently copied at value separation; consuming
callbacks transfer once.
Projected interfaces preserve shared, mutable, and consuming receivers plus required/provided
membership. Local classes adopt them with ordinary `implements`; generated Rust impls delegate
required methods to Terrane bodies. Rust defaults remain callable; Terrane overrides are currently
limited to owned, non-`Result` signatures. Owned concrete generic and `impl Trait` inputs retain
their lifetime, `Send`, and `Sync` bounds and specialize from the written class argument. Only
owning `Box<dyn Interface + Send/Sync>` parameters project: every object auto-trait is retained,
and the shim boxes a conforming concrete class or exact generated wrapper. Bare or borrowed trait
objects, non-interface boxes, multiple principal traits, boxed trait-object results, and wrappers
missing requested auto traits decline. The oracle records closed foreign `Send` and `Sync`
separately; conformance recursively checks the complete effective local-class field graph.
Canonical Rust `Drop` supertraits require Terrane `consuming destruct` and reuse the class's single
lineage-aware `Drop`; direct and reexported canonical `Drop` imports decline with that guidance.
Projected async methods require a `Send` interface and async `main` runtime context. Resource
classes require consuming async receivers. Rust-owned cancellation detaches receiver state, runs
nested `finally`, releases it, and is drained before executor shutdown. Packages without task
scope, projected async entry, or async `finally` retain the small runtime; a sync main with an
otherwise unused async function requires no Tokio runtime. One non-generic associated slot may be
closed explicitly as `Interface of ConcreteType`; the concrete argument is currently limited to a
closed boundary-representable scalar, aggregate, or projected foreign type, with source-class
arguments deferred pending an explicit Rust boundary conversion contract. That applied nominal
identity is retained recursively through permitted aggregate and optional annotations, reflection,
conformance, generic and erased Rust crossings, and generated associated-type declarations.
Grouping places optionality outside an application: `(Interface of ConcreteType)|none`.
Independently projectable supertraits form a recursive, diamond-safe requirement and implementation
closure. Bare or inferred applications,
source-class arguments, multiple or generic associated slots, failed bounds, incoherent erased
bindings, other wrapped receivers, static and generic methods, higher-ranked lifetimes, unsupported
owning containers, and unprojectable members or supertraits remain explicit stable declines.
Async producers and sinks are
resource-owning linear endpoints: borrowed operations must be awaited directly, preserve protocol
failure and task cancellation separately, and reborrow the endpoint for one suspension; consuming
`close` or `split` makes later use of the transferred endpoint a source ownership error.
Projection schema 43 retains these contracts alongside explicit root, continuation, and terminal
lifetime-bearing builders represented as chain-only values. Their intermediates may retain
a borrow from a named input but may appear only as receiver subtrees inside one nested expression;
binding, return, capture, argument
escape, and suspension are rejected before lowering. The terminal must return an owned projectable
value, and tooling marks the root as chain-only and non-escaping. The accepted SQLx witness projects
a concrete borrow-retaining adapter that runs SQLx inside its terminal; open `sqlx::Query` remains
declined rather than being described as directly projected.
Map keys and set items are limited to Terrane scalars. Cross-crate signature types
are admitted only when their canonical owner is declared directly at one lock-resolved version;
otherwise the member remains an explicit decline. Data-carrying enums remain opaque and use
projected crate accessors; every declined public item carries a reason.

Semantic import resolution and the language server consume that same projection. Lowering emits only
crossed-member Rust shims and generated Cargo dependencies; calls remain direct Rust calls inside one
generated crate. A projected Rust `async fn` emits an async shim and constructs a Terrane task whose
awaited result uses the same conversion, error, ownership, and panic boundary as a synchronous
projected call. Concrete Rust callback parameters accept matching Terrane function values; lowering
constructs the required Rust closure, converts its inputs and result, and preserves per-invocation
captured state. Retained or transferable bounds are checked against the callback's receiver,
captures, throwable contract, and async transferability before lowering. Foreign receivers borrow,
use `ref`, or require `move` according to their Rust receiver.
Unwinding dependency panics enter the compiler-owned `dependency-panic` throwable path; abort
profiles omit containment and generate Cargo `panic = "abort"`. Projection and generated-crate
compilation use `bwrap` containment where available and report the host tier otherwise.
`terrane-projection.lock` format 3 retains machine-independent top-level members, instance members
as `Type.member`, static members as `Type::member`, exact injected bound-owner dependencies,
dependency versions, source, rustdoc format, projection schema, exact cache identity, content hash,
and resolution events. A single admitted
concrete generic instantiation keeps its readable Rust type name; hash suffixes are reserved for
multiple admitted instantiations. `S2031` names removed members and their version change; a changed
payload under one exact cache identity is rejected as replay drift. Completion, signature help, and
hover remain advisory; Cargo and rustc are authoritative.

## Major planned surface absent today

The authoritative language draft proposes a much larger ontology. None of the following should be inferred from compiler-owned names or Rust support internals as implemented Terrane API:

```text
collection checked lookup children and source-visible typed lookup errors
reflection inventories beyond retained field metadata, callable contracts, throwable alternatives, and canonical descriptor identity
bytes indexing and slicing
user-authored implementations of general iteration protocols
user-declared type parameters and generic application
typed task errors, defined cancellation points, automatic sibling cancellation, and explicit detach
function/class/namespace/type reflection objects
```

This separation is intentional: executable conformance defines the current compiler contract, while the full specification describes the planned language.