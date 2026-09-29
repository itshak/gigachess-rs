## Purpose

Defines the core `turbochess-rs` engine architecture: native `u64` bitboards, hardware PEXT / Fancy Magic sliding attacks, 16-bit packed moves (`moves2`), batch game replay, and 64-bit incremental Zobrist hashing.

## Requirements

### Requirement: Null Move SHALL Pass the Turn as a Board Primitive

The system SHALL expose `Board::make_null_move() -> Result<Undo, IllegalMove>` and `Board::unmake_null_move(Undo)` as the null-move (pass) primitive: placement and castling rights untouched, side to move flipped, en-passant square cleared, halfmove clock advanced, and the full move completed (the number advances, whoever passed — after the pass White is to move at the next number). The incremental Polyglot hash and the cached `checkers` SHALL be maintained by that entry point, so `zobrist()` still equals `zobrist_full()` and `in_check()` stays branch-free. A null move with the side to move in check SHALL return `IllegalMove`. Unmake SHALL restore the exact prior position when paired like `unmake_move` (no ordinary moves interleaved past the matching make).

The pass SHALL also be available in the same three other maintenance shapes as an ordinary make, so a caller is never forced to pay for state it does not read: `make_null_move_hashed` (hash maintained, `checkers` **stale**), `make_null_move_checkered` (`checkers` maintained, `zobrist()` **stale**) and `make_null_move_fast` (neither maintained, and the in-check test answered from the bitboards rather than from the cache). All four SHALL produce the same position when they allow the pass, and SHALL refuse it in exactly the same positions.

#### Scenario: Pass flips the side and preserves the placement
- **WHEN** `make_null_move` is played from a position where the side to move is not in check
- **THEN** the placement and castling rights are unchanged, the side to move is flipped, the en-passant square is cleared, the halfmove clock advances, the full move completes, `zobrist()` equals `zobrist_full()`, and `in_check()` answers the new position

#### Scenario: Null move in check is illegal
- **WHEN** `make_null_move` is requested with the side to move in check
- **THEN** it returns `IllegalMove` and the board is unchanged

#### Scenario: Every pass variant allows and refuses the same positions
- **WHEN** a pass is requested in each of the four maintenance shapes, across 200,000 positions from random playouts
- **THEN** all four verdicts agree with each other and with a fresh `attackers_to` test
- **AND** each variant's unmake restores the prior position exactly.

### Requirement: Sliding Attacks SHALL Execute via PEXT with Fancy Magic Fallback

The system SHALL implement sliding attacks via `hardware _pext_u64` when `pext` feature on BMI2 else compact Fancy Magic fallback, with `colour-templated` `generate_legal_templated::<const WHITE: bool>` and `MoveVisitor` path, both monomorphised `LTO=fat`. `attacks::bishop/rook_attacks` SHALL be `#[inline(always)]` `get_unchecked` and `#[cfg(feature="pext")]` branch elided when `!pext`.

#### Scenario: Perft node count parity
- **WHEN** perft is evaluated on startpos and Kiwipete via `MoveVisitor` or `MoveSink`
- **THEN** node counts match standard reference at depths 1..6 (e.g. startpos d5=4865609)

#### Scenario: Attacks unchecked
- **WHEN** `cargo bench --bench micro` `movegen_one_shot` is run
- **THEN** median drops >10% vs baseline without `pext` branch (was 121ns) toward ultrachess 42ns

### Requirement: Move Representation SHALL Use 16-bit Packed Encoding

The system SHALL represent a move as a single `u16` in `moves2` wire format —
`from | (to << 6) | (promo << 12)` — convertible to and from `Move` without
allocation, and SHALL keep castling as king-to-rook squares (`e1h1`, `e1a1`)
for standard chess and Chess960 alike. The packed form SHALL be the currency of
the batch replay engine, so a `moves2` stream replays without materialising
anything per move.

#### Scenario: Packed moves round-trip and stay compact
- **WHEN** a `Move` is built for each kind of move — a quiet move, a capture, a
  double push, an en-passant capture, a promotion and both castlings — and
  converted to its `u16` word and back
- **THEN** every move round-trips to the identical `Move`
- **AND** every word fits in 16 bits, and a `moves2` stream of a whole game
  replays through `replay.rs` with no allocation.

### Requirement: Fast Move Execution SHALL Support Replay Without Hashing Overhead

The system SHALL expose `Board::play_fast(Move) -> Result<Undo, IllegalMove>`, `Board::make_move_fast(Move) -> Undo`, and `Board::unmake_move_fast(Move, Undo)` to allow high-throughput move execution and legality validation without computing or updating incremental Polyglot Zobrist hashes or checkers bitboards. The fast replay methods SHALL preserve 100% bit-for-bit parity with standard `play` across piece placement, castling rights, active turn, en-passant square, halfmove clock, and fullmove number.

