# Terrane scientific mathematics and data workloads

Generated at `2026-08-30T17:43:29.990480+00:00`.

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
| CPU frequency governor | powersave |
| Load average at start (1 / 5 / 15 min) | 1.82 / 1.77 / 1.70 |
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
- Memory: peak cgroup-v2 memory charged to a fresh cgroup containing the launched process and all descendants.
- Memory limitations: memory.peak includes anonymous memory, charged page cache, and kernel memory. Shared pages are charged once, potentially to a cgroup outside the measured execution, so small-footprint results depend on page-cache state and are not directly comparable across machines or reboots. It is not an RSS measurement.

## Lanes

| Lane | Implementation | Native build profile | Captured environment |
|---|---|---|---|
| Clean, idiomatic Terrane | Terrane compiler-generated Rust | Cargo release | terrane 0.1.0; rustc 1.94.0 (4a4ef493e 2026-03-02); cargo 1.94.0 (85eff7c80 2026-01-15) |
| Clean, idiomatic Python | system CPython using only the standard language and library | not applicable | Python 3.14.7 |

## Execution results

| Problem | Lane | Size | Result | Median wall time | Range | Median peak memory | Peak memory range | Warnings |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Deterministic sum-of-squares reduction | terrane | 50000000 | 4166675000000 | 164.68 ms | 159.02 ms–168.19 ms | 540.0 KiB | 460.0 KiB–692.0 KiB | 0 |
| Deterministic sum-of-squares reduction | python | 50000000 | 4166675000000 | 4.142 s | 4.110 s–4.201 s | 4.0 MiB | 3.8 MiB–4.0 MiB | 0 |
| Materialized quadratic element-wise transformation | terrane | 10000000 | 412683499.9998618 | 501.34 ms | 487.01 ms–515.80 ms | 78.1 MiB | 77.5 MiB–78.7 MiB | 0 |
| Materialized quadratic element-wise transformation | python | 10000000 | 412683500.0 | 1.723 s | 1.710 s–1.876 s | 387.4 MiB | 387.2 MiB–389.2 MiB | 0 |
| Fused rational transformation and reduction | terrane | 20000000 | 10225090.319585808 | 899.51 ms | 864.91 ms–934.44 ms | 512.0 KiB | 468.0 KiB–696.0 KiB | 0 |
| Fused rational transformation and reduction | python | 20000000 | 10225090.319507949 | 3.320 s | 3.287 s–3.493 s | 4.0 MiB | 3.8 MiB–4.0 MiB | 0 |
| Branch-heavy Collatz stopping-time total | terrane | 1000000 | 131434424 | 683.25 ms | 671.99 ms–692.19 ms | 512.0 KiB | 512.0 KiB–540.0 KiB | 0 |
| Branch-heavy Collatz stopping-time total | python | 1000000 | 131434424 | 8.082 s | 7.932 s–8.116 s | 4.0 MiB | 3.7 MiB–4.2 MiB | 0 |
| Composed generation, moments, and outlier classification | terrane | 10000000 | 1021428 | 790.09 ms | 721.67 ms–811.95 ms | 77.8 MiB | 77.6 MiB–78.4 MiB | 0 |
| Composed generation, moments, and outlier classification | python | 10000000 | 1021428 | 3.550 s | 3.523 s–3.778 s | 387.5 MiB | 387.3 MiB–389.2 MiB | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [initial-two-lane-baseline.json](initial-two-lane-baseline.json)
