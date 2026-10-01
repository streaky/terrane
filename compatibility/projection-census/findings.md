# Initial projection census findings

## Scope and confidence

The Linux x86-64 baseline assessed all 17 pinned configurations on 24 September 2026. Twelve are high priority, one is mid priority, and four are deliberately difficult low-priority packages. Every Cargo graph and static compiler assessment completed. A small Strsim consumer was admitted through checking and lowering. A direct Iced `application` import reproduced the compiler's `BootFn` contextual-callback decline; the existing GUI application remains context-only because its maintained Rust renderer would make a successful check invalid evidence for direct Iced admission.

This is a surface census, not a package-usability percentage. Native declarations and compiler projection operations have different units, reexports retain multiple public paths, and a single missing compositional capability can block a workflow spanning many admitted operations. The baseline therefore preserves the separate denominators and does not divide 4,586 projected operations by 9,921 discovered declarations. Generic and composition-sensitive conclusions remain provisional until more representative consumers are added.

## Cohort result

| Cohort | Configurations | Projectable declarations | Unsupported declarations | Deferred contextual | Namespace paths | Missing metadata | Projected operations | Declined operations |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| High priority | 12 | 256 | 404 | 17 | 3 | 0 | 3,204 | 2,371 |
| Mid priority | 1 | 31 | 9 | 0 | 0 | 0 | 331 | 64 |
| Low priority | 4 | 578 | 8,604 | 18 | 1 | 0 | 1,051 | 9,062 |
| Total | 17 | 865 | 9,017 | 35 | 4 | 0 | 4,586 | 11,497 |

The runner made 39 additional exact-package surveys for public declarations owned by resolved closure packages. In particular, `axum::body::HttpBody` carries the signature and package-relative source location recovered from `http-body@1.1.0`; its compiler result is assessed through the Axum facade rather than left unclassified. External enum variants are recovered from their surveyed parent declaration, covering the eight Iced alignment and length variants without inventing standalone declarations.

Pinned sysroot evidence completes the remaining selected surface. Sixteen requested `core` definitions were generated from the exact toolchain's `rust-src`, including `Infallible`, `ErrorKind`, and the C primitive aliases. Three `std` IO paths retain exact canonical identity and item kind from the selected package's pinned Rustdoc summary where a standalone owner definition is unavailable. Rustdoc-confirmed namespace-only reexports, including `linux_raw_sys::ctypes`, are recorded separately because they have no operation or type signature. No selected public path remains classified as missing metadata. Recursive closure-owner failures are still retained in the full report, but they do not leave any selected facade declaration unexplained.

Recursive owner support uses the complete generated `terrane-dependencies.lock` graph without
flattening it into Terrane source imports. Projection injected 21 exact private Cargo edges and
removed 266 of the 272 operation declines previously caused solely by an unreachable recursive
crate. Reqwest, Axum, H2 and Bindgen retain no such decline. The remaining six Iced operations
refer to `bitflags` while the lock contains both 1.3.2 and 2.13.2; they remain explicitly declined
rather than guessing an owner identity. The operation-level change adds 242 projected operations
and removes 246 declines overall because normalization and method grouping do not preserve a
one-for-one row count.

The low-priority system and runtime cohort accounts for most unsupported declarations. It must not be used to claim that ordinary high-priority packages need the same solution as libc, Linux raw bindings, Serde, or Tokio. Conversely, 404 explicitly declined high-priority declarations and two exercised representative consumers—one admitted and one declined—are not evidence that the remaining focused fixes alone are sufficient.

## Representative structural traces

The full report retains every native signature, source location, projection object, declined path, and explanation. Representative traces through the current discovery-to-admission boundary include:

- unsupported native item kinds: `hashbrown` discovery reaches `hashbrown`, which projection declines as an item kind with no Terrane projection;
- unsupported method kinds: `hashbrown::DefaultHashBuilder::Hasher` is retained by discovery and declined because its item kind has no Terrane method projection;
- lifetime-bearing boundaries: `hashbrown::DefaultHashBuilder::fmt` is declined because a lifetime-bearing foreign type cannot cross a projected boundary;
- non-escaping regions: `hashbrown::hash_map::Drain` is declined because lifetime parameter `'a` requires non-escaping chain projection;
- unstable or anonymous native identities: `hashbrown::DefaultHasher::write` is declined because a referenced type has no stable Rust path;
- unresolved generic interfaces: `iced::Border::color` is declined because a generic input retains an unresolved interface bound; and
- unsafe host surface: `iced::wgpu::Adapter::as_hal` is separately declined as an unsafe function.

Missing metadata is distinct from rejection. Public external owners are followed automatically when one exact resolved package matches, variants use their surveyed parent definition, and `core`/`std` use pinned-toolchain evidence. Absent owners, multiple resolved versions, failed owner surveys, and declarations still absent after all evidence paths remain explicit report data rather than being guessed or silently dropped; none affect the selected surface in this baseline.

## Decision and next work units

The first evidence does not justify immediately replacing the projection architecture. It identifies several composable ordinary-language gaps—non-escaping lifetime regions, generic interface resolution, and stable identity across associated and anonymous types—alongside intentionally separate unsafe/system surfaces. Unique recursive package owners now remain privately nameable through their declared facade; multiple-version owners stay an explicit identity decision. Raw unsupported-item volume is heavily cohort-sensitive. The architectural simplification gate should therefore remain open while the next work narrows workflow impact rather than selecting a rewrite from aggregate counts.

Recommended next work units are:

1. Add small valid consumers for Iced callback and widget composition, Hashbrown generic map use, HTTP/H2 request flow, Axum routing, Reqwest request construction, and Tokio stream/task boundaries. Each consumer should select concrete types from available evidence and record exactly which operations it exercises.
2. Split unsupported item-kind evidence by native kind and by whether it blocks a representative workflow. Constants, modules, macros, unsafe operations, constructors, associated types, and callable operations must not remain one decision category.
3. Trace the lifetime-region, generic-interface, stable-identity, and multiple-version-owner clusters end to end through discovery, projection admission, source semantics, and lowering. Compare where native facts are duplicated or lost before choosing focused extensions or consolidation.
4. Keep libc, linux-raw-sys, Serde, Tokio, and the Iced unsafe host surface as separate cohorts. Ordinary projection conclusions should be reported independently from system ABI, runtime replacement, serialization-policy, and unsafe host-integration decisions.
5. Add a newly selected ordinary package after those traces, without a compiler patch or prior package recipe, to test whether the resulting explanation and representative-use workflow generalize.

These are recommendations for the architectural decision gate, not implementation of later Milestone 31 mechanisms.
