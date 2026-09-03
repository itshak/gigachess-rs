# Changelog

All notable changes to **GigaChess** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.1] - 2026-09-04

### Changed
- Updated crate description to highlight 540M nodes/s perft throughput and all-axis performance leadership.

## [0.1.0] - 2026-09-04

### Added
- **100% Permissive MIT Engine**: Ultra-high-performance chess move generation and perft engine designed for database workstations and search backends.
- **Native Bitboards**: `u64` bitboard operations with precomputed $64 \times 64$ `BETWEEN` and `LINE` ray lookup tables.
- **Dual Sliding Attack Architecture**: Hardware BMI2 `PEXT` acceleration (`pext` feature) with compact Fancy Magic fallback (~800 KB rook + 41 KB bishop tables).
- **Zero-Allocation Movegen**: Legal and pseudo-legal move generation strictly utilizing stack arrays (`ArrayVec<Move, 256>`).
- **Record Perft Throughput**: 540M nodes/second on Apple Silicon (`Board::perft(5)` in 9.04 ms); depth-1 leaf counting at 700M nodes/sec (28.5 ns).
- **16-Bit Packed `moves2` Format**: Wire-format binary encoding (`from | to << 6 | promo << 12`).
- **Parallel Batch Replay**: Multi-threaded game stream replay engine via Rayon work-stealing pool (1.41M games/sec, 213M plies/sec).
- **Incremental Polyglot Zobrist**: 64-bit hashing updated in $O(1)$ (<3 ns per ply) with direct single-cycle register cache reads (476 ps).
- **Zero-Allocation SAN Parser**: Targeted reverse attacker queries parsing algebraic moves in under 700 ns without heap allocations.
- **Full Chess960 (FRC/DFRC) Support**: Path-based castling legality (including adjacent king+rook swap castling) with compile-time `CASTLE_PATH: [[u64; 8]; 8]` bitmask tables.
- **Batch Codecs**: Streaming PGN `movetext → moves2` import lexer (3.03M plies/s) and `moves2 → SAN` exporter.
