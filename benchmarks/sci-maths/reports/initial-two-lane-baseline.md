# Terrane scientific mathematics and data workloads

Generated at `2026-08-31T17:49:39.330396+00:00`.

## Environment

| Property | Value |
|---|---|
| Platform | Linux-6.18.42-1-cachyos-lts-x86_64-with-glibc2.44 |
| Kernel | 6.18.42-1-cachyos-lts |
| Machine | x86_64 |
| CPU model | AMD Ryzen 9 3900XT 12-Core Processor |
| Physical cores | 12 |
| Logical CPUs | 24 |
| Memory | 94.2 GiB |
| CPU frequency governor | powersave |
| Load average at start (1 / 5 / 15 min) | 6.60 / 3.14 / 1.30 |
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
- Memory: unavailable because delegated cgroup-v2 memory accounting is not accessible.

## Lanes

| Lane | Implementation | Native build profile | Captured environment |
|---|---|---|---|
| Clean, idiomatic Terrane | Terrane compiler-generated Rust | Cargo release | terrane 0.1.0; rustc 1.93.0 (254b59607 2026-01-19); cargo 1.93.0 (083ac5135 2025-12-15) |
| Clean, idiomatic Python | system CPython using only the standard language and library | not applicable | Python 3.14.7 |

## Execution results

| Problem | Lane | Size | Result | Median wall time | Range | Median peak memory | Peak memory range | Warnings |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Deterministic sum-of-squares reduction | terrane | 50000000 | 4166675000000 | 256.32 ms | 238.78 ms–264.79 ms | — | — | 0 |
| Deterministic sum-of-squares reduction | python | 50000000 | 4166675000000 | 4.740 s | 4.668 s–4.814 s | — | — | 0 |
| Materialized quadratic element-wise transformation | terrane | 10000000 | 412683499.9998618 | 441.48 ms | 416.27 ms–454.34 ms | — | — | 0 |
| Materialized quadratic element-wise transformation | python | 10000000 | 412683500.0 | 1.922 s | 1.814 s–2.056 s | — | — | 0 |
| Fused rational transformation and reduction | terrane | 20000000 | 10225090.319585808 | 735.40 ms | 682.80 ms–843.21 ms | — | — | 0 |
| Fused rational transformation and reduction | python | 20000000 | 10225090.319507949 | 3.864 s | 3.708 s–3.894 s | — | — | 0 |
| Branch-heavy Collatz stopping-time total | terrane | 1000000 | 131434424 | 1.366 s | 1.270 s–1.455 s | — | — | 0 |
| Branch-heavy Collatz stopping-time total | python | 1000000 | 131434424 | 8.964 s | 8.833 s–9.539 s | — | — | 0 |
| Composed generation, moments, and outlier classification | terrane | 10000000 | 1021428 | 792.56 ms | 732.54 ms–861.32 ms | — | — | 0 |
| Composed generation, moments, and outlier classification | python | 10000000 | 1021428 | 3.697 s | 3.668 s–3.939 s | — | — | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [report-20260831T174939.json](report-20260831T174939.json)
