# Scientific mathematics and data benchmarks

This corpus compares clear implementations of the same scientific or data problem. Its first two lanes are ordinary Terrane and ordinary Python without NumPy, SciPy, native extensions, or embedded foreign-language kernels.

The corpus is design and performance evidence, not compiler conformance. Every implementation must pass its problem's correctness profile before the runner records performance measurements.

## Run it

Python 3.11 or newer is required for the standard-library TOML reader. The Terrane lane also uses the repository's Rust toolchain.

```console
python3 benchmarks/sci-maths/run.py list
python3 benchmarks/sci-maths/run.py lower
python3 benchmarks/sci-maths/run.py check
python3 benchmarks/sci-maths/run.py benchmark --runs 7 --warmups 2 \
  --output benchmarks/sci-maths/results/local.json
python3 benchmarks/sci-maths/run.py report
python3 benchmarks/sci-maths/run.py report --cold-builds
```

Global selectors go before the command and may be repeated:

```console
python3 benchmarks/sci-maths/run.py \
  --problem scalar-reduction --lane python check
```

`check` prepares each selected implementation and runs the small correctness profile. `benchmark` and `report` preserve adapter-declared build caches by default, complete all setup and preparation before timing any program execution, recheck correctness, perform warm-ups, and then record end-to-end program runs. Compilation and preparation time is never included in an execution result; a lane with no build step, such as Python, follows the same timing boundary. Pass `--cold-builds` to clear only adapter-declared caches inside this suite and record separate cold and immediately repeated incremental preparation measurements. `benchmark` emits JSON; `report` runs every selected problem and lane and writes a timestamped Markdown report with a same-named complete JSON file under `results/`. Neither command removes the repository's Cargo target directory.

`lower` refreshes each lane's declared inspectable lowering. The Terrane lane writes
`main.lowered.rs` beside every `main.trn`, keeping the generated Rust receipt with the solution.

On Linux, each process measurement includes best-effort sampling of the summed resident set size of the process and descendants. The sampler waits 5 ms between scans, so the effective cadence also includes the time needed to inspect `/proc`. `--memory-trace` retains the samples as well as the peak. Sampling can miss short-lived peaks; RSS includes runtime state, shared mappings, and allocator-retained pages. The JSON report records this limitation rather than presenting the values as exact allocation counts.

The Terrane adapter builds both the compiler and every generated benchmark executable with Cargo's optimized release profile. Development and release artifacts are cached separately by the Terrane CLI.

## Corpus shape

`suite.toml` is the ordered index of lane adapters and problems. A problem owns:

- a declarative `problem.toml` with dataset construction, correctness results, tolerances, and correctness/performance sizes;
- one implementation directory per participating lane;
- no stored large dataset or language-specific correctness policy.

The initial problems cover:

1. scalar reduction;
2. materialized element-wise transformation;
3. fused transformation and reduction;
4. branch-heavy irregular iteration;
5. composed generation, moment calculation, and classification.

Inputs are generated deterministically inside each process from the formula and size in `problem.toml`. Data preparation therefore belongs to the reported end-to-end time and memory. The correctness profile is deliberately small enough to diagnose; the performance profile is reproducible without checked-in bulk data.

Each implementation prints exactly one finite decimal integer or floating-point result. The runner parses it according to the problem's shared result kind and applies the shared exact or tolerance-based correctness contract.

## Add a language lane

Add one TOML file under `lanes/` and index it from `suite.toml`. The adapter declares reusable command templates and the implementation path relative to every problem:

```toml
id = "example"
name = "Example language"
implementation = "example/main.ext"
setup = ["examplec", "--version"]                 # optional, once per run
prepare = ["examplec", "{implementation}", "-o", "{problem}/example/program"]
lower = ["examplec", "--emit-readable", "{implementation}"] # optional inspectable lowering
lower-output = "{implementation_dir}/main.lowered.ext"
prepare-output = "none"                            # or "executable-path"
run = ["{problem}/example/program"]
cache-paths = ["{problem}/example/cache"]          # optional, suite-local only

[metadata]
implementation = "Example compiler 1.x"
```

Available placeholders are `{repo}`, `{suite}`, `{problem}`, `{implementation}`, `{implementation_dir}`, and, after preparation, `{prepared}`. Commands are argument arrays, not shell strings. Adapters contain no problem-specific commands or expected values, and adding one does not change `run.py`.

The next lanes should be Python with NumPy/SciPy or the relevant scientific package, idiomatic maintainable Rust, and Terrane using a Rust scientific crate, in that order. C, Java, Julia, and specialised environments can use the same adapter boundary when they provide a useful comparison.
