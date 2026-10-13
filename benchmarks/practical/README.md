# Practical benchmarks

Real-world-shaped workloads, using the same declarative adapters, correctness gates, execution measurements, and reports as `sci-maths`. The shared implementation lives in `../runner.py`; run it through a suite's `run.py` entrypoint.

## HTTP/1 workload

| Problem | Transport | Terrane | Go reference | Rust reference |
|---|---|---|---|---|
| `http1-server` | HTTP/1.1, cleartext loopback TCP | Standalone core TCP/JSON implementation | fasthttp server, standard HTTP client | Axum server, Hyper client |

For size `N`, a process starts its own server, launches concurrent client requests for `i = 1..N`, POSTs JSON `{"value":i}` to `/api`, decodes the returned decimal integer, and prints the sum `N*(N+1)/2`. Correctness uses 10 requests (55); performance uses 1,000 (500500).

The Terrane implementation is authored entirely in Terrane: no `packages/http-server`, template engine, projected HTTP framework, Rust escape hatch, or authored Rust adapter. It uses core asynchronous TCP and `decode-typed-json` into a `document-decodable` payload with a `uint64` value, performing real JSON parsing and exact range checking. Unknown fields are ignored, as in the reference decoders. Typed core decoding borrows the parsed tree without canonical reserialization or subtree cloning. A flat accept loop spawns handlers into a typed structured task group; clients likewise use one group instead of recursive fork/join. Both groups drain and join every child. The threaded executor permits transferable work across runtime workers, core write-all handles partial writes, and owned read buffers accumulate fragmented input without reallocating receive storage. Omitted operation options install no cancellation token or timer.

## Run it

Requires Python 3.11+, Go 1.22+, and the repository-selected Rust toolchain. The Terrane adapter builds the repository compiler automatically. First builds need network access for the pinned reference dependencies; Go sums and Cargo locks are checked in.

From the repository root:

```console
python3 benchmarks/practical/run.py list
python3 benchmarks/practical/run.py check
python3 benchmarks/practical/run.py lower
python3 benchmarks/practical/run.py benchmark --runs 7 --warmups 2 \
  --output benchmarks/practical/results/local.json
python3 benchmarks/practical/run.py report
python3 benchmarks/practical/run.py --group http1 --lane terrane check
python3 benchmarks/practical/run.py --problem http1-server report
```

Selectors precede the command and can be repeated. `lower` writes inspectable Terrane application and support Rust beside its source. `report` produces Markdown and complete JSON records, including correctness, preparation, warm-ups, and measured runs. Defaults are two warm-ups and seven measured executions. Setup is limited to 600 seconds; each execution to 60 seconds. Reference retry loops are bounded externally by that process timeout.

Build/setup is outside execution timing. Each measurement includes process startup, server startup, request construction, networking, JSON work, response decoding, checksum accumulation, and process exit. Rust and generated Terrane programs use release optimization level 3, fat LTO, and one codegen unit. Go uses its standard optimized build. `--cold-builds` clears only adapter-declared suite-local build caches and reports preparation separately; it does not promise to clear external compiler caches.

Memory is measured with the existing delegated cgroup-v2 accounting, not a misleading RSS fallback. If delegation is unavailable, the runner leaves memory unavailable and prints a command to retry under a delegated systemd user scope.

## Interpretation

These compare complete implementations, not just handler throughput or isolated language overhead. All lanes use HTTP/1.1 over cleartext loopback TCP, but framework, runtime, connection pooling, scheduling, and startup policies differ.

The minimal Terrane HTTP/1 implementation uses one request per connection and sends `Connection: close`. It handles partial writes and fragmented ASCII benchmark messages using bounded Content-Length framing, verifies the request/status line, performs actual JSON parsing, and checks every returned value. It is not a general HTTP server: no chunked encoding, keep-alive, pipelining, arbitrary Unicode streaming bodies, or general routing. The upstream clients retain their connection-pooling and retry behavior. All lanes bind OS-selected ephemeral loopback ports before launching clients; this removes the upstream randomized-port collision race without changing request work.

Use the default repetitions and a controlled host for performance conclusions. A one-run report is a smoke check, not evidence that one language is inherently faster.

Shared runner regression checks: `python3 -m unittest discover -s benchmarks/tests -v`.

## Upstream attribution

The Go and Rust references come from [hanabi1224/Programming-Language-Benchmarks](https://github.com/hanabi1224/Programming-Language-Benchmarks), under its MIT license. Each imported implementation includes the upstream copyright/license notice and a source URL pinned to its source commit.

| Reference | Pinned upstream source |
|---|---|
| HTTP/1 Go | [`2.go` at `71e9eb58`](https://github.com/hanabi1224/Programming-Language-Benchmarks/blob/71e9eb58cdbd6b3cb8568b1a135e0d75209ca318/bench/algorithm/http-server/2.go) |
| HTTP/1 Rust | [`1.rs` at `6aead359`](https://github.com/hanabi1224/Programming-Language-Benchmarks/blob/6aead3598958a3b310dc7ed3d33cc33c6f27382e/bench/algorithm/http-server/1.rs) |

Local adaptations are recorded in source headers: both references bind OS-assigned ports before starting clients instead of guessing a random free port; Go closes response bodies and targets `/api` instead of `/`. Rust retains the supplied HTTP algorithm and legacy framework APIs, with pinned dependencies. Random-number dependencies used solely for port selection are removed. The reference workload, concurrent requests, protocol, and checksum remain intact.
