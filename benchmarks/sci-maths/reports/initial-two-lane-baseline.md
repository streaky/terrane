# Terrane scientific mathematics and data workloads

Generated at `2026-08-30T06:35:40.262058+00:00`.

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
- Memory: kernel ru_maxrss high-water mark from wait4 for the launched process and children it waited for.
- Memory limitations: ru_maxrss is a lifetime high-water mark, not simultaneous summed tree RSS; it includes runtime and allocator-retained pages and does not separate shared mappings.

## Lanes

| Lane | Implementation | Native build profile | Captured environment |
|---|---|---|---|
| Clean, idiomatic Terrane | Terrane compiler-generated Rust | Cargo release | terrane 0.1.0; rustc 1.94.0 (4a4ef493e 2026-03-02); cargo 1.94.0 (85eff7c80 2026-01-15) |
| Clean, idiomatic Python | system CPython using only the standard language and library | not applicable | Python 3.14.7 |

## Execution results

| Problem | Lane | Size | Result | Median wall time | Range | Peak RSS | Warnings |
|---|---|---:|---:|---:|---:|---:|---:|
| Deterministic sum-of-squares reduction | terrane | 50000000 | 4166675000000 | 156.56 ms | 153.17 ms–183.26 ms | 22.9 MiB | 0 |
| Deterministic sum-of-squares reduction | python | 50000000 | 4166675000000 | 4.238 s | 4.172 s–4.561 s | 22.9 MiB | 0 |
| Materialized quadratic element-wise transformation | terrane | 10000000 | 412683499.9998618 | 525.99 ms | 520.89 ms–536.44 ms | 80.6 MiB | 0 |
| Materialized quadratic element-wise transformation | python | 10000000 | 412683500.0 | 1.907 s | 1.865 s–2.026 s | 396.8 MiB | 0 |
| Fused rational transformation and reduction | terrane | 20000000 | 10225090.319585808 | 903.52 ms | 892.40 ms–969.54 ms | 22.9 MiB | 0 |
| Fused rational transformation and reduction | python | 20000000 | 10225090.319507949 | 3.391 s | 3.329 s–3.562 s | 22.9 MiB | 0 |
| Branch-heavy Collatz stopping-time total | terrane | 1000000 | 131434424 | 693.88 ms | 676.56 ms–786.35 ms | 22.9 MiB | 0 |
| Branch-heavy Collatz stopping-time total | python | 1000000 | 131434424 | 8.262 s | 8.090 s–8.798 s | 22.9 MiB | 0 |
| Composed generation, moments, and outlier classification | terrane | 10000000 | 1021428 | 792.56 ms | 719.57 ms–832.29 ms | 80.3 MiB | 0 |
| Composed generation, moments, and outlier classification | python | 10000000 | 1021428 | 3.640 s | 3.558 s–3.736 s | 396.5 MiB | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [initial-two-lane-baseline.json](initial-two-lane-baseline.json)
