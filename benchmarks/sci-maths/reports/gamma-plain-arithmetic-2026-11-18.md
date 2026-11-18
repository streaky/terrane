# Terrane scientific mathematics and data workloads

Generated at `2026-11-18T16:52:41.327568+00:00`.

## Environment

| Property | Value |
|---|---|
| Platform | Linux-7.3.0-4-cachyos-x86_64-with-glibc2.44 |
| Kernel | 7.3.0-4-cachyos |
| Machine | x86_64 |
| CPU model | AMD Ryzen 9 3900XT 12-Core Processor |
| Physical cores | 12 |
| Logical CPUs | 24 |
| Memory | 94.2 GiB |
| CPU frequency governor | powersave |
| Load average at start (1 / 5 / 15 min) | 0.21 / 0.25 / 0.62 |
| Inherited performance environment | none of the recorded variables set |
| Runner Python | 3.14.9 |

## Measurement

- Warm-up executions per problem and lane: **2**
- Measured executions per problem and lane: **7**
- Clock: `time.perf_counter`
- Build cache: existing adapter-declared caches preserved.
- Execution timing: process spawn to exit, as observed by the parent; setup and preparation complete before spawning.
- Run order: problem-major and lane-minor within each warm-up or measured run index.
- Setup timeout: 300.000 s; runtime timeout: 60.000 s.
- Memory: unavailable: memory measurement requires Linux cgroup v2 with a writable delegated subtree and the memory controller enabled; could not initialize a measurement subtree at /sys/fs/cgroup/user.slice/user-1000.slice/session-277.scope: [Errno 13] Permission denied: '/sys/fs/cgroup/user.slice/user-1000.slice/session-277.scope/terrane-benchmark-runner-2468298'.
- Memory retry command: `systemd-run --user --scope --quiet --property=Delegate=yes --same-dir python3 benchmarks/sci-maths/run.py report --output benchmarks/sci-maths/reports/gamma-plain-arithmetic-2026-11-18.md`

## Lanes

| Lane | Implementation | Native build profile | Captured environment |
|---|---|---|---|
| Clean, idiomatic Terrane | Terrane compiler-generated Rust | Cargo release: opt-level 3, fat LTO, one codegen unit | terrane 0.1.0 (build rust 1.100.0, projection rustdoc nightly-2026-10-02); rustc 1.100.0 (0ef1bc386 2026-11-09); cargo 1.100.0 (3acc79c0c 2026-11-05) |
| Clean, idiomatic Python | system CPython using only the standard language and library | not applicable | Python 3.14.9 |
| Clean, idiomatic Rust control | standalone Rust compiled directly with rustc | opt-level=3, fat LTO, one codegen unit | rustc 1.100.0 (0ef1bc386 2026-11-09) |
| Clean, idiomatic Go control | standalone Go compiled directly with go build | default optimized Go build with trimmed paths | go version go1.27.3-X:nodwarf5 linux/amd64 |
| Python scientific stack | CPython with vectorized NumPy and SciPy special functions | official binary wheels from the locked uv environment | Python 3.14.9; numpy 2.5.2; scipy 1.18.1 |
| Terrane with numr | Terrane using numr 0.7.0 through /deps | Cargo release: opt-level 3, fat LTO, one codegen unit | terrane 0.1.0 (build rust 1.100.0, projection rustdoc nightly-2026-10-02); rustc 1.100.0 (0ef1bc386 2026-11-09); cargo 1.100.0 (3acc79c0c 2026-11-05) |

## Execution results

Each problem uses its fastest median wall time as the baseline.

### Language baseline

#### Deterministic sum-of-squares reduction

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| rust | 50000000 | 4166675000000 | 29.74 ms | 0.0% | 27.81 ms–31.97 ms | — | — | 0 |
| terrane | 50000000 | 4166675000000 | 35.14 ms | 18.1% | 33.15 ms–35.52 ms | — | — | 0 |
| go | 50000000 | 4166675000000 | 50.03 ms | 68.2% | 45.57 ms–51.26 ms | — | — | 0 |
| python | 50000000 | 4166675000000 | 5.098 s | 17038.0% | 5.015 s–5.396 s | — | — | 0 |

#### Materialized quadratic element-wise transformation

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| rust | 10000000 | 412683499.9998618 | 20.86 ms | 0.0% | 20.43 ms–22.27 ms | — | — | 0 |
| go | 10000000 | 412683499.9998618 | 29.02 ms | 39.2% | 27.03 ms–32.26 ms | — | — | 0 |
| terrane | 10000000 | 412683499.9998618 | 30.95 ms | 48.4% | 29.49 ms–31.27 ms | — | — | 0 |
| python | 10000000 | 412683500.0 | 1.828 s | 8665.5% | 1.794 s–1.863 s | — | — | 0 |

#### Fused rational transformation and reduction

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 20000000 | 10225090.319585808 | 49.59 ms | 0.0% | 49.13 ms–50.91 ms | — | — | 0 |
| go | 20000000 | 10225090.319585808 | 49.60 ms | 0.0% | 46.83 ms–52.96 ms | — | — | 0 |
| rust | 20000000 | 10225090.319585808 | 51.15 ms | 3.2% | 47.65 ms–52.31 ms | — | — | 0 |
| python | 20000000 | 10225090.319507949 | 3.826 s | 7614.8% | 3.652 s–3.919 s | — | — | 0 |

#### Branch-heavy Collatz stopping-time total

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 1000000 | 131434424 | 106.77 ms | 0.0% | 99.95 ms–111.03 ms | — | — | 0 |
| rust | 1000000 | 131434424 | 132.89 ms | 24.5% | 121.80 ms–143.06 ms | — | — | 0 |
| go | 1000000 | 131434424 | 210.07 ms | 96.8% | 190.58 ms–222.82 ms | — | — | 0 |
| python | 1000000 | 131434424 | 8.893 s | 8229.3% | 8.678 s–9.724 s | — | — | 0 |

#### Composed generation, moments, and outlier classification

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| go | 10000000 | 1021428 | 37.25 ms | 0.0% | 34.71 ms–39.61 ms | — | — | 0 |
| rust | 10000000 | 1021428 | 42.56 ms | 14.3% | 40.21 ms–44.33 ms | — | — | 0 |
| terrane | 10000000 | 1021428 | 46.00 ms | 23.5% | 45.13 ms–47.16 ms | — | — | 0 |
| python | 10000000 | 1021428 | 3.803 s | 10109.0% | 3.765 s–3.898 s | — | — | 0 |

### Scientific stack

#### Pairwise oscillatory Bessel-kernel energy

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 6709 | 0.11273859033211667 | 492.65 ms | 0.0% | 489.80 ms–544.43 ms | — | — | 0 |
| terrane-numr | 6709 | 0.11273859032554075 | 564.42 ms | 14.6% | 539.86 ms–567.33 ms | — | — | 0 |
| python-scipy | 6709 | 0.11273859023654974 | 1.296 s | 163.0% | 1.247 s–1.310 s | — | — | 0 |

#### Gamma survival-model calibration loss

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane-numr | 20000000 | 0.14715086979581601 | 1.840 s | 0.0% | 1.762 s–1.923 s | — | — | 0 |
| terrane | 20000000 | 0.14715086979582306 | 1.980 s | 7.6% | 1.864 s–2.040 s | — | — | 0 |
| python-scipy | 20000000 | 0.14715086979516895 | 2.714 s | 47.5% | 2.650 s–2.872 s | — | — | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [gamma-plain-arithmetic-2026-11-18.json](gamma-plain-arithmetic-2026-11-18.json)
