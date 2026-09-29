## ADDED Requirements

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
## MODIFIED Requirements

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
