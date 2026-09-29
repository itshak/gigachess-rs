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