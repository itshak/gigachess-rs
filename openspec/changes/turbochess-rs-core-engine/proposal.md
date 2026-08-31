# Proposal: TurboChess-RS Core Engine, PEXT Movegen, Moves2 Replay & Shakmaty Compat

## Why
High-throughput chess database workstations (such as `blind-base`) and search engines require extreme move generation throughput and binary replay capabilities. Currently, the Rust chess ecosystem is split between:
1. **`shakmaty`**: Feature-rich with Lichess variant support, but constrained by **GPL-3.0 viral licensing** and slower performance (~25–35M nodes/s due to heavy 24-byte `Move` enums and variant abstractions).
2. **`cozy-chess`**: Blazing fast and **100% MIT-licensed** (65M+ nodes/s with PEXT), but focused strictly on single-node minimax engines, lacking batch game replay pipelines, SAN disambiguation, and 16-bit database wire formats.

`turbochess-rs` bridges this gap. By building on `cozy-chess`'s MIT-licensed bitboard foundation and incorporating TurboChess's 16-bit `moves2` packed format, batch replay engine, and Shakmaty compatibility facade, `turbochess-rs` delivers a **100% MIT-licensed, 75M+ nodes/sec engine** that allows `blind-base` and proprietary applications to completely eliminate GPL dependencies.

## What Changes
- **Core Bitboards & Attacks (`src/bitboard.rs`, `src/attacks.rs`)**:
  - Native `u64` bitboard primitives.
  - Hardware `PEXT` (BMI2) 1-cycle sliding attacks on x86-64, with cache-compact Fancy Magic fallback for ARM / Apple Silicon.
  - Precomputed $64 \times 64$ ray and between lookup tables.
- **16-bit Packed Moves (`src/moves.rs`)**:
  - Compact `u16` move format (`word = from | (to << 6) | (promo << 12)`).
  - Stack-allocated legal move buffer (`ArrayVec<Move, 256>`) for zero-allocation move generation.
- **High-Throughput Batch Replayer (`src/replay.rs`)**:
  - Fast `moves2` stream player replaying 100,000 games in parallel across Rayon threads (>500,000 games/sec).
- **Zero-Alloc Incremental Zobrist (`src/zobrist.rs`)**:
  - Polyglot and Shakmaty-compatible 64-bit Zobrist hash table.
  - Incremental updates in `make_move` with $<3\text{ ns}$ latency per ply.
- **FEN & SAN Codecs (`src/fen.rs`, `src/san.rs`)**:
  - Branchless ASCII lookup table FEN parser.
  - Zero-allocation SAN disambiguation engine.
- **Shakmaty Compatibility Facade (`src/compat/shakmaty.rs`)**:
  - Drop-in API wrapper matching `shakmaty::Chess` and `shakmaty::Position`, enabling a 1-line swap in `blind-base/src-tauri`.

## Capabilities

### New Capabilities
- `turbochess-rs-core-engine`: Complete native Rust bitboard engine, PEXT sliding move generator, 16-bit `moves2` packed format, and batch replay pipeline.
- `turbochess-rs-shakmaty-compat`: Drop-in compatibility layer mirroring the `shakmaty` 0.30 API surface for zero-friction migration of existing Rust chess projects.

## Impact
- **Performance**: Projected Perft speed of **75–80M nodes/sec/core** (~2.3x faster than `shakmaty`).
- **License**: 100% Permissive MIT license across the entire codebase.
- **Memory**: Moves buffer takes 512 bytes on stack (zero heap allocation in move generation).
