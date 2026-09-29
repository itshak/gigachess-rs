# Changelog

All notable changes to **GigaChess** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.8] - 2026-09-29

### Added
- **The null move is a first-class move.** ChessBase (`.cbh`) databases encode a pass as the
  single word `0xffff` in the `moves2` stream. That word used to reach the board and be treated
  as an ordinary move with squares: it aborted on an incidental debug assert, and in release it
  moved whatever decoded out of `0xffff` and destroyed castling rights.
  - `Move::NULL` is the word, with `is_null()` — a single `u16` compare, and no legal move
    collides with it.
  - It dispatches in **all eight** make/play entries (`play`, `play_fast`, `play_hashed`,
    `play_checkered`, `make_move_unchecked`, `make_move_hashed`, `make_move_checkered`,
    `make_move_fast`), so a generic `play` loop over a `moves2` stream handles a pass without
    the caller knowing passes exist.
  - `is_pseudo_legal(Move::NULL)` is `true` — the sentinel is structurally well-formed, needing
    no piece, geometry or target. `is_legal(Move::NULL)` is answered by a **dedicated branch**.
  - SAN out is `--`, never suffixed; in, exactly `--` and `Z0`. UCI is `0000`.
  - `parse_movetext_to_moves2`, `moves2_to_san_movetext`, `replay_*` and `position_stats` all
    carry a pass, with no per-caller branches: they reach the transition through `play`.
  - `Move::NULL` pairs exclusively with `unmake_null_move`; `unmake_move` on a null word is
    undefined and asserts in debug.

### Fixed
- **`is_legal(Move::NULL)` would have been answered by the wrong question.** The generic
  make-and-test body asks whether the *mover's* king survives the move. A pass moves no king, so
  that body cannot express it — it ends up testing the *opponent's* king, which is safe, and
  returns `true` on exactly the positions where a pass must be refused. Verified on a
  stale-cache check position: the generic body says `true`, the correct fresh test says `false`.
  `is_legal` now has its own branch, and a test named after the hazard fails if it is removed.
- `make_null_move_fast` was a hand-copied duplicate of the null transition. All four variants now
  share one definition of what a pass does to a board.

### Notes
- The pass/refuse test is always a fresh `attackers_to`, never the cached `in_check()` — the same
  rule 0.1.7 established, now carried by all eight entry points. A test asserts the premise (the
  cache and the fresh computation genuinely disagree) before asserting the refusal.
- No breaking change: a pass was never in `legal_moves()` or any legal-move stream, so nothing
  that did not already carry `0xffff` can change behaviour.

## [0.1.7] - 2026-09-29

### Added
- **The make now declares the state it maintains.** A make maintains two things beyond the
  position: the incremental Polyglot `hash` and the cached `checkers` bitboard. The API offered
  only "both" (`play`, `make_move_unchecked`) and "neither" (`play_fast`, `make_move_fast`), so
  a caller that wanted the key but had no use for the checkers cache paid for both. The two
  missing corners are now available and cost nothing:
  - `play_hashed` / `make_move_hashed` — hash maintained, `checkers` **stale**.
  - `play_checkered` / `make_move_checkered` — `checkers` maintained, `zobrist()` **stale**.
  - Null-move counterparts: `make_null_move_hashed`, `make_null_move_checkered`,
    `make_null_move_fast`.

  All four are instantiations of one implementation, so the half a caller does not need is
  removed at compile time rather than branched over. Each entry point's docs state which
  accessor stays correct and which must not be read.

### Fixed
- **`make_null_move_fast`, and with it a latent correctness bug in every cache-free walker.**
  `make_null_move` decided legality with `in_check()`, which reads the cached `checkers`. A
  walker that keeps neither cache makes its moves with `play_fast`, which leaves that cache
  stale — so a CBH null-move token (`0xffff` in `moves2`) was validated against a stale value
  and a game with a pass turn could be accepted or refused wrongly. The new
  `make_null_move_fast` answers the in-check test from the bitboards instead.

### Fixed
- **A null move decided legality from a stale cache in three of the four variants.** `make_null_move_with`
  — the body behind `make_null_move`, `make_null_move_hashed` and `make_null_move_checkered` — tested the
  pass with `in_check()`, which reads the cached `checkers` bitboard. A caller that reached a pass after
  `play_fast` (or any other cache-free make) held a cache describing an earlier position, so the pass was
  **allowed while the side to move was in check**. Verified on `f3 e5 g4 Qh4#` walked with `play_fast`:
  the three cached variants allowed the pass, only `make_null_move_fast` refused it. Every variant now
  asks the bitboards with `attackers_to`, so all four refuse in exactly the same positions. The scan costs
  one `attackers_to` on a path taken roughly once per ten thousand plies.

  The regression test that was supposed to catch this was **vacuous**: its fixtures (`e2e4`, `g1f3`,
  `d2d4`) never leave the mover in check, so it compared two agreeing `true`s. It now uses two positions
  that do leave the side to move in check, asserts that premise, and checks all four variants on
  independent boards — and it fails if the fix is reverted.

### Changed
- The `make_move_perft` doc comment claimed it skips the `checkers` refresh; the code maintains
  it (once per node, which is what perft wants). The comment now matches the code, and says
  plainly that `zobrist()` is stale and the clocks are not advanced.

### Performance
Measured by the `cbvault` consumer over ChessBase's Mega Database 2025 — 11,149,374 games,
883,141,466 positions, one thread, identical record plumbing per pass:

| pass | make | wall clock | ns/ply |
|------|------|-----------|--------|
| A | `play_fast` (neither) | 42.92 s | 48.6 |
| B | `play` (hash + checkers) | 46.96 s | 53.2 |
| C | `play` + 8 B per position written | 47.18 s | 53.4 |
| D | `play_hashed` + 8 B per position written | 43.26 s | 49.0 |

The `checkers` refresh costs **3.92 s over 883 M positions — 4.4 ns/ply**, about twice the
~2 ns the source comment assumed; the incremental hash is 0.1–0.5 s, i.e. noise. A conversion
that writes a position key per ply therefore drops from **+9.9 %** over a moves-only pass to
**+0.8 %**. The SAN path keeps `checkers` (it is what decides `+`/`#`) and so gains only the
hash, which is noise; its output is byte-identical — the consumer's gold comparison stays at
407,350 of 419,385 exact, 0 read errors, 0 decode errors.

### Tests
- `tests/make_variants.rs` (10 tests): the hash contract against `zobrist_full()` across a
  double push, an en-passant capture, a capture, a promotion, castling and a check evasion; the
  checkers contract against a fresh `attackers_to`; position parity across all four variants
  (perft held to its documented clock-skipping contract); illegal-move rejection with board
  restoration on all four validating entry points; a pass refused in check by every null-move
  variant; the stale-cache regression; the canonical Polyglot start-position key; and the
  make/unmake pairing table.
- `tests/make_variants_property.rs` (1 test, 200,335 moves): the same questions asked of every
  position a random game passes through, from the standard opening and two Chess960 starts —
  1,005 castlings, 510 promotions, 9,016 en-passant captures. The fixtures cannot reach a
  Chess960 castle, which moves the rook file the castling keys are built from. It caught a real
  hazard on its first run: a fast make's `Undo` handed to `unmake_move` does not fail, it
  silently rewrites the key, so the pairing is now explicit in the docs, in the spec and in a
  test that walks all five shapes.

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
