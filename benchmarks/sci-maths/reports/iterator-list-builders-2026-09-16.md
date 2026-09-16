# Terrane scientific mathematics and data workloads

Generated at `2026-09-16T04:38:04.839094+00:00`.

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
| Load average at start (1 / 5 / 15 min) | 2.00 / 3.78 / 6.83 |
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
- Memory: unavailable: memory measurement requires Linux cgroup v2 with a writable delegated subtree and the memory controller enabled; could not initialize a measurement subtree at /sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope: [Errno 13] Permission denied: '/sys/fs/cgroup/user.slice/user-1000.slice/session-167.scope/terrane-sci-runner-392574'.
- Memory retry command: `systemd-run --user --scope --quiet --property=Delegate=yes --same-dir python3 benchmarks/sci-maths/run.py report --output benchmarks/sci-maths/reports/iterator-list-builders-default-2026-09-16.md`

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
| rust | 50000000 | 4166675000000 | 30.01 ms | 0.0% | 28.13 ms–33.69 ms | — | — | 0 |
| terrane | 50000000 | 4166675000000 | 48.69 ms | 62.2% | 45.48 ms–50.38 ms | — | — | 0 |
| go | 50000000 | 4166675000000 | 49.79 ms | 65.9% | 45.77 ms–50.75 ms | — | — | 0 |
| python | 50000000 | 4166675000000 | 4.656 s | 15412.6% | 4.561 s–4.793 s | — | — | 0 |

#### Materialized quadratic element-wise transformation

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 10000000 | 412683499.9998618 | 20.84 ms | 0.0% | 20.61 ms–22.49 ms | — | — | 0 |
| rust | 10000000 | 412683499.9998618 | 21.12 ms | 1.3% | 19.13 ms–21.68 ms | — | — | 0 |
| go | 10000000 | 412683499.9998618 | 28.36 ms | 36.0% | 26.67 ms–29.26 ms | — | — | 0 |
| python | 10000000 | 412683500.0 | 1.917 s | 9096.9% | 1.807 s–1.938 s | — | — | 0 |

#### Fused rational transformation and reduction

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 20000000 | 10225090.319585808 | 49.25 ms | 0.0% | 46.16 ms–52.21 ms | — | — | 0 |
| go | 20000000 | 10225090.319585808 | 49.64 ms | 0.8% | 45.76 ms–49.90 ms | — | — | 0 |
| rust | 20000000 | 10225090.319585808 | 50.41 ms | 2.3% | 46.39 ms–52.86 ms | — | — | 0 |
| python | 20000000 | 10225090.319507949 | 3.812 s | 7638.7% | 3.686 s–3.892 s | — | — | 0 |

#### Branch-heavy Collatz stopping-time total

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| rust | 1000000 | 131434424 | 133.41 ms | 0.0% | 123.11 ms–139.20 ms | — | — | 0 |
| terrane | 1000000 | 131434424 | 207.61 ms | 55.6% | 190.44 ms–217.22 ms | — | — | 0 |
| go | 1000000 | 131434424 | 208.81 ms | 56.5% | 192.10 ms–222.73 ms | — | — | 0 |
| python | 1000000 | 131434424 | 8.845 s | 6529.7% | 8.755 s–8.956 s | — | — | 0 |

#### Composed generation, moments, and outlier classification

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| go | 10000000 | 1021428 | 37.18 ms | 0.0% | 34.47 ms–40.62 ms | — | — | 0 |
| rust | 10000000 | 1021428 | 41.06 ms | 10.4% | 39.38 ms–43.53 ms | — | — | 0 |
| terrane | 10000000 | 1021428 | 47.10 ms | 26.7% | 44.15 ms–48.81 ms | — | — | 0 |
| python | 10000000 | 1021428 | 3.805 s | 10134.0% | 3.630 s–3.832 s | — | — | 0 |

### Scientific stack

#### Pairwise oscillatory Bessel-kernel energy

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 6709 | 0.11273859033211667 | 474.90 ms | 0.0% | 473.61 ms–513.81 ms | — | — | 0 |
| terrane-numr | 6709 | 0.11273859032554075 | 542.64 ms | 14.3% | 499.77 ms–563.05 ms | — | — | 0 |
| python-scipy | 6709 | 0.11273859023654974 | 1.262 s | 165.7% | 1.216 s–1.309 s | — | — | 0 |

#### Gamma survival-model calibration loss

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| terrane | 20000000 | 0.14715086979582306 | 2.497 s | 0.0% | 2.442 s–2.635 s | — | — | 0 |
| terrane-numr | 20000000 | 0.14715086979581601 | 2.642 s | 5.8% | 2.481 s–2.715 s | — | — | 0 |
| python-scipy | 20000000 | 0.14715086979516895 | 2.778 s | 11.3% | 2.686 s–2.825 s | — | — | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [iterator-list-builders-default-2026-09-16.json](iterator-list-builders-default-2026-09-16.json)