#### Scenario: State parity between fast replay and standard play
- **WHEN** a legal move is played on a position via `play_fast` vs `play`
- **THEN** both boards SHALL have identical piece placement bitboards, color turn, castling rights, en-passant square, halfmove clock, and fullmove number.

#### Scenario: Illegal move rejection
- **WHEN** an illegal move or move leaving the king in check is executed with `play_fast`
- **THEN** it SHALL return `IllegalMove` and the board position SHALL be restored unaltered.

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

### Requirement: SAN Rendering SHALL Be Splittable Into Body and Post-Move Suffix

The system SHALL expose the two halves of SAN rendering separately, so a caller that already holds the position a move leads to never pays for a second make/unmake:

- `san::move_to_san_body(board: &Board, mv: Move) -> Option<San>` SHALL render everything of the move's SAN except the check/mate suffix — the piece letter, the minimal disambiguation, the capture `x`, the target squares and the promotion — from the position the move is played from, and SHALL return `None` when there is no piece on the move's origin square. It SHALL never end in `+` or `#`.
- `san::check_mate_suffix(after: &Board) -> Option<char>` SHALL return `Some('#')` when the side to move in the already-reached position `after` is checkmated, `Some('+')` when it is in check with at least one legal reply, and `None` when it is not in check. It SHALL read the O(1) cached `checkers` through `Board::in_check()` and SHALL run the `count_legal_moves()` mate test only when the position is in check.
- `san::move_to_san(board, mv)` SHALL remain `move_to_san_body` followed by the same suffix rule applied to a position it reaches by making the move on a copy, and SHALL return byte-identical output to its pre-split behaviour for every legal move. Both routes SHALL share one implementation of the suffix rule so they cannot diverge.

The system SHALL document that `check_mate_suffix` requires an after-position reached through a make that maintains the cached `checkers` — `make_move_unchecked`, `make_move_perft`, `make_move_checkered` or `play`/`play_checkered` — and SHALL NOT apply it to a position reached through `make_move_fast` or `play_fast`, which leave `checkers` stale. A SAN path that reads neither the key nor anything else the hash serves may use `play_checkered` and skip the Polyglot maintenance; it SHALL NOT use `play_hashed`, whose `checkers` are stale.

#### Scenario: Split rendering equals the monolith
- **WHEN** `tests/san_split_property.rs` renders every legal move over 100,000 positions from random playouts across the standard start and Chess960 starts 284 and 518
- **THEN** `move_to_san_body(board, mv)` with `check_mate_suffix(after)` appended equals `move_to_san(board, mv)` for every one of them, the body never ends in `+` or `#`, and the make/unmake around the after-position is clean

#### Scenario: The body alone carries no suffix
- **WHEN** the `src/san.rs` unit tests render a quiet move, a checking move, a mating move, a promotion with check, a disambiguated knight move and both castlings
- **THEN** `move_to_san_body` returns the notation without any suffix, `check_mate_suffix` on the made position returns `None`, `Some('+')` or `Some('#')` as the position dictates, and their concatenation equals `move_to_san`

#### Scenario: A stale checkers cache is not a valid after-position
- **WHEN** a move is made with `make_move_fast` or `play_fast` — whose `checkers` are stale by contract — and `check_mate_suffix` is read from that board
- **THEN** `in_check()` reads as "not in check", and the behaviour is pinned by the unit test `a_stale_checkers_cache_would_be_a_stale_suffix` rather than silently trusted; the same holds for `play_hashed` and `make_move_hashed`

#### Scenario: No regression in the notation contract
- **WHEN** `cargo test --all-features` runs
- **THEN** `tests/san_parity.rs` (byte parity against `shakmaty::SanPlus`), `tests/san_disambiguation.rs`, `tests/perft.rs`, `tests/chess960.rs`, `tests/zobrist.rs`, `tests/replay.rs`, `tests/null_move.rs`, `tests/play_fast_property.rs`, `tests/make_variants.rs`, `tests/make_variants_property.rs` and the 49 lib unit tests all pass, and `cargo clippy --all-targets --all-features` is clean

### Requirement: SAN Disambiguation SHALL Equal a Legal-Move Query

`move_to_san_body` SHALL emit a file, rank, or file-and-rank qualifier exactly when another piece of the same role and colour can **legally** reach the same square, and SHALL emit none when no such piece can. The candidate set therefore equals the set a full legal-move generation would produce for the position, and the qualifier is the minimal one: the file when no other candidate shares it, otherwise the rank when none shares that, otherwise both.

Legality of a candidate SHALL mean that the move is pseudo-legal (which the `attackers_bb` pre-filter already decides exactly for knights, kings and sliders) **and** that after it is made the *mover's* king is not attacked by the side to move next. Both colours SHALL come from the position before the candidate is made. A candidate built with a different promotion than the move being rendered SHALL not count, and the search SHALL stop after eight candidates.

