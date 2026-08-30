# Terrane scientific mathematics and data workloads

Generated at `2026-08-30T07:20:28.510630+00:00`.

## Environment

| Property | Value |
|---|---|
| Platform | Linux-7.1.8-1-cachyos-x86_64-with-glibc2.44 |
| Kernel | 7.1.8-1-cachyos |
| Machine | x86_64 |
| CPU model | AMD Ryzen 5 5600H with Radeon Graphics |
| Physical cores | 6 |
| Logical CPUs | 12 |
| Memory | 30.7 GiB |
| Runner Python | 3.14.7 |

## Measurement

- Warm-up executions per problem and lane: **2**
- Measured executions per problem and lane: **7**
- Clock: `time.perf_counter`
- Build cache: existing adapter-declared caches preserved.
- Execution timing: program process only; setup and preparation complete before timing begins.
- Run order: problem-major and lane-minor within each warm-up or measured run index.
- Setup timeout: 300.000 s; runtime timeout: 60.000 s.
- Memory: peak cgroup-v2 memory charged to a fresh cgroup containing the launched process and all descendants.
- Memory limitations: memory.peak includes anonymous memory, charged page cache, and kernel memory; shared pages are charged once. It is not an RSS measurement.

## Lanes

| Lane | Implementation | Native build profile | Captured environment |
|---|---|---|---|
| Clean, idiomatic Terrane | Terrane compiler-generated Rust | Cargo release | terrane 0.1.0; rustc 1.94.0 (4a4ef493e 2026-03-02); cargo 1.94.0 (85eff7c80 2026-01-15) |
| Clean, idiomatic Python | system CPython using only the standard language and library | not applicable | Python 3.14.7 |

## Execution results

| Problem | Lane | Size | Result | Median wall time | Range | Peak memory | Warnings |
|---|---|---:|---:|---:|---:|---:|---:|
| Deterministic sum-of-squares reduction | terrane | 50000000 | 4166675000000 | 178.47 ms | 166.41 ms–183.17 ms | 768.0 KiB | 0 |
| Deterministic sum-of-squares reduction | python | 50000000 | 4166675000000 | 4.420 s | 4.310 s–4.543 s | 4.2 MiB | 0 |
| Materialized quadratic element-wise transformation | terrane | 10000000 | 412683499.9998618 | 539.21 ms | 525.53 ms–548.38 ms | 78.5 MiB | 0 |
| Materialized quadratic element-wise transformation | python | 10000000 | 412683500.0 | 2.028 s | 1.978 s–2.177 s | 389.4 MiB | 0 |
| Fused rational transformation and reduction | terrane | 20000000 | 10225090.319585808 | 929.69 ms | 895.85 ms–944.31 ms | 768.0 KiB | 0 |
| Fused rational transformation and reduction | python | 20000000 | 10225090.319507949 | 3.575 s | 3.509 s–3.716 s | 4.2 MiB | 0 |
| Branch-heavy Collatz stopping-time total | terrane | 1000000 | 131434424 | 716.50 ms | 697.46 ms–721.65 ms | 1.1 MiB | 0 |
| Branch-heavy Collatz stopping-time total | python | 1000000 | 131434424 | 8.610 s | 8.408 s–8.732 s | 4.2 MiB | 0 |
| Composed generation, moments, and outlier classification | terrane | 10000000 | 1021428 | 813.25 ms | 758.53 ms–820.43 ms | 78.8 MiB | 0 |
| Composed generation, moments, and outlier classification | python | 10000000 | 1021428 | 3.814 s | 3.667 s–3.877 s | 388.9 MiB | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [initial-two-lane-baseline.json](initial-two-lane-baseline.json)
