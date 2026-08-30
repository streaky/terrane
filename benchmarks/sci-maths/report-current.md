# Terrane scientific mathematics and data workloads

Generated at `2026-08-30T04:54:38.106666+00:00`.

## Environment

| Property | Value |
|---|---|
| Platform | Linux-7.1.8-1-cachyos-x86_64-with-glibc2.44 |
| Machine | x86_64 |
| Processor | not reported |
| Python | 3.14.7 |
| Logical CPUs | 12 |

## Measurement

- Warm-up executions per problem and lane: **2**
- Measured executions per problem and lane: **7**
- Clock: `time.perf_counter`
- Build cache: existing adapter-declared caches preserved.
- Execution timing: program process only; setup and preparation complete before timing begins.
- Memory: sum of sampled Linux /proc VmRSS for the process and observed descendants.
- Memory limitations: The sampler waits 5 ms between scans, so its effective cadence also includes /proc scan time. Sampling can miss short-lived peaks; RSS includes runtime and allocator-retained pages and does not separate shared mappings.

## Lanes

| Lane | Implementation | Native build profile |
|---|---|---|
| Clean, idiomatic Terrane | Terrane compiler-generated Rust | Cargo release |
| Clean, idiomatic Python | system CPython using only the standard language and library | not applicable |

## Execution results

| Problem | Lane | Result | Median wall time | Range | Peak RSS |
|---|---|---:|---:|---:|---:|
| Deterministic sum-of-squares reduction | terrane | 4166675000000 | 165.95 ms | 164.99 ms–204.48 ms | 2.2 MiB |
| Deterministic sum-of-squares reduction | python | 4166675000000 | 4.469 s | 4.434 s–4.588 s | 11.9 MiB |
| Materialized quadratic element-wise transformation | terrane | 412683499.9998618 | 505.37 ms | 489.14 ms–509.76 ms | 80.4 MiB |
| Materialized quadratic element-wise transformation | python | 412683499.9998618 | 1.591 s | 1.559 s–1.597 s | 396.6 MiB |
| Fused rational transformation and reduction | terrane | 10225090.319585808 | 800.18 ms | 727.59 ms–855.74 ms | 2.6 MiB |
| Fused rational transformation and reduction | python | 10225090.319585808 | 3.145 s | 3.139 s–3.223 s | 12.0 MiB |
| Branch-heavy Collatz stopping-time total | terrane | 131434424 | 679.08 ms | 672.50 ms–718.35 ms | 2.3 MiB |
| Branch-heavy Collatz stopping-time total | python | 131434424 | 9.720 s | 9.469 s–10.358 s | 12.0 MiB |
| Composed generation, moments, and outlier classification | terrane | 1021428 | 802.80 ms | 742.33 ms–824.65 ms | 80.4 MiB |
| Composed generation, moments, and outlier classification | python | 1021428 | 2.919 s | 2.904 s–2.955 s | 396.2 MiB |

Every recorded execution passed its problem's shared correctness contract.

Complete measurements: [report-20260830T045438.json](report-20260830T045438.json)
