# Reference sync

This Terrane tool lives with the compiler, but edits a separately supplied manual checkout. Use a current `terrane` on `PATH` that supports `reference-snapshot`.

## Export and preview

Run from the compiler checkout. The example source already imports `/core/codecs`, so the compiler loads the bundled declarations:

```sh
terrane run tools/reference-sync -- export \
  --input tests/conformance/run/grouped-imports/case.trn --scope /core/codecs \
  --snapshot codecs.json --view codecs-projection.trn
terrane run tools/reference-sync -- sync --snapshot codecs.json --manual-root manual
```

`export` writes a versioned semantic snapshot and a Terrane declaration view of the input package's native dependency projection. The latter is for inspection, not a parser input to `sync`. A namespace scope exports what is loaded and resolved by the input; it is not evidence that a symbol missing from the output has been removed. Unsupported declarations are identified under `omissions`. The projection view can contain more than the declarations loaded by semantic analysis for the selected namespace.

`sync` previews updates and additions. Pass `--write` to apply them. Existing YAML records retain their authored bytes outside `surface` and `provenance.compiler`; new records remain unlisted with `documentation.status: missing` and `lifecycle.status: planned`. Review a scaffold before adding it to navigation. The YAML splice supports the manual's conventional block layout and rejects unrecognized layouts rather than reserializing the whole record. The manual-wide catalogue refresh remains separate.

## Release removal tracking

For a supported manifest package, export both revisions with `--scope all`, using their respective compilers and locked dependencies. Preserve the prior JSON as a release artifact. `all` covers production declarations in the selected manifest package; it is not a completeness claim for bundled core, projected Rust dependencies, or build-time generated APIs. Snapshots with unsupported public declarations or consumer output do not qualify as complete. The snapshot profile identifies the package, coverage, host and capability profile independently of release version.

```sh
terrane run tools/reference-sync -- sync --snapshot current.json --manual-root manual \
  --release 0.2.0 --previous previous.json
```

Review the preview, then add `--write`. Only a `current` or `deprecated` API present in the comparable prior snapshot can be marked `removed`; it gets `removed-in` while keeping its last signature and authored text. Explicit renames in `reference/lifecycle/renames.yaml` are reported for review instead of being treated as removal. No records are deleted or automatically added to publication navigation.
