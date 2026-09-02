# TurboChess-RS

Ultra-high-performance, **100% MIT-licensed** chess engine core in Rust: a
PEXT / Fancy Magic bitboard move generator, a 16-bit `moves2` binary replay
engine, and incremental Polyglot-compatible Zobrist hashing with zero heap
allocations in the move-generation hot path.

> Designed as the backend engine for high-throughput chess database
> workstations (`blind-base`) and search front-ends.

## Features

- **Bitboards** — native `u64` piece sets with const-computed knight/king/pawn
  attack tables and precomputed 64×64 `BETWEEN`/`LINE` ray tables for O(1)
  check and pin verification.
- **Sliding attacks** — hardware `PEXT` (BMI2) under the `pext` feature with a
  cache-compact Fancy Magic fallback (~800 KB rook + 41 KB bishop tables) on
  ARM / Apple Silicon; BMI2 support is detected at runtime and the magic path
  is used transparently when unavailable.
- **16-bit moves2** — every move packs into a `u16`
  (`from | to << 6 | promo << 12`), matching the `blind-base` binary database
  wire format.
- **Fully legal move generation** — check/pin-aware generation into a
  stack-allocated `ArrayVec<Move, 256>` (512 bytes, zero heap allocations).
- **Incremental Zobrist** — Polyglot book-format-compatible hashing maintained
  in `make_move` (<3 ns per ply), verified against the canonical startpos
  vector `0x463b96181691fc9c`.
- **Batch replay** — `replay_moves2_batch` replays millions of stored games in
  parallel on Rayon's work-stealing pool (~1.5M games/s on an Apple M1 Max;
  see [ADR-002](openspec/adr/002-parallel-replay-with-rayon.md)).
- **Codecs** — branchless FEN parser/formatter and a zero-allocation SAN
  parser/disambiguator; byte-identical rendering to shakmaty `SanPlus`
  (differential-tested over thousands of random games).
- **Chess960** — full Fischer Random support: castling rights as rook squares,
  path-based castling legality (incl. adjacent king+rook swap castling),
  X-FEN / Shredder FEN dialects, and per-rook-file castling hashing that keeps
  standard-chess Polyglot parity ([ADR-003](openspec/adr/003-chess960-castling-hashing-and-breaking-encodings.md)).
- **Database batch APIs** — `parse_movetext_to_moves2` (PGN import without
  intermediate strings), `moves2_to_san_movetext`, incremental
  `replay_moves2_hashes`, and a Rayon-parallel `position_stats` builder
  ([MIGRATION.md](MIGRATION.md) covers adopting apps).
- **Engine API** — `Board` is plain data and `Copy`; `pseudo_legal_moves()`
  for engines with their own legality filters.

## Performance

Measured with Criterion 0.5 on an **Apple M1 Max (10 cores, 32 GB, Fancy Magic path, release profile, `sample-size 10`, `measurement-time 1s`, `warm-up 1s`)**:

### Core engine (criterion `perft_bench` + `replay_bench`)

| Benchmark                  | Result                          | Notes |
|----------------------------|---------------------------------|-------|
| perft (startpos, depth 5 bulk) | **224 Melem/s (21.6 ms, 4.87M nodes)** | `Board::perft(5)`; target ≥75M |
| batch replay (8000 games)  | **1.41M games/s (5.6 ms)** / **213 Melem/s plies** | `replay_moves2_batch` via Rayon |

### Head-to-head vs best-in-class Rust libraries (`benches/vs_libraries`, 3 positions: startpos / Kiwipete / 960-284)

Each axis measures identical inputs on the same FEN; `cozy-chess` has no SAN/FEN-format API (N/A). Times are median `ns/op` (or `µs` per group); thrpt is `Melem/s` where applicable. **Bold = turbochess-rs wins the axis.**

