# ADR-002: Rayon as the sole batch-replay backend

## Context

`replay_moves2_batch` replays stored `moves2` games in parallel. The first
implementation (shipped with ADR-001's core engine) split the batch into
exactly `available_parallelism()` static chunks processed by
`std::thread::scope` workers — no extra dependencies. During review, an
opt-in `rayon` feature and a head-to-head Criterion comparison were added,
and the fallback was subsequently improved with dynamic (atomic-counter)
chunk distribution to give the comparison its best shot.

All numbers below: Apple M1 Max (8P+2E cores, 10 hardware threads), release
build, Criterion medians, 8,000 self-played games (~1.24M plies) per call,
`replay/batch_8000_games` and `replay/small_batches_128x8` benchmarks.

## Options considered

| Option | Large batch (8k games) | Small batches (128×8) | Notes |
|---|---|---|---|
| A. Static chunks (`std::thread::scope`) | 914K games/s | 203K games/s (sequential <64-game batches) | Original implementation |
| B. Dynamic chunking (atomic counter + mutex) | 1.21–1.26M games/s | 203K games/s (sequential by design) | +60 lines of sync/ordering code |
| C. Rayon `par_iter` | **1.41–1.48M games/s** (~206M plies/s) | **230K games/s** | +13% vs B on small batches |

### Why static chunking lost

The M1 Max has asymmetric cores (8 performance + 2 efficiency). With exactly
N chunks for N threads, the two efficiency-core threads process their whole
800-game chunk ~2× slower and dominate the join — work-stealing, not thread
count, is what matters on heterogeneous CPUs. Dynamic chunking recovers most
of this, but still loses ~15–20% to Rayon's fine-grained stealing, and its
per-call spawn cost caps the small-batch case at the sequential rate.

## Decision

**Rayon is the sole batch-replay backend.** It is a plain (non-optional)
dependency; the `std::thread::scope` fallback and the `rayon` feature flag
are removed.

Rationale:

1. **Performance is the product** (ADR-001): the best hand-rolled `std`
   effort still loses 15–20% on large batches and 13% on repeated small
   batches. There is no scenario in this crate where the fallback wins.
2. **Dependency cost is small and measured**: rayon adds 6 crates
   (`rayon`, `rayon-core`, `crossbeam-deque`, `crossbeam-epoch`,
   `crossbeam-utils`, `either`) and **~133 KB to a linked release binary**
   (470,576 → 606,688 bytes for a minimal consumer; rlib artifacts are a
   ~7 MB unstripped upper bound). Compile time increases by a few seconds.
3. **Licensing is unaffected**: rayon is MIT OR Apache-2.0; the crate's
   100% MIT claim holds.
4. **Lean beats configurable**: the fallback was ~60 lines of
   atomic/mutex/ordering code with its own correctness surface (order
   restoration, lock poisoning) that duplicated a solved problem. One
   backend means one code path to test and reason about.

## Consequences

- `replay_moves2_batch` is ~10 lines: `games.par_iter().map(...).collect()`.
- Rayon's global pool is process-wide and reused across calls; consumers
  that call the batch API from multiple threads share it safely.
- Rayon initializes its pool lazily on first use (~ms on the first call).
- If a future `no_std` effort starts (none is planned; the crate already
  requires `std` for `OnceLock`/`String`), the fallback can be resurrected
  from git history (commit `5488d17`.. this ADR's predecessor commit).
- Re-benchmark after major Rust or rayon upgrades; numbers are recorded
  here and in `README.md`.

## References

- ADR-001: Maximum Performance Primacy and Pure Native API Architecture
- Benchmarks: `benches/replay_bench.rs` (`cargo bench`)
- Rayon vs std comparison data: this document, tables above (2026-09-01)
