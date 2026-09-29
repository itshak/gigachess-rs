## MODIFIED Requirements

### Requirement: Undo SHALL Cache prev_checkers + Perft Slim

The system SHALL extend `Undo` with `prev_checkers:Bitboard` + `prev_zobrist:u64` and maintain `Board.checkers` + `history` in `make`/`unmake` (`checkers !=0` is `in_check` 0.32ns); `make_move_perft`/`unmake_move_perft` slim now also maintains `checkers` (`attackers_to` after turn flip) so `generate_moves_templated` can use cached `self.checkers` (saves 5 attacks ~20ns per movegen) for both `legal_moves()` and perft.

The refresh SHALL be documented as what it costs, measured rather than estimated. Over a real consumer's corpus of 11,149,374 games and 883,141,466 positions, the difference between a checkers-maintaining keyed walk and a checkers-free one is **3.92 s, 4.4 ns per ply** — about twice the ~2 ns per make the source comment assumed. The Polyglot hash updates in the same walk are 0.1–0.5 s, i.e. measurement noise. A caller that reads the key per ply and never asks `in_check()` — a position indexer — SHALL therefore not be made to pay the refresh, and the `make_move_perft` doc comment SHALL say that it maintains `checkers` (once per node) rather than skipping it, which is the opposite of what that comment claimed.

#### Scenario: in_check O(1) toward 0.32ns
- **WHEN** `board.in_check()` after `make_move` is called
- **THEN** it returns `self.checkers !=0` via `unsafe` load, `cargo bench --bench micro` `isCheck` 0.48→0.43 toward 0.32/0.33

#### Scenario: Make+unmake cycle vs ultrachess
- **WHEN** `make/unmake 48-ply` micro bench is run
- **THEN** turbo median < ultrachess 736ns on M1 Max (x86 1903 vs 2073 0.92×) via `piece_code_at_color` 6-scan + `pawn_code_at` + `CASTLE_CLEAR_STD` table, `perft` still wins

#### Scenario: The refresh price is measured, not assumed
- **WHEN** the same corpus is walked with the checkers-maintaining keyed make and with the checkers-free one, both maintaining the Polyglot hash
- **THEN** the difference is reported in seconds and in nanoseconds per ply, and the checkers share of it is stated
- **AND** the keyed, index-emitting pass is within 15 % of a moves-only pass (measured: +0.8 %).

### Requirement: Unmake SHALL Restore Without Recompute

The system SHALL restore `checkers` + `zobrist` from `Undo` on `unmake` without `attackers` recompute; perft uses slim path. Each make SHALL be paired with the unmake that shares its shape — `unmake_move` for the hashing and checkering makes, `unmake_move_fast` for `make_move_fast`, `unmake_move_perft` for `make_move_perft` — and a mismatched pairing SHALL be documented as silently rewriting derived state rather than failing, because that is what a caller hits.

#### Scenario: Make+unmake cycle vs ultrachess tradeoff
- **WHEN** `make/unmake 48-ply` micro bench is run
- **THEN** `ns/op` not regressed beyond known `503ns vs cozy 353ns` gap, and `perft position 3` depth `7` still correct

#### Scenario: Each make round-trips through its own unmake
- **WHEN** a legal move is made and unmade with each of the five make shapes, over the fixture positions and 200,000 positions from random playouts including two Chess960 starts
- **THEN** the position, and every piece of state the make claims to maintain, are restored exactly
- **AND** the make/unmake pairing table is walked explicitly, so the rule is stated once and checked in every corner.

## ADDED Requirements

### Requirement: Cached Checkers Shall Be Maintained Only for Callers That Read Them

The system SHALL keep the cached `checkers` bitboard current on the make paths
whose callers read it through the branch-free `in_check()` — `play`,
`make_move_unchecked` and their null-move counterpart — and SHALL offer a make
that skips the refresh for callers that do not (`make_move_hashed`,
`make_null_move_hashed`), documented on each function.

The refresh cost SHALL be recorded as measured, not as estimated: over a real
consumer's corpus of 11,149,374 games and 883,141,466 positions, the difference
between the checkers-maintaining and the checkers-free keyed walk is **3.92 s,
4.4 ns per ply**, against the ~2 ns per make previously assumed in the source
comment. A caller that needs the Polyglot key per ply and never asks
`in_check()` — a position indexer building a postings index over a whole
database — SHALL NOT pay it, and the keyed conversion it enables SHALL stay
within 15 % of a moves-only pass.

#### Scenario: The checkers refresh is measured, not assumed

- **WHEN** the same corpus is walked once with the checkers-maintaining make and
  once with the checkered-free one, both maintaining the hash
- **THEN** the difference is reported in seconds and in nanoseconds per ply
- **AND** the keyed, index-emitting pass is within 15 % of a moves-only pass.

#### Scenario: A caller that never reads the cache is not asked to maintain it

- **WHEN** a caller replays a corpus with `play_hashed` and reads only
  `zobrist()`
- **THEN** `zobrist()` equals `zobrist_full()` at every position
- **AND** `in_check()` is documented as stale for that make.
