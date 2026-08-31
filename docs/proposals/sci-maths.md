# Scientific Mathematics and Data Workloads

**Note:** This proposal was the basis for the work done in `benchmarks/sci-maths` - it is moved to the repo because there's still work needs doing to complete it and it isn't very portable just on my pc :)

## Purpose

Scientific mathematics and data processing should be an important design pressure for Terrane.
Python is successful in this area because NumPy, SciPy, and the surrounding ecosystem make a wide
range of established native implementations accessible from a comfortable language. Rust has an
increasingly capable scientific ecosystem too, but using it directly still requires developers to
make and maintain representation, ownership, specialisation, and performance decisions throughout
their application code.

Terrane has a credible opportunity to combine a comfortable high-level source language with light,
specialised generated Rust. The compiler can make data-layout and execution choices that would be
possible in handwritten Rust but undesirable to maintain there, while leaving the lowered form
available for inspection. Even before substantial optimisation work, this may allow Terrane to
compete in workloads where Python-level loops, intermediate arrays, repeated native-library
crossings, or an awkward vectorised formulation impose meaningful costs.

We should build a long-lived corpus of scientific and data problems to test that proposition. It
should help shape the language and compiler, measure progress honestly, and expose regressions. It
must not assume in advance either that Terrane will win or that established implementations are an
unreachable ceiling.

## Questions the Corpus Should Answer

The corpus should begin with a simple question: how do clear, idiomatic Terrane and clear, idiomatic
Python behave when they directly express the same scientific or data problem?

As it grows, it should also help investigate:

- whether Terrane implementations are correct and numerically credible;
- where lowering, representation selection, or specialisation gives Terrane an advantage;
- where another language or implementation remains ahead, and why;
- how compilation, allocation, data movement, temporary values, dispatch, and execution contribute
  to a result;
- whether Terrane can use scientific Rust crates without losing their correctness or performance
  characteristics at the language boundary;
- how closely generated Rust approaches credible handwritten Rust;
- which missing language or compiler capabilities would make the greatest practical difference.

The aim is not a single league table. A result is useful when it can be explained.

## Comparison Shape

The initial comparison should have only two lanes:

1. **Clean, idiomatic Terrane**, written as a Terrane user should reasonably want to write it.
2. **Clean, idiomatic Python**, using the language directly rather than escaping to NumPy, SciPy, or
   another native implementation.

The first corpus should keep the algorithm and broad representation comparable without forcing
either language into an unnatural line-for-line translation. This gives us a narrow, legible
starting point for the kinds of loops, branching, transformations, and data handling that users
would otherwise write in Python itself.

The next comparison lanes should definitely be:

3. **Python using NumPy, SciPy, or another relevant scientific package**, representing the
   established high-level scientific experience with tuned native implementations.
4. **Idiomatic, maintainable Rust**, representing code a Rust project might realistically keep and
   evolve.
5. **Terrane using a Rust scientific crate**, where delegation is the natural formulation.

Other languages and environments, including C, Java, Julia, or specialised tools, should be easy
to add when they provide a useful control or comparison.

Direct Rust will show how closely Terrane approaches its lowered environment and help separate
compiler costs from the underlying algorithm or library. More aggressively hand-tuned
implementations may occasionally provide context, but should not displace maintainable code as the
main baseline: generated specialisation that would be unpleasant to maintain by hand is part of
Terrane's value proposition.

Every result must state whether it compares equivalent algorithms or the best credible formulation
available in each environment.

## Initial Scope

The first corpus should contain a small set of problems that both current Terrane and ordinary
Python can express clearly and that exercise meaningfully different behaviour:

- a scalar or collection reduction;
- an element-wise transformation;
- a fused transformation or aggregation where intermediate data may matter;
- a branch-heavy or irregular loop;
- a small composed numerical or data-processing workload.

Each problem should have a small correctness input and at least one realistic performance size.
The corpus can expand into crate-backed implementations and broader scientific fields as Terrane's
implemented capabilities grow. Future problems may be retained as design pressure, but must not be
presented as evidence of current support.

