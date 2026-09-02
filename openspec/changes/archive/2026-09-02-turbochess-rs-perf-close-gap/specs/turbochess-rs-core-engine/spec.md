## MODIFIED Requirements

### Requirement: Sliding Attacks SHALL Execute via PEXT with Fancy Magic Fallback

The system SHALL implement sliding piece move generation using hardware `_pext_u64` (when compiled with the `pext` feature on BMI2 CPUs) and a compact Fancy Magic lookup table on non-PEXT architectures. The system SHALL also expose a colour-templated path `generate_legal_templated::<const WHITE: bool>` and a `MoveVisitor` path for perft visitor counting, both monomorphised and `LTO=fat`.

#### Scenario: Perft node count parity
- **WHEN** perft is evaluated on startpos and Kiwipete positions via `MoveVisitor` or `MoveSink`
- **THEN** node counts match standard reference counts at depths 1 through 6 (e.g. startpos depth 5 = 4,865,609, Kiwipete depth 4 = 4,085,603)

### Requirement: Move Representation SHALL Use 16-bit Packed Encoding

The system SHALL represent individual moves as a 16-bit struct (`u16`) where:
- Bits 0..5 (6 bits): `from` square (0..63)
- Bits 6..11 (6 bits): `to` square (0..63)
- Bits 12..15 (4 bits): promotion role (0=none, 1=Knight, 2=Bishop, 3=Rook, 4=Queen)

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

### Requirement: A Comparative Benchmark Suite SHALL Measure Best-in-Class Libraries

The system SHALL ship a Criterion benchmark suite measuring turbochess-rs head-to-head against shakmaty 0.30 and cozy-chess (both as dev-dependencies) on these axes: legal move generation, perft (with and without bulk counting, plus visitor), board copy, make-move, FEN parsing and formatting, SAN parsing and rendering, Zobrist hashing (from-scratch and incremental), and movetext/moves2 import plus hash replay (the latter against a shakmaty-based baseline mirroring blind-base's current implementation). Results SHALL be published in README.md with machine context, and turbochess-rs SHALL meet or beat both reference libraries on every axis; published best-in-class non-Rust figures (e.g. Stockfish 400-500 Mnps, Gigantua 2.1 Gnps CPOL) SHALL be included as stretch-target context with licensing notes (Gigantua CPOL, Stockfish GPL-3, not copied).

#### Scenario: Head-to-head results published
- **WHEN** the benchmark suite is run on the reference machine
- **THEN** README.md contains a results table covering every listed axis with turbochess-rs, shakmaty, and cozy-chess numbers, with turbochess-rs at least as fast as both on each axis or the gap explicitly documented with a follow-up issue

#### Scenario: Best-in-class context
- **WHEN** perft throughput is evaluated
- **THEN** the results table includes published best-in-class non-Rust reference figures (e.g. Stockfish 400-500 Mnps, Gigantua 2.1 Gnps) as the stretch target with MIT compliance notes
