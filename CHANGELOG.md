# Changelog

All notable changes to **GigaChess** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.6] - 2026-09-29

### Changed
- SAN disambiguation asks each candidate directly instead of generating every legal move in
  the position. `move_to_san_body` used to run a full `generate_moves_into` whenever a second
  piece of the same role attacked the destination square — 4.0 % of moves over a 13,908,447-ply
  slice of ChessBase's Mega Database 2025 — and filter that list for the two or three squares it
  wanted. The `attackers_bb` pre-filter already decides pseudo-legality exactly for the roles that
  reach the branch, so each surviving square is now asked directly: one 144B stack copy, one
  `make_move_unchecked`, one king-safety query for **the mover's** king against the side that
  moves next, one `unmake_move`. The candidate set, and therefore every SAN, is unchanged: 0
  differing moves over that slice against the old filter, and
  `tests/san_disambiguation_property.rs` asserts it over 200,000 moves from random playouts
  (standard and two Chess960 starts) against a movegen oracle.
  In-crate `cargo bench --bench micro` `san_48` moves −3.2 % on a busy middlegame and +1.9 % on an
  opening line, where the branch almost never fires; the consumer (`cbh-parser`'s whole-database
  export) is where the branch rate is real.

### Added
- `tests/san_disambiguation_property.rs`: a movegen oracle for the disambiguation, over 200,000
  moves of random playouts, plus the qualifier shapes the direct query has to get right.
- `tests/san_disambiguation.rs`: the two sharp edges as cases — a defender on the king's own file
  is not an attacker (the move the study of this branch first got wrong, which silently dropped
  hints), and a promotion carries only its origin file.

## [0.1.5] - 2026-09-28

### Added
- Splittable SAN rendering: `san::move_to_san_body(board, mv) -> Option<San>` renders everything
  of a move's SAN except the check/mate suffix, and `san::check_mate_suffix(after: &Board) ->
  Option<char>` returns that suffix for a position the caller has already reached — `Some('#')` when
  the side to move is checkmated, `Some('+')` when it is in check with a legal reply, `None`
  otherwise. `move_to_san` is exactly the two composed and is byte-identical to its previous
  output, so no existing caller changes and nothing about the notation rules moved. A caller whose
  walk makes the move anyway — a replay engine, a database exporter — no longer pays for a 144B
  `Board` copy, a second `make_move_unchecked` and an `unmake_move` per SAN: the suffix costs the
  O(1) cached `in_check()` (0.32 ns) and, only in check, `count_legal_moves()`. The after-position
  must be reached through a make that maintains `checkers` (`make_move_unchecked`,
  `make_move_perft`, `play`); the v0.1.4 fast makes leave that cache stale and must not be used
  with it. Covered by `tests/san_split_property.rs` (100,000 positions from random playouts across
  the standard start and Chess960 starts 284 and 518, asserting the split equals the monolith) and
  by unit tests for the quiet, `+`, `#`, promotion-with-check, disambiguation and castling shapes.


## [0.1.4] - 2026-09-28

### Added
- Fast move execution primitives: `Board::play_fast(mv) -> Result<Undo, IllegalMove>`,
  `Board::make_move_fast(mv) -> Undo`, and `Board::unmake_move_fast(mv, undo)`. Allows high-throughput
  move replay and legality verification while skipping incremental Polyglot Zobrist hashing and
  checkers bitboard recalculation. Preserves 100% bit-for-bit parity with standard `play` on piece
  placement bitboards, turn, castling rights, en-passant square, halfmove clock, and fullmove number.
  Covered by property-based testing across 100,000 positions in `tests/play_fast_property.rs`.


## [0.1.3] - 2026-09-27

### Added
- Null-move pass primitive: `Board::make_null_move() -> Result<Undo, IllegalMove>` and
  `Board::unmake_null_move(Undo)`. A pass flips the side to move without moving a piece:
  placement and castling rights untouched, en-passant square cleared, halfmove clock advanced,
  and the full move completed (the number advances, whoever passed). Incremental Polyglot hash
  and cached checkers are maintained, so `zobrist() == zobrist_full()` across the pass. A null
  move in check returns `IllegalMove`. CBH null-move tokens (`0xffff` in `moves2`) decode onto
  this primitive. Covered by `tests/null_move.rs` (pass semantics, EP clearing, move-number
  completion for both colors, in-check refusal, make/unmake round-trip with hash parity) and by
  the `turbochess-rs-core-engine` spec's null-move requirement.

## [0.1.2] - 2026-09-04

### Changed
- Generalized database indexing codecs, benchmark documentation, and internal comments.
- Polished README marketing, Criterion benchmarks against ultrachess, shakmaty, and cozy-chess.
- Added dual-ecosystem cross-referencing between Rust and JavaScript/TypeScript libraries.

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
