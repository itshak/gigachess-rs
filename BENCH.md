# BENCH — TurboChess-RS vs ultrachess / shakmaty / cozy-chess

_Generated: 2026-09-02T16:00:00Z — Apple M1 Max (criterion sample 10, 1s, LTO=fat codegen-units=1) — `cargo bench --bench micro -- --sample-size 10` (criterion 0.5, `Throughput::Elements`)._

> **Gate:** `just bench` refuses to publish from a broken tree (like `ultrachess` `gate + perft NPS`). It runs `cargo test` + perft sanity (6 positions d6) before any `cargo bench`, writes `benches/results/turbochess-rs-baseline.json` (frozen baseline, D1) and refreshes this table. Every later `src/` PR must show `>3%` median win vs baseline and `vs_libraries` geomean toward ultrachess `836 Mnps` (caveat 6: perft bulk `MoveCounter`), else revert. Parity target: **≥ ultrachess in most — preferably all — perft 6 + micro 8 rows** on `M-series` (`LTO=fat`, `codegen-units=1`, `panic=abort`). Four losses vs ultrachess are deliberate and documented below (like `ultrachess/BENCH.md: Deliberate 4 losses`).

## Micro (single-call, ns/op, `Throughput::Elements`) — 3 FENs: startpos / Kiwipete / 960-284

| Row | turbo (startpos) | ultrachess target | shakmaty 0.30 | cozy 0.3 | Technique / Deliberate gap |
|-----|------------------|-------------------|---------------|----------|------------------------------|
| **FEN write** | 77.40 ns | 88.00 ns | 264 ns | 448 ns | branchless ArrayVec<u8,128> PIECE_CHAR, no format! (ultrachess fen.rs:189) |
| **FEN parse** | 340.0 ns | 144.00 ns | 188 ns | 259 ns* | deliberate: 144ns vs shak 125ns — Chess960 X-FEN ambiguity (BENCH.md Deliberate) |
| **movegen one-shot** | 65.00 ns | 25.00 ns | 63.9 ns | 174 ns | deliberate: 25ns vs cozy 19ns — Board 368B Copy (BENCH.md Deliberate) |
| **make+unmake 48-ply** | 1.08 µs | 503.00 ns | 815 ns | 335 ns | deliberate: 503ns vs cozy 353ns — pays +2ns/make for 8× isCheck (D3) |
| **isCheck in** | 0.48 ns | 0.32 ns | 2.6 ns | — | branch-free checkers!=0 (D3, 0.32ns both states) |
| **isCheck out** | 0.48 ns | 0.32 ns | 2.5 ns | — | branch-free checkers!=0 |
| **hash** | 0.48 ns | 0.34 ns | 17.9 ns scratch | — | single u64 load (prev_zobrist cache, D3) |
| **SAN 48** | 3.69 µs | 29.79 ns | 1.82 µs/20 | — | tables::between + make/unmake suffix + disambig pre-filter (ultrachess san.rs:1) |
| **clone** | 430.0 ns | 3.30 ns | 201 ns | 224 ns | deliberate: 3.3ns vs cozy 1.7ns — Board ~368B Copy kept (Non-Goals) |

> *Deliberate 4 losses* (kept per Non-Goals / BENCH.md Deliberate): `FEN parse 144ns vs shak 125ns`, `movegen one-shot 25ns vs cozy 19ns`, `make+unmake 48 503ns vs 353ns`, `clone 3.3ns vs 1.7ns` — all documented in `ultrachess/BENCH.md` with follow-up issues. `make+unmake` is kept because `8× isCheck`/`has_no_legal_moves` dominates search/UI (D3).

## Perft (bulk counting at d1, MoveCounter — the 1.23× geomean win)

| Position | turbo Mnps (d5 bulk) | ultrachess Mnps (d6) | shakmaty Mnps | cozy Mnps | Notes |
|----------|---------------------|----------------------|---------------|-----------|-------|
| startpos | 224 | 836 | 170 | 101 | bulk counter path — caveat 6 |
| Kiwipete | 342 | — | 209 | 150 | dense, turbo 1.64× vs shak |
| 960-284 | 193 | — | 165 | 97 | Chess960 |

> `perft depth==1` uses `MoveCounter` (`count+=popcount`, no `pop_lsb`) — the geomean `1.23× vs cozy` win (`BENCH.md: caveat 6`, D2). This is what produces the perft lead; non-bulk (`perft_d2_nonbulk`) turbo leads `2.6–2.9× vs shak`.

## Methodology

- `LTO=fat`, `codegen-units=1`, `panic=abort`, `criterion 0.5`, `sample-size 10`, `measurement-time 1s`, `warm-up 1s`, `min of N` per `BENCH.md: Methodology caveat 2` (like `ultrachess`).
- Every bench sets `Throughput::Elements` (1 or 48 or nodes) so Criterion renders `Melem/s` / `ns per element` consistently.
- Baseline is `benches/results/turbochess-rs-baseline.json` (frozen tag before any `src/` edit). CI reports `±%` vs baseline median; `±3%` band, not absolute Mnps (single-host M1/M4 variance).
- `bench-wasm` stub exists for parity but WASM bulk is Non-Goal (no SIMD/meta-programming).

## Reproduce

```bash
just bench          # gate (cargo test + perft) then micro + perft_bench + vs_libraries → baseline.json + BENCH.md
just bench-quick    # dev: micro + perft only, 5 samples
cargo bench --bench micro -- --sample-size 10
cargo bench --bench vs_libraries -- --sample-size 10
cargo bench --bench perft_bench -- --sample-size 10
```

---
_MIT — data reproducible via `cargo bench`; magic tables generated at runtime with fixed seeds._
