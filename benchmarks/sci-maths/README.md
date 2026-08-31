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

On Linux systems with delegated cgroup-v2 memory accounting, every launched program and its descendants run in a fresh cgroup. Reports record that cgroup's `memory.peak`: total peak memory charged to the group, including anonymous memory, charged page cache, and kernel memory. Shared pages are charged once, potentially to a cgroup outside the measured execution, so small-footprint results depend on page-cache state and are not directly comparable across machines or reboots. Reports summarize the median and range across measured runs and deliberately label the metric peak memory rather than RSS. When this accounting is unavailable, memory results remain unavailable rather than falling back to a misleading process estimate.

The Terrane adapter builds both the compiler and every generated benchmark executable with Cargo's optimized release profile. Development and release artifacts are cached separately by the Terrane CLI. Reports capture the machine platform, kernel, CPU model, core counts, memory capacity, CPU frequency governor, start-of-run load average, runner Python version, relevant inherited Python/Rust/Cargo environment variables, and each lane's configured tool versions. The runner records relevant inherited variables rather than publishing the complete environment, which can contain secrets. Successful stderr is retained in JSON and warning-like lines are counted and surfaced in Markdown.

## Published evidence

The initial two-lane baseline is
[`initial-two-lane-baseline.md`](reports/initial-two-lane-baseline.md), with its complete process
records and frozen workload profiles in the adjacent
[JSON report](reports/initial-two-lane-baseline.json). It was produced with the suite's default two
warm-ups and seven measured executions per problem and lane.

## What currently limits the Terrane lane

The Terrane lane's results are bounded by generated-code characteristics rather than by the CPU
being idle. Every measurement below is `perf stat` against the release binary each problem already
builds, at its `problem.toml` performance size.

| Problem | Work unit | Instructions | Per unit | IPC |
|---|---|---|---|---|
| scalar reduction | 50,000,000 elements | 2.850 G | 57 | 5.15 |
| branch-heavy Collatz | 131,434,424 loop iterations | 10.050 G | 76 | 4.16 |
| fused transform and reduction | 20,000,000 elements | 8.376 G | 419 | 3.04 |
| materialized element-wise | 10,000,000 elements | 4.506 G | 451 | 2.44 |
| composed moments and classification | 10,000,000 elements | 8.430 G | 843 | 2.32 |

The last row makes several passes over its data, so its per-element figure is not comparable with
the others.

Two observations frame the rest. The processor is saturated: the Collatz binary reports 99.2% CPU
and a 0.84% branch-miss rate, and instructions-per-cycle across the corpus ranges from 2.3 to 5.15,
which is a large fraction of the machine's issue width. And the work being retired is far larger
than the arithmetic requires — a Collatz step is roughly ten machine instructions written directly
in Rust, and the lane executes about 76. The lane is therefore not slow because the generated code
stalls; it is slow because it executes several times more instructions than the equivalent
hand-written Rust would.

Three mechanisms are visible in the disassembly of the built binaries.

**Fixed-width arithmetic helpers are not inlined.** `terrane_int_support::fixed_remainder`,
`fixed_division`, and their siblings carry no `#[inline]` attribute, and each appears in the hot
loop as an out-of-line call. In the Collatz inner loop:

```text
mov  $0x2,%edx            ; divisor
lea  0x90(%rsp),%rdi      ; return buffer
mov  %r12,%rsi            ; value
call *0x64eae(%rip)       ; fixed_remainder
mov  0x90(%rsp),%rcx      ; read the result back
mov  0x98(%rsp),%rax
mov  0xa0(%rsp),%rbx
```

Written directly in Rust, `value % 2` against a constant divisor is a single instruction.

**The support crates' arithmetic error type is large, so every checked operation returns through
memory.** `ArithmeticError` is 72 bytes, because its `IntegerConversionOverflowDetail` variant
carries an owned `String` and three `&'static str`. `Result<i64, ArithmeticError>` is therefore also
72 bytes and cannot be returned in registers, which is the return buffer visible above. The
cheapest and most frequent operation in the language pays the representation cost of its rarest
error variant. The size also enlarges each helper's body, which is part of why the inliner declines
them across the crate boundary.

**Integer-to-float conversion routes through the adaptive integer type.** In the fused problem,
`(index % 1000)` reaching a `float64` binding lowers to a call to
`terrane_int_support::exact_f64` taking a pointer to an adaptive `Int`, rather than the single
`cvtsi2sd` a direct `i64` to `f64` conversion compiles to. The surrounding floating-point
arithmetic is clean and inline; the conversion at the edge is not.

None of these is a property of the language's semantics. Checked fixed-width arithmetic is a
deliberate guarantee, and honouring it costs an add and a not-taken branch — in the Collatz loop
that check compiles to `imul`/`jo` and `inc`/`jo`, which the branch predictor handles essentially
for free. What the measurements show is the cost of how the guarantee is currently *packaged*: an
out-of-line call, a 72-byte memory round trip, and a conversion through a wider representation than
the operands need.

These limits have not been addressed, and no work in the corpus depends on them being addressed.
They are recorded here so that lane results are read as a measurement of the current lowering and
support crates rather than as a bound on the approach. Anyone re-measuring after changing them
should control code alignment: layout luck alone has produced reproducible 19% swings between
binaries whose hot loops were instruction-for-instruction identical, in both directions, and
repeating a run cannot detect it because alignment is a property of the binary rather than of the
execution.

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

Each implementation prints exactly one finite decimal integer or floating-point result. The runner parses it according to the problem's shared result kind and applies the shared exact or tolerance-based correctness contract. Floating expected values are mathematical references rather than fingerprints of one accumulation order. Their absolute tolerances use the standard binary64 forward-error bound with unit roundoff `u = 2^-53`, plus an explicit 2x margin for conservative decimal rounding; as in `math.isclose`, a result passes when either its absolute or relative tolerance holds. The fused performance tolerance is about 1.3e-8 relative to its expected result: wide enough for realistic accumulation-order rounding, but far too narrow to conceal an algorithmic discrepancy.

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
