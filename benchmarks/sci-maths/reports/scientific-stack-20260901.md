# Terrane scientific mathematics and data workloads

Generated at `2026-09-01T08:48:19.513502+00:00`.

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
| Load average at start (1 / 5 / 15 min) | 1.03 / 2.79 / 3.10 |
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
| Deterministic sum-of-squares reduction | terrane | 50000000 | 4166675000000 | 66.04 ms | 63.82 ms–70.31 ms | 768.0 KiB | 512.0 KiB–796.0 KiB | 0 |
| Deterministic sum-of-squares reduction | python | 50000000 | 4166675000000 | 4.595 s | 4.574 s–4.691 s | 3.7 MiB | 3.6 MiB–4.0 MiB | 0 |
| Deterministic sum-of-squares reduction | rust | 50000000 | 4166675000000 | 46.88 ms | 38.37 ms–53.41 ms | 768.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Materialized quadratic element-wise transformation | terrane | 10000000 | 412683499.9998618 | 100.77 ms | 91.99 ms–108.00 ms | 78.5 MiB | 78.0 MiB–79.0 MiB | 0 |
| Materialized quadratic element-wise transformation | python | 10000000 | 412683500.0 | 1.852 s | 1.843 s–1.990 s | 387.8 MiB | 387.4 MiB–388.5 MiB | 0 |
| Materialized quadratic element-wise transformation | rust | 10000000 | 412683499.9998618 | 40.96 ms | 35.09 ms–46.43 ms | 77.5 MiB | 76.8 MiB–77.8 MiB | 0 |
| Fused rational transformation and reduction | terrane | 20000000 | 10225090.319585808 | 66.77 ms | 58.76 ms–70.45 ms | 900.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Fused rational transformation and reduction | python | 20000000 | 10225090.319507949 | 3.854 s | 3.749 s–4.012 s | 3.8 MiB | 3.6 MiB–4.0 MiB | 0 |
| Fused rational transformation and reduction | rust | 20000000 | 10225090.319585808 | 66.37 ms | 63.42 ms–73.54 ms | 768.0 KiB | 768.0 KiB–1.0 MiB | 0 |
| Branch-heavy Collatz stopping-time total | terrane | 1000000 | 131434424 | 226.40 ms | 215.11 ms–253.26 ms | 696.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Branch-heavy Collatz stopping-time total | python | 1000000 | 131434424 | 9.039 s | 8.950 s–9.363 s | 3.7 MiB | 3.6 MiB–4.2 MiB | 0 |
| Branch-heavy Collatz stopping-time total | rust | 1000000 | 131434424 | 145.61 ms | 134.34 ms–158.21 ms | 768.0 KiB | 512.0 KiB–768.0 KiB | 0 |
| Composed generation, moments, and outlier classification | terrane | 10000000 | 1021428 | 114.65 ms | 106.84 ms–121.14 ms | 78.4 MiB | 78.0 MiB–79.2 MiB | 0 |
| Composed generation, moments, and outlier classification | python | 10000000 | 1021428 | 3.817 s | 3.700 s–3.943 s | 387.5 MiB | 387.3 MiB–387.9 MiB | 0 |
| Composed generation, moments, and outlier classification | rust | 10000000 | 1021428 | 58.66 ms | 54.30 ms–64.07 ms | 77.2 MiB | 77.0 MiB–77.5 MiB | 0 |

### Scientific stack

| Problem | Lane | Size | Result | Median wall time | Range | Median peak memory | Peak memory range | Warnings |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Pairwise oscillatory Bessel-kernel energy | terrane | 6709 | 0.11273859033211667 | 779.05 ms | 720.97 ms–818.26 ms | 768.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Pairwise oscillatory Bessel-kernel energy | python-scipy | 6709 | 0.11273859023654974 | 1.287 s | 1.251 s–1.408 s | 1.4 GiB | 1.4 GiB–1.4 GiB | 0 |
| Pairwise oscillatory Bessel-kernel energy | terrane-numr | 6709 | 0.11273859032554075 | 562.47 ms | 521.03 ms–571.79 ms | 512.0 KiB | 512.0 KiB–768.0 KiB | 0 |
| Gamma survival-model calibration loss | terrane | 20000000 | 0.14715086979582306 | 2.559 s | 2.469 s–2.840 s | 700.0 KiB | 512.0 KiB–876.0 KiB | 0 |
| Gamma survival-model calibration loss | python-scipy | 20000000 | 0.14715086979516895 | 2.784 s | 2.705 s–3.007 s | 953.2 MiB | 952.8 MiB–954.4 MiB | 0 |
| Gamma survival-model calibration loss | terrane-numr | 20000000 | 0.14715086979581601 | 1.968 s | 1.841 s–1.997 s | 512.0 KiB | 512.0 KiB–512.0 KiB | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [scientific-stack-20260901.json](scientific-stack-20260901.json)
