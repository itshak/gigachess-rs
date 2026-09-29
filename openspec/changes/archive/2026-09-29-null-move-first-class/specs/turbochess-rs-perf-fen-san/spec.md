## ADDED Requirements

### Requirement: SAN SHALL Render and Parse the Null Move as ChessBase Does

The system SHALL render `Move::NULL` as SAN `--` with never a check/mate suffix: `move_to_san` and `move_to_san_body` SHALL return `--` for the null move without making any move, reading any piece, or appending `+`/`#`, regardless of whether the after-position is check. Rendering a null SHALL neither require nor disturb the hash or `checkers` caches (it makes no move, so no `wants_checkers`-style obligation attaches — reconciled with the state contract). `san_to_move` SHALL accept exactly the ChessBase spellings `--` and `Z0` (nothing else invented) and SHALL return `Move::NULL` when a fresh `attackers_to` test finds the side to move not in check, else `None` (never the cached `in_check()`). `play_san` SHALL play a null token through the null-move path. UCI SHALL spell the null move `0000`: `parseUci("0000")` returns `Move::NULL` and `makeUci(Move::NULL)`/`Display` renders `0000`.

#### Scenario: Null renders as dashes without suffix

- **WHEN** `move_to_san` or `move_to_san_body` is called with `Move::NULL` in any position
- **THEN** the result is `--` with no `+`/`#`, and no legality scan or board copy is performed

#### Scenario: Null rendering touches neither cache

- **WHEN** `move_to_san_body` renders `Move::NULL` on a board whose hash or `checkers` are stale (e.g. after a `*_fast` make)
- **THEN** the result is still `--` and both caches are bit-identical afterwards

#### Scenario: Null parses from ChessBase spellings only

- **WHEN** `san_to_move` is called with `--` or `Z0` outside check
- **THEN** it returns `Move::NULL`; in check it returns `None`, and any other invented spelling (e.g. `null`, `pass`) returns `None`

#### Scenario: Null SAN round-trips through movetext

- **WHEN** a game containing a null move is rendered and re-parsed
- **THEN** `--` parses back to `Move::NULL` and the `moves2` stream is byte-identical

#### Scenario: UCI null spelling

- **WHEN** `parseUci("0000")` is called
- **THEN** it returns `Move::NULL`, and rendering `Move::NULL` as UCI yields `0000`
