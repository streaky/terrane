# Application projection surface IR

This directory documents the deliberately broad YAML intermediate representation in the adjacent `application-surfaces.yaml`. It answers a superset-first question: what native surface, resolved graph evidence, and current Terrane presentation would we want in front of us if the surveyed package sets described one application?

The single document has `high-mid` and `high-mid-low` profile views. The latter adds the deliberately difficult low-priority roots. Package surfaces are stored once and profile records reference the exact package identities they include. Each profile connects three layers:

- `root-projections` retains every declared root configuration, representative workflow, declaration assessment, projected item, declined item, private recursive-owner edge, and compiler explanation from the real projection census;
- `resolved-graph` is the union of exact package identities visible from those roots, with source, manifest identity, active features, exact dependency edges, and every root context in which each package appeared; and
- shared `package-surfaces` surveys every resolved library package and retains its complete rendered Rust public API, structured declaration/signature records, public paths, source locations, and explicit discovery failures.

The rendered API comes from the `public-api` crate over the same pinned Rustdoc JSON used by Terrane. Blanket implementations, auto-trait implementations, auto-derived implementations, parameter names, associated items, trait implementations, reexports, constants, statics, macros, types, traits, functions, methods, and modules are retained. The IR stores rendered items as strings rather than repeating unstable Rustdoc IDs. This intentionally favors completeness and reviewability over a prematurely exact Terrane semantic model. Structured declarations and raw compiler projection evidence remain alongside the rendered API so later schemas can split or reinterpret entries without repeating ecosystem discovery.

Regenerate the document from a completed projection census with `python tools/generate-projection-surface-ir.py`. Use `--profile high-mid` or `--profile high-mid-low` for a narrower one-profile document, `--output PATH` to select its destination, and `--check` to validate that local output is byte-for-byte current. Generated application-surface YAML is intentionally ignored because complete inventories are very large; this schema guide and the generator remain committed. Full per-package survey intermediates stay under ignored `target/projection-surface-ir/`. Output ordering, profile selection, graph merging, shared-surface references, and YAML alias suppression are deterministic.


The format is versioned but intentionally provisional. Consumers should key on `schema` and `kind`, preserve unknown fields, and treat rendered Rust API text as evidence rather than a stable parser protocol. Future revisions can add normalized callable/type semantics while retaining these complete source facts as the migration oracle.
