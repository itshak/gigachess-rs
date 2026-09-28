## ADDED Requirements

### Requirement: SAN Rendering SHALL Be Splittable Into Body and Post-Move Suffix

The system SHALL expose the two halves of SAN rendering separately, so a caller that already holds the position a move leads to never pays for a second make/unmake:

- `san::move_to_san_body(board: &Board, mv: Move) -> Option<San>` SHALL render everything of the move's SAN except the check/mate suffix — the piece letter, the minimal disambiguation, the capture `x`, the target squares and the promotion — from the position the move is played from, and SHALL return `None` when there is no piece on the move's origin square. It SHALL never end in `+` or `#`.
- `san::check_mate_suffix(after: &Board) -> Option<char>` SHALL return `Some('#')` when the side to move in the already-reached position `after` is checkmated, `Some('+')` when it is in check with at least one legal reply, and `None` when it is not in check. It SHALL read the O(1) cached `checkers` through `Board::in_check()` and SHALL run the `count_legal_moves()` mate test only when the position is in check.
- `san::move_to_san(board, mv)` SHALL remain `move_to_san_body` followed by the same suffix rule applied to a position it reaches by making the move on a copy, and SHALL return byte-identical output to its pre-split behaviour for every legal move. Both routes SHALL share one implementation of the suffix rule so they cannot diverge.

The system SHALL document that `check_mate_suffix` requires an after-position reached through a make that maintains the cached `checkers` (`make_move_unchecked`, `make_move_perft`, or `play`), and SHALL NOT apply it to a position reached through `make_move_fast` or `play_fast`, which leave `checkers` stale.

#### Scenario: Split rendering equals the monolith
- **WHEN** `tests/san_split_property.rs` renders every legal move over 100,000 positions from random playouts across the standard start and Chess960 starts 284 and 518
- **THEN** `move_to_san_body(board, mv)` with `check_mate_suffix(after)` appended equals `move_to_san(board, mv)` for every one of them, the body never ends in `+` or `#`, and the make/unmake around the after-position is clean

#### Scenario: The body alone carries no suffix
- **WHEN** the `src/san.rs` unit tests render a quiet move, a checking move, a mating move, a promotion with check, a disambiguated knight move and both castlings
- **THEN** `move_to_san_body` returns the notation without any suffix, `check_mate_suffix` on the made position returns `None`, `Some('+')` or `Some('#')` as the position dictates, and their concatenation equals `move_to_san`

#### Scenario: A stale checkers cache is not a valid after-position
- **WHEN** a move is made with `make_move_fast` and `check_mate_suffix` is read from that board
- **THEN** the fast make has left `checkers` stale, `in_check()` reads as "not in check", and the behaviour is pinned by the unit test `a_stale_checkers_cache_would_be_a_stale_suffix` rather than silently trusted

#### Scenario: No regression in the notation contract
- **WHEN** `cargo test --all-features` runs
- **THEN** `tests/san_parity.rs` (byte parity against `shakmaty::SanPlus`), `tests/san_disambiguation.rs`, `tests/perft.rs`, `tests/chess960.rs`, `tests/zobrist.rs`, `tests/replay.rs`, `tests/null_move.rs`, `tests/play_fast_property.rs` and the 49 lib unit tests all pass, and `cargo clippy --all-targets --all-features` is clean