#### Scenario: A pinned twin is not a candidate
- **WHEN** the position is `r1bq1b1r/ppp3pp/2n1k3/3np3/2B5/2N2Q2/PPPP1PPP/R1B1K2R b KQ - 3 8` and the move is c6-e7
- **THEN** the SAN is `Ne7` without a qualifier, because the other knight is pinned and a pinned piece does not reach the square legally

#### Scenario: A defender on the king's own file is not an attack
- **WHEN** a candidate is tested for legality in a position where the mover's king is defended by one of the mover's own pieces along a rank, and the test asks whether the side to move *after* the candidate attacks its own king's square
- **THEN** the candidate is judged on the mover's king against the opponent, so the defender is not mistaken for an attacker; a hint that the notation requires is not dropped

#### Scenario: The qualifier is minimal
- **WHEN** three knights can legally reach one square, two of them from the same file as the move and one from another
- **THEN** the moves from the shared file carry the rank and the third carries its file, and no move carries both when a file or a rank alone identifies it

#### Scenario: The query agrees with a movegen oracle
- **WHEN** `tests/san_disambiguation_property.rs` renders every legal move of 200,000 positions from random playouts across the standard start and Chess960 starts, against a reference implementation that filters `generate_moves_into`
- **THEN** the two agree on every move of every position

### Requirement: Move Execution SHALL Declare the State It Maintains

A make move in this crate maintains two pieces of derived state beyond the
position itself: the incremental Polyglot Zobrist `hash` and the cached
`checkers` bitboard. The crate SHALL provide all four combinations, and each
public entry point's documentation SHALL state which of `zobrist()` and
`in_check()` remains correct after it and which is stale and MUST NOT be read:

| entry point | hash | checkers |
|-------------|------|----------|
| `play` / `make_move_unchecked` | maintained | maintained |
| `play_hashed` / `make_move_hashed` | maintained | **stale** |
| `play_checkered` / `make_move_checkered` | **stale** | maintained |
| `play_fast` / `make_move_fast` / `make_move_perft` | **stale** | **stale** |

The four SHALL be instantiations of one implementation, selected so that the
work a caller does not need is removed at compile time rather than branched over
at run time. The position itself — piece placement, side to move, castling
rights, en-passant square and, for every variant except the perft path, the
clocks — SHALL be identical whichever variant made it.

#### Scenario: A position indexer pays for the hash only

- **WHEN** a caller replays a `moves2` stream with `play_hashed` and reads
  `zobrist()` after each ply
- **THEN** `zobrist()` equals `zobrist_full()` after every ply
- **AND** the key equals the key the hash-and-checkers variant produces for the
  same move.

#### Scenario: A SAN renderer pays for the checkers only

- **WHEN** a caller replays with `play_checkered` and reads `in_check()`
- **THEN** `in_check()` equals a freshly computed `attackers_to` test after every
  ply
- **AND** `zobrist()` is not maintained, so the caller MUST NOT read it.

#### Scenario: The variant does not change the position

- **WHEN** the same legal move is made with `play_hashed`, `play_checkered` and
  `play_fast` from the same position
- **THEN** the three resulting boards agree on placement, side to move, castling
  rights, en-passant square, halfmove clock and fullmove number

### Requirement: A Null Move Shall Not Trust a Cache Its Caller May Not Maintain

`make_null_move` decides legality by asking whether the side to move is in
check. That test SHALL be answered from the bitboards rather than from the
cached `checkers` bitboard whenever the caller does not guarantee the cache is
current, because a make that skips the cache refresh (`play_fast`,
`make_move_fast`, `make_move_perft`) leaves a stale value there and a null move
that reads it accepts or refuses the wrong positions. The crate SHALL therefore
provide a cache-free null move alongside the hashing and checkered ones, and all
of them SHALL refuse a pass by a side to move that is in check, and SHALL
produce the same position when they allow it.

#### Scenario: A pass is refused in check, whichever variant is asked

- **WHEN** the side to move is in check and a null move is requested
- **THEN** `make_null_move`, `make_null_move_hashed`, `make_null_move_checkered`
  and `make_null_move_fast` all return `IllegalMove`.

#### Scenario: A cache-free walk does not decide the pass from a stale cache

- **WHEN** a legal move is played with `play_fast` — leaving `checkers` stale —
  and a null move is then requested
- **THEN** `make_null_move_fast` and `make_null_move` reach the same verdict
- **AND** that verdict equals "allowed" exactly when the side to move is not in
  check according to a fresh `attackers_to` test.

#### Scenario: An allowed pass leaves the same position

- **WHEN** a null move is allowed and made with `make_null_move_fast`
- **THEN** the resulting position equals the one `make_null_move` produces, with
  the side to move flipped, the double-push square cleared and the halfmove clock
  advanced.
- **AND** `make_move_perft` agrees on everything except the clocks, which it
  documents that it skips.

#### Scenario: Legality is still validated

- **WHEN** a pseudo-legal move that leaves the mover's king attacked is passed to
  `play`, `play_hashed`, `play_checkered` or `play_fast`
- **THEN** each returns `IllegalMove` and leaves the board exactly as it was.