## Correctness

Correctness evaluation and performance measurement are related but distinct. Every benchmarked
implementation must first satisfy a declared correctness contract appropriate to the problem.

Those contracts may use analytic answers, independently established reference results, invariants,
cross-implementation agreement, or a combination of them. Floating-point comparisons must state
their tolerances and account for the numerical properties of the problem rather than relying on
unqualified equality. Edge conditions, invalid inputs, convergence failure, non-finite values, and
numerically difficult inputs should be included where they are meaningful.

Correctness work here does not replace the compiler conformance corpus. Minimal conformance cases
prove language contracts and rejected boundaries; this corpus proves that larger scientific
programs behave credibly end to end.

## Performance

Performance reporting should separate at least:

- compiler checking and lowering;
- cold and incremental native builds;
- first execution or first library call where initialisation is material;
- warmed steady-state execution;
- end-to-end workload time, including data preparation or transfer when that cost belongs to the
  user-visible operation;
- kernel-only execution where isolating the implementation is useful;
- peak and, where practical, changing memory use during both end-to-end and isolated execution.

Warm-up must be deliberate rather than used to hide costs. Each result should make clear what was
prepared before timing and what occurred inside the measured region. Runs should report enough
environment and dependency information to reproduce and interpret them, and should show variation
rather than only a single best number.

Memory should be treated as a first-class result rather than inferred from runtime. The project
should seek a language-neutral process-level measure that works across all lanes, while allowing
language-specific adapters to report useful additional information such as allocated bytes or
allocation counts. Reports must distinguish input data from additional working memory where that
can be established, and make clear whether startup state, child processes, mapped files, allocator
retention, or library-managed pools are included. Perfectly equivalent accounting may not be
available across every runtime, so the method and its limitations must accompany the result.

The suite should avoid prescribing optimisation techniques in advance. Its role is to reveal
behaviour and give compiler work a stable target. When Terrane performs differently, the generated
Rust and relevant runtime evidence should be available so the result can guide implementation
rather than encourage benchmark-specific source contortions.

## Test Data

Implementations of a problem must operate on the same inputs. Test data should be generated
programmatically and reproducibly so the corpus can cover ordinary cases, edge cases, and large
performance workloads without storing bulky datasets in Git.

Generated datasets may be cached and ignored by Git, but they must not become irreplaceable local
state. Their generation parameters, random seeds, format, and identity must be stable enough for
another machine to recreate the same inputs. Small, important correctness examples may be stored
directly when that makes review and diagnosis clearer.

## Language Support

The benchmark project should be language-neutral. Adding a language should require a small,
reusable adapter describing only how to prepare and run an implementation and how the harness
receives its result. A compact declarative file, such as YAML, may be a suitable shape, but the
exact format should follow from the first implementations rather than being designed in advance.

Problem definitions, datasets, correctness criteria, measurement policy, and result reporting
should belong to the corpus rather than being duplicated in each language. Language adapters should
not contain benchmark-specific knowledge, and adding a language should not require changing the
runner's core logic. The harness must still allow a language to distinguish interpreted execution,
compilation, first-run setup, and warmed execution where those phases exist.

This generality is deliberate even though the first comparison has only Terrane and ordinary
Python. Scientific Python should be the third lane, followed by direct Rust and Terrane-to-Rust
crate comparisons; further languages can then be added without reshaping the project.

## Project Relationship

The authoritative corpus should live outside `demos/`. Demos are exploratory design exercises and
may intentionally use unsupported language ideas; they must not become correctness or performance
evidence for the compiler. Suitable scientific programs may later be adapted into demos or
supported examples once their required language surface is implemented.

The corpus should likewise remain distinct from release performance gates for the compiler itself.
Individual stable measurements may eventually become regression baselines, but only after the
workload, environment, and expected variability are understood.

This work can begin with the subset Terrane supports today. Its scope should then grow with the
language rather than requiring speculative implementation of scientific facilities. The enduring
deliverable is a credible body of problems, controls, data, correctness criteria, and measurements
through which Terrane's scientific capabilities can be developed and judged.
