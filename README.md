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
  parser/disambiguator.

## Performance

Measured with Criterion on an Apple M1 Max (10 cores, Fancy Magic path):

| Benchmark                  | Result                          |
|----------------------------|---------------------------------|
| perft (startpos, depth 5)  | ~100M nodes/s (target ≥75M)     |
| batch replay               | ~1.48M games/s (target ≥500k)   |

```bash
cargo bench
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
cross-checked against independent oracles.

## License

MIT — see [LICENSE](LICENSE). All table data (Zobrist keys, magic numbers)
is either generated at runtime with fixed seeds or taken from public format
specifications.
