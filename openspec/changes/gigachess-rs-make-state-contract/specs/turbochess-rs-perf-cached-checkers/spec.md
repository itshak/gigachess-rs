## MODIFIED Requirements

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