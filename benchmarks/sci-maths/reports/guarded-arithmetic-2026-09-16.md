# Terrane scientific mathematics and data workloads

Generated at `2026-09-16T13:20:53.220665+00:00`.

## Environment

| Property | Value |
|---|---|
| Platform | Linux-7.2.3-1-cachyos-x86_64-with-glibc2.44 |
| Kernel | 7.2.3-1-cachyos |
| Machine | x86_64 |
| CPU model | AMD Ryzen 9 3900XT 12-Core Processor |
| Physical cores | 12 |
| Logical CPUs | 24 |
| Memory | 94.2 GiB |
| CPU frequency governor | powersave |
| Load average at start (1 / 5 / 15 min) | 10.80 / 29.11 / 21.78 |
| Inherited performance environment | none of the recorded variables set |
| Runner Python | 3.14.7 |

## Measurement

- Warm-up executions per problem and lane: **2**
- Measured executions per problem and lane: **7**
- Clock: `time.perf_counter`
- Build cache: existing adapter-declared caches preserved.
- Execution timing: process spawn to exit, as observed by the parent; setup and preparation complete before spawning.
- Run order: problem-major and lane-minor within each warm-up or measured run index.
- Setup timeout: 300.000 s; runtime timeout: 60.000 s.
- Memory: unavailable: memory measurement requires Linux cgroup v2 with a writable delegated subtree and the memory controller enabled; could not initialize a measurement subtree at /sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope: [Errno 13] Permission denied: '/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope/terrane-sci-runner-2421520'.
- Memory retry command: `systemd-run --user --scope --quiet --property=Delegate=yes --same-dir python3 benchmarks/sci-maths/run.py report --runs 7 --warmups 2 --output benchmarks/sci-maths/reports/guarded-arithmetic-2026-09-16.md`

## Lanes

| Lane | Implementation | Native build profile | Captured environment |
|---|---|---|---|
| Clean, idiomatic Terrane | Terrane compiler-generated Rust | Cargo release: opt-level 3, fat LTO, one codegen unit | terrane 0.1.0 (build rust 1.98.1, projection rustdoc nightly-2026-04-29); rustc 1.98.1 (48a229cea 2026-09-01); cargo 1.98.1 (797e8a9bc 2026-08-05) |
| Clean, idiomatic Python | system CPython using only the standard language and library | not applicable | Python 3.14.7 |
| Clean, idiomatic Rust control | standalone Rust compiled directly with rustc | opt-level=3, fat LTO, one codegen unit | rustc 1.98.1 (48a229cea 2026-09-01) |
| Clean, idiomatic Go control | standalone Go compiled directly with go build | default optimized Go build with trimmed paths | go version go1.27.1-X:nodwarf5 linux/amd64 |
| Python scientific stack | CPython with vectorized NumPy and SciPy special functions | official binary wheels from the locked uv environment | Python 3.14.7; numpy 2.5.2; scipy 1.18.1 |
| Terrane with numr | Terrane using numr 0.7.0 through /deps | Cargo release: opt-level 3, fat LTO, one codegen unit | terrane 0.1.0 (build rust 1.98.1, projection rustdoc nightly-2026-04-29); rustc 1.98.1 (48a229cea 2026-09-01); cargo 1.98.1 (797e8a9bc 2026-08-05) |

## Execution results

Each problem uses its fastest median wall time as the baseline.

### Language baseline

#### Deterministic sum-of-squares reduction

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| rust | 50000000 | 4166675000000 | 30.23 ms | 0.0% | 29.74 ms–33.28 ms | — | — | 0 |
| terrane | 50000000 | 4166675000000 | 48.47 ms | 60.4% | 44.44 ms–49.18 ms | — | — | 0 |
| go | 50000000 | 4166675000000 | 50.31 ms | 66.4% | 46.07 ms–55.27 ms | — | — | 0 |
| python | 50000000 | 4166675000000 | 4.727 s | 15539.3% | 4.622 s–4.884 s | — | — | 0 |

#### Materialized quadratic element-wise transformation

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| rust | 10000000 | 412683499.9998618 | 20.50 ms | 0.0% | 19.73 ms–21.78 ms | — | — | 0 |
| terrane | 10000000 | 412683499.9998618 | 20.72 ms | 1.1% | 19.35 ms–21.94 ms | — | — | 0 |
| go | 10000000 | 412683499.9998618 | 28.84 ms | 40.7% | 27.57 ms–31.19 ms | — | — | 0 |
| python | 10000000 | 412683500.0 | 1.878 s | 9064.4% | 1.800 s–1.952 s | — | — | 0 |

#### Fused rational transformation and reduction

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| rust | 20000000 | 10225090.319585808 | 49.22 ms | 0.0% | 46.27 ms–51.34 ms | — | — | 0 |
| go | 20000000 | 10225090.319585808 | 49.68 ms | 0.9% | 47.10 ms–50.26 ms | — | — | 0 |
| terrane | 20000000 | 10225090.319585808 | 50.18 ms | 1.9% | 46.67 ms–51.60 ms | — | — | 0 |
| python | 20000000 | 10225090.319507949 | 3.806 s | 7631.7% | 3.701 s–3.976 s | — | — | 0 |

#### Branch-heavy Collatz stopping-time total

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 1000000 | 131434424 | 105.11 ms | 0.0% | 96.85 ms–108.80 ms | — | — | 0 |
| rust | 1000000 | 131434424 | 132.99 ms | 26.5% | 123.09 ms–139.80 ms | — | — | 0 |
| go | 1000000 | 131434424 | 201.31 ms | 91.5% | 194.04 ms–218.45 ms | — | — | 0 |
| python | 1000000 | 131434424 | 9.183 s | 8635.8% | 8.845 s–9.270 s | — | — | 0 |

#### Composed generation, moments, and outlier classification

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| go | 10000000 | 1021428 | 37.66 ms | 0.0% | 34.58 ms–40.46 ms | — | — | 0 |
| rust | 10000000 | 1021428 | 41.18 ms | 9.3% | 38.90 ms–44.12 ms | — | — | 0 |
| terrane | 10000000 | 1021428 | 46.00 ms | 22.1% | 43.68 ms–46.94 ms | — | — | 0 |
| python | 10000000 | 1021428 | 3.801 s | 9992.3% | 3.671 s–3.866 s | — | — | 0 |

### Scientific stack

#### Pairwise oscillatory Bessel-kernel energy

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 6709 | 0.11273859033211667 | 506.81 ms | 0.0% | 476.74 ms–519.80 ms | — | — | 0 |
| terrane-numr | 6709 | 0.11273859032554075 | 542.18 ms | 7.0% | 513.89 ms–546.46 ms | — | — | 0 |
| python-scipy | 6709 | 0.11273859023654974 | 1.289 s | 154.4% | 1.271 s–1.304 s | — | — | 0 |

#### Gamma survival-model calibration loss

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 20000000 | 0.14715086979582306 | 2.592 s | 0.0% | 2.489 s–2.636 s | — | — | 0 |
| terrane-numr | 20000000 | 0.14715086979581601 | 2.667 s | 2.9% | 2.596 s–2.753 s | — | — | 0 |
| python-scipy | 20000000 | 0.14715086979516895 | 2.748 s | 6.0% | 2.688 s–2.882 s | — | — | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [guarded-arithmetic-2026-09-16.json](guarded-arithmetic-2026-09-16.json)
