## Purpose

Defines the core `turbochess-rs` engine architecture: native `u64` bitboards, hardware PEXT / Fancy Magic sliding attacks, 16-bit packed moves (`moves2`), batch game replay, and 64-bit incremental Zobrist hashing.

## Requirements

### Requirement: Sliding Attacks SHALL Execute via PEXT with Fancy Magic Fallback

The system SHALL implement sliding attacks via `hardware _pext_u64` when `pext` feature on BMI2 else compact Fancy Magic fallback, with `colour-templated` `generate_legal_templated::<const WHITE: bool>` and `MoveVisitor` path, both monomorphised `LTO=fat`. `attacks::bishop/rook_attacks` SHALL be `#[inline(always)]` `get_unchecked` and `#[cfg(feature="pext")]` branch elided when `!pext`.

#### Scenario: Perft node count parity
- **WHEN** perft is evaluated on startpos and Kiwipete via `MoveVisitor` or `MoveSink`
- **THEN** node counts match standard reference at depths 1..6 (e.g. startpos d5=4865609)

#### Scenario: Attacks unchecked
- **WHEN** `cargo bench --bench micro` `movegen_one_shot` is run
- **THEN** median drops >10% vs baseline without `pext` branch (was 121ns) toward ultrachess 42ns

### Requirement: Move Representation SHALL Use 16-bit Packed Encoding

The system SHALL represent moves as `u16` (`from|to<<6|promo<<12`) and `Board` SHALL be `#[repr(C)]` 144B `Copy` with `hash:u64` at offset 0, `checkers:u64` at 8, `bbs 96` at 16, `occ 16` at 112, `king_sq 2` at 128 (first cache line hot `hash/checkers`), `profile.release`/`bench` `lto=fat codegen-units=1 panic=abort`.

#### Scenario: Zero-allocation legal move generation
- **WHEN** `board.legal_moves()` is called
- **THEN** it returns an `ArrayVec<Move, 256>` allocated strictly on the CPU stack with 0 heap allocations

#### Scenario: Zero-allocation visitor perft
- **WHEN** `board.perft_visitor(depth)` via `CountingVisitor` is called
- **THEN** it counts leaf nodes without materialising `Move` values and without `pop_lsb`

### Requirement: High-Throughput Batch Replay Engine

The system SHALL provide `replay_moves2_batch` replaying slices of 16-bit encoded games in parallel, achieving ≥500,000 games/second across CPU cores. The per-ply hash SHALL be maintained incrementally (O(1)) using Polyglot keys with the Pseudo en-passant condition (ep key included iff a pawn of the side to move geometrically attacks the ep square), and castling keys SHALL be per (color, rook file) per `turbochess-rs-chess960`. Castling moves SHALL be decoded as king-from → own-rook-square (per `turbochess-rs-chess960`); no legal-movegen scan SHALL be required to decode a word.

#### Scenario: Batch replay matches FEN positions
- **WHEN** 100,000 games are replayed from binary `moves2` slices
- **THEN** final board hashes and legal statuses are verified with 100% parity

#### Scenario: Pinned en-passant hash condition
- **WHEN** a position has an en-passant capturer that is pinned (e.g. `8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1`)
- **THEN** the ep key SHALL still be included (Pseudo condition), yielding `0x83bf25e378cb17d0` as verified against python-chess

### Requirement: Board SHALL Be Copy and Support Pseudo-Legal Generation

The system SHALL make `Board` a `Copy` type (bit-for-bit copy semantics for engine hot paths) and SHALL expose `pseudo_legal_moves()` generating moves without king-safety filtering, for engines that apply their own legality handling. The system SHALL keep `Board:Copy` in both default and `compact` feature layouts.

#### Scenario: Copy semantics
- **WHEN** a `Board` is copied before making a move
- **THEN** the copy is a bit-for-bit snapshot and the original is unchanged after the move is applied to the copy

#### Scenario: Chess960 gate
- **WHEN** `board.is_chess960()==false` (standard)
- **THEN** `castle_rights_after` uses `CASTLE_CLEAR_STD[64]` table (2 loads) not 4-loop

### Requirement: A Comparative Benchmark Suite SHALL Measure Best-in-Class Libraries

The system SHALL ship a Criterion benchmark suite measuring gigachess head-to-head against shakmaty 0.30 and cozy-chess (both as dev-dependencies) on these axes: legal move generation, perft (with and without bulk counting, plus visitor), board copy, make-move, FEN parsing and formatting, SAN parsing and rendering, Zobrist hashing (from-scratch and incremental), and movetext/moves2 import plus hash replay. Results SHALL be published in README.md with machine context, and gigachess SHALL meet or beat reference libraries on these axes; published best-in-class non-Rust figures (e.g. Stockfish 400-500 Mnps, Gigantua 2.1 Gnps CPOL) SHALL be included as stretch-target context with licensing notes (Gigantua CPOL, Stockfish GPL-3, not copied).

#### Scenario: Head-to-head results published
- **WHEN** the benchmark suite is run on the reference machine
- **THEN** README.md contains a results table covering every listed axis with turbochess-rs, shakmaty, and cozy-chess numbers, with turbochess-rs at least as fast as both on each axis or the gap explicitly documented with a follow-up issue

#### Scenario: Best-in-class context
- **WHEN** perft throughput is evaluated
- **THEN** the results table includes published best-in-class non-Rust reference figures (e.g. Stockfish 400-500 Mnps, Gigantua 2.1 Gnps) as the stretch target with MIT compliance notes