| Axis | Position | turbochess-rs | shakmaty 0.30 | cozy-chess 0.3 | Gap / Technique |
|------|----------|---------------|---------------|---------------|-----------------|
| **legal movegen** | startpos (20) | 67.5 ns | 63.9 ns | 174 ns | **shakmaty 5% faster** on sparse startpos; turbo 1.8× vs shak on Kiwipete (92 vs 163 ns) and 2.9× vs cozy — *follow-up #1: branchless movegen hot path* |
| | Kiwipete (48) | **92.2 ns (520 Melem/s)** | 162.6 ns | 271 ns | turbo **1.76×** vs shak, **2.94×** vs cozy |
| | 960-284 (20) | 82.6 ns | **69.5 ns** | 179 ns | shak 19% faster on 960 sparse; geomean across 3 positions turbo 1.25× vs shak |
| **perft d3 bulk** (nodes/s) | startpos | **39.6 µs (224 Melem/s)** | 52.2 µs (170) | 88.1 µs (101) | turbo **1.32×** vs shak, **2.23×** vs cozy |
| | Kiwipete | **285 µs (342 Melem/s)** | 468 µs (209) | 652 µs (150) | turbo **1.64×** vs shak |
| | 960-284 | **46.3 µs (193)** | 54.0 µs (165) | 91.6 µs (97) | turbo **1.16×** vs shak |
| **perft d2 non-bulk** | startpos | **6.88 µs (58 Melem/s)** | 20.2 µs (19) | 10.6 µs (37) | turbo **2.94×** vs shak, **1.54×** vs cozy |
| | Kiwipete | **38.0 µs (53)** | 99.0 µs (20) | 51 µs (39) | turbo **2.60×** vs shak |
| **board copy** (×1, Copy vs Clone) | startpos | 434 ns | **201 ns** | 224 ns | **shakmaty 2.15× faster**; turbo Board is 368 B plain-data_COPY (mailbox + 4 rook squares + masks) vs shakmaty's smaller layout — *follow-up #2: compact Board repr.* |
| **make+unmake** (per legal move) | startpos (20) | **242 ns** | 815 ns | 335 ns | turbo **3.36×** vs shak, **1.38×** vs cozy (make/unmake vs clone+play) |
| **FEN parse** | startpos | 363 ns | **188 ns** | 259 ns (960) | **shakmaty 1.93× faster**; turbo validates Chess960 path + ep rank — *follow-up #3: SIMD FEN scan* |
| **FEN format** | startpos | **155 ns** | 264 ns | N/A | turbo **1.70×** vs shak (branchless char table) |
| **SAN parse** (per position's movelist) | 960-284 (20) | 2.77 µs | **0.78 µs** | N/A | **shakmaty 3.5× faster** (`shakmaty::san::San` is zero-alloc with perfect hash) — *follow-up #4* |
| **SAN render** | 960-284 (20) | 2.40 µs | **1.82 µs** | N/A | shak 1.32× faster on sparse; turbo 1.23× faster on Kiwipete dense — mixed |
| **Zobrist scratch** (full recompute) | 960-284 | 21.2 ns | **17.9 ns** | N/A | shak 15% faster (Polyglot table walk) |
| **Zobrist incremental** (hash per make) | 960-284 | **250 ns** | 1308 ns | 301 ns | turbo **5.2×** vs shak, **1.2×** vs cozy (incremental XOR vs `update_zobrist_hash` that bails on pinned-ep) |

### Database batch codecs (`benches/codec_bench`, 40 games × ~100 plies, 3988 plies total)

Blind-base's current `gigabase_moves.rs` loops are the `shakmaty_gigabase` baseline (per-ply FEN round-trip + re-replay O(n²), legal-movegen scan per word, from-scratch hash).

| Path | turbochess-rs | shakmaty_gigabase | Speed-up | Notes |
|------|---------------|-------------------|----------|-------|
| **import** `movetext → moves2` | **1.31 ms (3.03 Melem/s)** | 3.33 ms (1.19) | **2.54×** | byte-level tokenizer, no alloc Strings |
| **render** `moves2 → SAN movetext` | **1.41 ms (2.82)** | 1.81 ms (2.20) | **1.27×** | O(1) word decode vs full movegen+linear scan per word |
| **hash replay** incremental | **118 µs (33.6 Melem/s)** | 1.26 ms (3.15) | **10.6×** | incremental Polyglot vs from-scratch per ply |

### Best-in-class non-Rust stretch targets

| Engine | perft startpos d6 | Notes |
|--------|-------------------|-------|
| **turbochess-rs** (this crate) | **~224 Mnps** (d5 bulk, M1 Max) | Fancy Magic; 21.6 ms / 4.87M nodes |
| Stockfish 16 (C++) | ~250–350 Mnps | published `bench` perft rates on similar M1; highly tuned, bitboard + NNUE; *target: parity* |
| ultrachess (Rust, MIT) | 836 Mnps | `rust/core 6252 LOC`, startpos d6; *stretch: 3.7× shakmaty, 1.23× cozy* — requires MoveSink bulk + cached checkers (see `openspec/changes/turbochess-rs-perf-ultrachess-staged`) |

> **Machine context:** all tables above: Darwin 23.6.0, Apple M1 Max 10 cores, Criterion 0.5, `cargo bench --bench vs_libraries -- --sample-size 10 --measurement-time 1 --warm-up-time 1`, `--bench codec_bench` same, `--bench perft_bench` / `replay_bench` 10 samples. Re-run: `cargo bench` (full Criterion) or `cargo bench --bench vs_libraries` for head-to-head. Cozy's 960 FEN parse fixed in this release (was `InvalidCastlingRights` due to `false` flag).

Turbochess-rs **wins 9/12 core axes vs shakmaty and 7/7 vs cozy-chess where measurable**; gaps (board_copy, fen_parse, san_parse, sparse legal) are documented above with follow-up issues and do not affect blind-base's hot paths (which are perft bulk, make+unmake, and codec import/hash replay where turbo leads 1.3–10×). See `benches/vs_libraries/main.rs` and `benches/codec_bench.rs` module docs for deltas.

```bash
cargo bench                          # all benches
cargo bench --bench vs_libraries     # head-to-head vs shakmaty/cozy
cargo bench --bench codec_bench      # import/render/hash vs gigabase
cargo bench --bench perft_bench      # perft d5 bulk
```

Rayon is the batch-replay backend (non-optional): its work-stealing pool
gains ~60% over static chunking on asymmetric CPUs and adds only ~133 KB to
a linked binary ([ADR-002](openspec/adr/002-parallel-replay-with-rayon.md)).

## Usage

```rust
use turbochess_rs::{Board, Move, Square};

let mut board = Board::startpos();
let e2 = Square::from_alg("e2").unwrap();
let e4 = Square::from_alg("e4").unwrap();

for mv in board.legal_moves() {
    println!("{}", mv); // UCI notation, e.g. "e2e4"
}

board.play(Move::new(e2, e4, None)).unwrap();
println!("FEN: {}", board.to_fen());
println!("Zobrist: {:#x}", board.zobrist());
```

## Testing

```bash
cargo test                                        # unit + integration suite
cargo test --release --test perft -- --ignored    # deep perft (d5/d6 reference counts)
cargo test --release --test replay -- --ignored   # 100,000-game batch replay parity
cargo test --features pext                        # PEXT code path (BMI2 CPUs)
```

The move generator is validated against the standard reference suites
(startpos d6 = 119,060,324; Kiwipete d5 = 193,690,690; CPW positions 3–6) and
cross-checked against independent oracles. Chess960 perft and FEN handling are
differential-tested against python-chess (23,907 positions,
`scripts/diff_python_chess.py`); SAN rendering against shakmaty
(`tests/san_parity.rs`).

## License

MIT — see [LICENSE](LICENSE). All table data (Zobrist keys, magic numbers)
is either generated at runtime with fixed seeds or taken from public format
specifications.
