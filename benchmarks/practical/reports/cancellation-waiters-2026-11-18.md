# Terrane practical workloads

Generated at `2026-11-18T17:50:55.982111+00:00`.

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
| Load average at start (1 / 5 / 15 min) | 2.27 / 1.62 / 1.15 |
| Inherited performance environment | none of the recorded variables set |
| Runner Python | 3.14.9 |

## Measurement

- Warm-up executions per problem and lane: **2**
- Measured executions per problem and lane: **7**
- Clock: `time.perf_counter`
- Build cache: existing adapter-declared caches preserved.
- Execution timing: process spawn to exit, as observed by the parent; setup and preparation complete before spawning.
- Run order: problem-major and lane-minor within each warm-up or measured run index.
- Setup timeout: 600.000 s; runtime timeout: 60.000 s.
- Memory: peak cgroup-v2 memory charged to a fresh cgroup containing the launched process and all descendants.
- Memory limitations: memory.peak includes anonymous memory, charged page cache, and kernel memory. Shared pages are charged once, potentially to a cgroup outside the measured execution, so small-footprint results depend on page-cache state and are not directly comparable across machines or reboots. It is not an RSS measurement.

## Lanes

| Lane | Implementation | Native build profile | Captured environment |
|---|---|---|---|
| Pure Terrane HTTP/1.1 | Standalone Terrane source using core TCP and JSON; no authored Rust dependencies or escape hatches | Cargo release: opt-level 3, fat LTO, one codegen unit | terrane 0.1.0 (build rust 1.100.0, projection rustdoc nightly-2026-10-02); rustc 1.100.0 (0ef1bc386 2026-11-09); cargo 1.100.0 (3acc79c0c 2026-11-05) |
| Go protocol reference | Upstream protocol-specific Go HTTP server/client reference | default optimized Go build with trimmed paths | go version go1.27.3-X:nodwarf5 linux/amd64 |
| Rust protocol reference | Upstream Rust HTTP server/client reference with pinned dependencies | Cargo release: opt-level 3, fat LTO, one codegen unit | rustc 1.100.0 (0ef1bc386 2026-11-09); cargo 1.100.0 (3acc79c0c 2026-11-05) |

## Execution results

Each problem uses its fastest median wall time as the baseline.

### HTTP/1 workloads

#### Concurrent HTTP/1 JSON request-response workload

| Lane | Size | Result | Median wall time | Longer than fastest | Range | Median peak memory | Peak memory range | Warnings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| go | 1000 | 500500 | 78.73 ms | 0.0% | 60.26 ms–99.21 ms | 54.1 MiB | 41.4 MiB–69.3 MiB | 0 |
| terrane | 1000 | 500500 | 90.38 ms | 14.8% | 72.11 ms–100.54 ms | 50.7 MiB | 48.8 MiB–53.1 MiB | 0 |
| rust | 1000 | 500500 | 97.39 ms | 23.7% | 80.60 ms–110.58 ms | 48.7 MiB | 48.3 MiB–49.6 MiB | 0 |

Every recorded execution passed its problem's shared correctness contract. Successful process stderr is retained in the raw data; **0 warning line(s)** were detected.

Complete measurements: [cancellation-waiters-2026-11-18.json](cancellation-waiters-2026-11-18.json)
