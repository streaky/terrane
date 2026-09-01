# Terrane scientific mathematics and data workloads

Generated at `2026-09-01T06:24:06.797419+00:00`.

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
| Load average at start (1 / 5 / 15 min) | 6.86 / 4.33 / 3.04 |
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
| Deterministic sum-of-squares reduction | terrane | 50000000 | 4166675000000 | 69.06 ms | 63.60 ms–75.12 ms | 768.0 KiB | 536.0 KiB–768.0 KiB | 0 |
| Deterministic sum-of-squares reduction | python | 50000000 | 4166675000000 | 5.148 s | 4.766 s–5.242 s | 3.7 MiB | 3.6 MiB–4.0 MiB | 0 |
| Deterministic sum-of-squares reduction | rust | 50000000 | 4166675000000 | 49.65 ms | 45.09 ms–59.25 ms | 768.0 KiB | 512.0 KiB–768.0 KiB | 0 |
| Materialized quadratic element-wise transformation | terrane | 10000000 | 412683499.9998618 | 106.02 ms | 100.52 ms–109.94 ms | 78.7 MiB | 78.2 MiB–79.8 MiB | 0 |
| Materialized quadratic element-wise transformation | python | 10000000 | 412683500.0 | 2.054 s | 1.915 s–2.109 s | 387.9 MiB | 387.7 MiB–390.2 MiB | 0 |
| Materialized quadratic element-wise transformation | rust | 10000000 | 412683499.9998618 | 44.76 ms | 36.71 ms–52.10 ms | 77.2 MiB | 76.8 MiB–77.3 MiB | 0 |
| Fused rational transformation and reduction | terrane | 20000000 | 10225090.319585808 | 68.89 ms | 61.94 ms–75.55 ms | 512.0 KiB | 476.0 KiB–768.0 KiB | 0 |
| Fused rational transformation and reduction | python | 20000000 | 10225090.319507949 | 4.073 s | 3.948 s–4.223 s | 4.0 MiB | 3.7 MiB–4.0 MiB | 0 |
| Fused rational transformation and reduction | rust | 20000000 | 10225090.319585808 | 67.32 ms | 62.02 ms–75.77 ms | 536.0 KiB | 256.0 KiB–1.0 MiB | 0 |
| Branch-heavy Collatz stopping-time total | terrane | 1000000 | 131434424 | 230.21 ms | 221.82 ms–242.38 ms | 536.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Branch-heavy Collatz stopping-time total | python | 1000000 | 131434424 | 9.538 s | 9.175 s–10.039 s | 3.7 MiB | 3.6 MiB–4.2 MiB | 0 |
| Branch-heavy Collatz stopping-time total | rust | 1000000 | 131434424 | 149.40 ms | 141.18 ms–155.21 ms | 768.0 KiB | 512.0 KiB–768.0 KiB | 0 |
| Composed generation, moments, and outlier classification | terrane | 10000000 | 1021428 | 124.76 ms | 112.75 ms–127.65 ms | 79.0 MiB | 78.0 MiB–79.3 MiB | 0 |
| Composed generation, moments, and outlier classification | python | 10000000 | 1021428 | 4.066 s | 3.867 s–4.271 s | 388.4 MiB | 387.7 MiB–389.7 MiB | 0 |
| Composed generation, moments, and outlier classification | rust | 10000000 | 1021428 | 62.61 ms | 59.45 ms–71.73 ms | 77.2 MiB | 77.0 MiB–77.5 MiB | 0 |

### Scientific stack

| Problem | Lane | Size | Result | Median wall time | Range | Median peak memory | Peak memory range | Warnings |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Pairwise oscillatory Bessel-kernel energy | terrane | 6709 | 0.11273859033211667 | 814.90 ms | 793.26 ms–882.07 ms | 536.0 KiB | 512.0 KiB–1.0 MiB | 0 |
| Pairwise oscillatory Bessel-kernel energy | python-scipy | 6709 | 0.11273859023654974 | 1.386 s | 1.321 s–1.462 s | 1.4 GiB | 1.4 GiB–1.4 GiB | 0 |
| Pairwise oscillatory Bessel-kernel energy | terrane-numr | 6709 | 0.11273859032554075 | 593.22 ms | 544.78 ms–642.81 ms | 512.0 KiB | 512.0 KiB–700.0 KiB | 0 |
| Gamma survival-model calibration loss | terrane | 20000000 | 0.14715086979582306 | 2.723 s | 2.580 s–2.794 s | 512.0 KiB | 464.0 KiB–516.0 KiB | 0 |
| Gamma survival-model calibration loss | python-scipy | 20000000 | 0.14715086979516895 | 2.966 s | 2.820 s–3.151 s | 952.5 MiB | 952.2 MiB–954.0 MiB | 0 |
| Gamma survival-model calibration loss | terrane-numr | 20000000 | 0.14715086979581601 | 2.009 s | 1.913 s–2.172 s | 512.0 KiB | 512.0 KiB–640.0 KiB | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [report-sci-tweaked.json](report-sci-tweaked.json)
