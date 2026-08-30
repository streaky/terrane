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

`check` prepares each selected implementation and runs the small correctness profile. `benchmark` and `report` preserve adapter-declared build caches by default, complete all lane setup before running a case, recheck correctness, perform warm-ups, and then record end-to-end program runs. Compilation and preparation time is never included in an execution result; a lane with no build step, such as Python, follows the same timing boundary. Cases are interleaved in suite order within each warm-up or measured run index, rather than finishing all repetitions for one lane before starting another. Pass `--cold-builds` to clear only adapter-declared caches inside this suite and record separate cold and immediately repeated incremental preparation measurements. `benchmark` emits JSON. `report` writes a timestamped Markdown report and a same-named complete JSON record under `reports/` by default; `--output path/report.md` chooses another pair of paths. Neither command removes the repository's Cargo target directory.

`lower` refreshes each lane's declared inspectable lowering. The Terrane lane writes
`main.lowered.rs` beside every `main.trn`, keeping the generated Rust receipt with the solution.

On Linux systems with delegated cgroup-v2 memory accounting, every launched program and its descendants run in a fresh cgroup. Reports record that cgroup's `memory.peak`: total peak memory charged to the group, including anonymous memory, charged page cache, and kernel memory, with shared pages charged once. It is deliberately labelled peak memory rather than RSS. When this accounting is unavailable, memory results remain unavailable rather than falling back to a misleading process estimate.

The Terrane adapter builds both the compiler and every generated benchmark executable with Cargo's optimized release profile. Development and release artifacts are cached separately by the Terrane CLI. Reports capture the machine platform, kernel, CPU model, core counts, memory capacity, runner Python version, and each lane's configured tool versions. Successful stderr is retained in JSON and warning-like lines are counted and surfaced in Markdown.

## Published evidence

The initial two-lane baseline is
[`initial-two-lane-baseline.md`](reports/initial-two-lane-baseline.md), with its complete process
records and frozen workload profiles in the adjacent
[JSON report](reports/initial-two-lane-baseline.json). It was produced with the suite's default two
warm-ups and seven measured executions per problem and lane.

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

Each implementation prints exactly one finite decimal integer or floating-point result. The runner parses it according to the problem's shared result kind and applies the shared exact or tolerance-based correctness contract. Floating expected values are mathematical references rather than fingerprints of one accumulation order. Their absolute tolerances are forward-error bounds for the least accurate admitted implementation; as in `math.isclose`, a result passes when either its absolute or relative tolerance holds.

## Add a language lane

Add one TOML file under `lanes/` and index it from `suite.toml`. The adapter declares reusable command templates and the implementation path relative to every problem:

```toml
id = "example"
name = "Example language"
implementation = "example/main.ext"
setup = ["examplec", "--version"]                  # optional, once per report
prepare = ["examplec", "$implementation", "-o", "$implementation_dir/program"]
lower = ["examplec", "--emit-readable", "$implementation"] # optional inspectable lowering
lower-output = "$implementation_dir/main.lowered.ext"
prepare-output = "none"                            # or "executable-path"
run = ["$implementation_dir/program"]
cache-paths = ["$implementation_dir/cache"]        # optional, suite-local only
environment = [["examplec", "--version"]]          # optional report metadata

[metadata]
implementation = "Example compiler 1.x"
```

Available placeholders are `$repo`, `$suite`, `$problem`, `$implementation`, `$implementation_dir`, and, after preparation, `$prepared`. Use `${name}` where a placeholder touches adjacent text. Commands are argument arrays, not shell strings; literal braces require no escaping. Adapters contain no problem-specific commands or expected values, and adding one does not change `run.py`.

The next lanes should be Python with NumPy/SciPy or the relevant scientific package, idiomatic maintainable Rust, and Terrane using a Rust scientific crate, in that order. C, Java, Julia, and specialised environments can use the same adapter boundary when they provide a useful comparison.
