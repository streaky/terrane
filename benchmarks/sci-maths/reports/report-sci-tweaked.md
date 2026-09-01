# Terrane scientific mathematics and data workloads

Generated at `2026-09-01T02:47:55.199452+00:00`.

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
| Load average at start (1 / 5 / 15 min) | 0.81 / 2.29 / 3.41 |
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
| Clean, idiomatic Terrane | Terrane compiler-generated Rust | Cargo release | terrane 0.1.0; rustc 1.93.0 (254b59607 2026-01-19); cargo 1.93.0 (083ac5135 2025-12-15) |
| Clean, idiomatic Python | system CPython using only the standard language and library | not applicable | Python 3.14.7 |
| Clean, idiomatic Rust control | standalone Rust compiled directly with rustc | opt-level=3, fat LTO, one codegen unit | rustc 1.93.0 (254b59607 2026-01-19) |
| Python scientific stack | CPython with vectorized NumPy and SciPy special functions | official binary wheels from the locked uv environment | Python 3.14.7; numpy 2.5.2; scipy 1.18.1 |
| Terrane with numr | Terrane using numr 0.7.0 through /deps | Cargo release | terrane 0.1.0; rustc 1.93.0 (254b59607 2026-01-19); cargo 1.93.0 (083ac5135 2025-12-15) |

## Execution results

### Language baseline

| Problem | Lane | Size | Result | Median wall time | Range | Median peak memory | Peak memory range | Warnings |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Deterministic sum-of-squares reduction | terrane | 50000000 | 4166675000000 | 62.37 ms | 57.44 ms–73.58 ms | 768.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Deterministic sum-of-squares reduction | python | 50000000 | 4166675000000 | 4.727 s | 4.625 s–5.003 s | 3.8 MiB | 3.6 MiB–4.0 MiB | 0 |
| Deterministic sum-of-squares reduction | rust | 50000000 | 4166675000000 | 43.42 ms | 38.30 ms–48.52 ms | 768.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Materialized quadratic element-wise transformation | terrane | 10000000 | 412683499.9998618 | 102.06 ms | 96.93 ms–152.91 ms | 78.5 MiB | 77.8 MiB–78.8 MiB | 0 |
| Materialized quadratic element-wise transformation | python | 10000000 | 412683500.0 | 1.927 s | 1.837 s–2.033 s | 388.8 MiB | 387.2 MiB–389.5 MiB | 0 |
| Materialized quadratic element-wise transformation | rust | 10000000 | 412683499.9998618 | 43.69 ms | 38.50 ms–50.75 ms | 77.0 MiB | 76.9 MiB–77.2 MiB | 0 |
| Fused rational transformation and reduction | terrane | 20000000 | 10225090.319585808 | 65.56 ms | 58.05 ms–69.77 ms | 768.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Fused rational transformation and reduction | python | 20000000 | 10225090.319507949 | 3.914 s | 3.724 s–4.042 s | 3.8 MiB | 3.6 MiB–3.8 MiB | 0 |
| Fused rational transformation and reduction | rust | 20000000 | 10225090.319585808 | 66.51 ms | 63.81 ms–73.01 ms | 768.0 KiB | 536.0 KiB–1.0 MiB | 0 |
| Branch-heavy Collatz stopping-time total | terrane | 1000000 | 131434424 | 220.20 ms | 209.93 ms–228.30 ms | 768.0 KiB | 768.0 KiB–1.0 MiB | 0 |
| Branch-heavy Collatz stopping-time total | python | 1000000 | 131434424 | 9.024 s | 8.784 s–9.266 s | 3.7 MiB | 3.6 MiB–4.0 MiB | 0 |
| Branch-heavy Collatz stopping-time total | rust | 1000000 | 131434424 | 148.44 ms | 135.61 ms–152.76 ms | 768.0 KiB | 508.0 KiB–768.0 KiB | 0 |
| Composed generation, moments, and outlier classification | terrane | 10000000 | 1021428 | 124.10 ms | 110.57 ms–131.16 ms | 78.7 MiB | 78.0 MiB–79.2 MiB | 0 |
| Composed generation, moments, and outlier classification | python | 10000000 | 1021428 | 3.865 s | 3.782 s–3.931 s | 387.8 MiB | 387.4 MiB–389.7 MiB | 0 |
| Composed generation, moments, and outlier classification | rust | 10000000 | 1021428 | 57.71 ms | 49.71 ms–63.48 ms | 77.5 MiB | 77.0 MiB–77.7 MiB | 0 |

### Scientific stack

| Problem | Lane | Size | Result | Median wall time | Range | Median peak memory | Peak memory range | Warnings |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Pairwise oscillatory Bessel-kernel energy | terrane | 6709 | 0.11273859033211667 | 764.24 ms | 715.28 ms–780.74 ms | 540.0 KiB | 536.0 KiB–792.0 KiB | 0 |
| Pairwise oscillatory Bessel-kernel energy | python-scipy | 6709 | 0.11273859023654974 | 1.299 s | 1.243 s–1.418 s | 1.4 GiB | 1.4 GiB–1.4 GiB | 0 |
| Pairwise oscillatory Bessel-kernel energy | terrane-numr | 6709 | 0.11273859032554075 | 559.01 ms | 516.24 ms–566.60 ms | 512.0 KiB | 464.0 KiB–704.0 KiB | 0 |
| Gamma survival-model calibration loss | terrane | 20000000 | 0.14715086979582306 | 2.546 s | 2.504 s–2.663 s | 772.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Gamma survival-model calibration loss | python-scipy | 20000000 | 0.14715086979516895 | 2.826 s | 2.740 s–2.885 s | 953.0 MiB | 952.2 MiB–953.7 MiB | 0 |
| Gamma survival-model calibration loss | terrane-numr | 20000000 | 0.14715086979581601 | 1.974 s | 1.854 s–2.096 s | 512.0 KiB | 512.0 KiB–696.0 KiB | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [report-20260901T024755.json](report-20260901T024755.json)
