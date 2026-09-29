## Why

A make maintains two pieces of derived state beyond the position itself: the
incremental Polyglot hash and the cached `checkers` bitboard. The API offered
only two of the four combinations — `play`/`make_move_unchecked` (both) and
`play_fast`/`make_move_fast` (neither) — so a caller that wanted the hash but had
no use for `checkers` had to pay for both. `gigachess`'s own note put the
`checkers` refresh at ~2 ns/make; measured over a real consumer's whole
database, it is **~2× that**.

The consumer is `cbvault`, which converts a ChessBase `.cbh` set into a
position-indexed database: it reads `zobrist()` on every ply and never asks
`in_check()`. Four passes over that database (11,149,374 games, 883,141,466
positions, one thread, identical record plumbing per pass):

| pass | make | wall clock | ns/ply |
|------|------|-----------|--------|
| A | `play_fast` (neither) | 42.92 s | 48.6 |
| B | `play` (hash + checkers) | 46.96 s | 53.2 |
| C | `play` + 8 B/position written | 47.18 s | 53.4 |
| D | **`play_hashed` (hash only)** + 8 B/position written | **43.26 s** | **49.0** |

So the `checkers` cache costs **3.92 s over 883 M positions — 4.4 ns/ply**, and
the incremental hash costs 0.1–0.5 s, i.e. measurement noise. Dropping the
`checkers` refresh turns an indexed conversion from **+9.9 %** over a moves-only
pass into **+0.8 %**: the position index becomes, for practical purposes, free.

The work also found a real bug. `make_null_move` decides legality with
`self.in_check()`, which reads the cached `checkers`. A walker that keeps
neither cache makes its moves with `play_fast`, which leaves that cache stale —
so CBH's null-move token (`0xffff` in `moves2`) was validated against a stale
value, and a game with a pass turn could be accepted or rejected wrongly. This
is exactly ADR-003's failure mode: a fast primitive that skips a cache some
later call trusts.

## What Changes

- **The make is parameterised on what it maintains.** One body, four
  instantiations, selected by const generics so each compiles away the half it
  does not need: `make_move_unchecked_with::<HASH, CHECKERS>`. The public
  surface gains the two missing corners —
  - `play_hashed` / `make_move_hashed` — hash yes, `checkers` no,
  - `play_checkered` / `make_move_checkered` — `checkers` yes, hash no,
  - and the null-move counterparts `make_null_move_hashed`,
    `make_null_move_checkered`, `make_null_move_fast`.
- **Each variant's contract is documented on the function**, not in a comment
  elsewhere: which accessor stays correct, and which one is stale and must not
  be read. The existing `play_fast`/`make_move_perft` wording is brought into the
  same shape.
- **`make_null_move_fast` fixes the bug**: it answers the "may the side to move
  pass while in check?" question from the bitboards (`attackers_to`) rather than
  from the cache, which is what a cache-free walker needs.
- **Tests** (`tests/make_variants.rs`): each variant keeps the state it claims
  correct correct (the hashed one against `zobrist_full()` on positions that
  exercise a double push, an en-passant capture, a capture, a promotion and a
  king move out of check; the checkered one against a fresh `attackers_to`); all
  four produce the same position, with `make_move_perft` held only to the
  clocks-free core because skipping the clocks is its documented contract; all
  four validating entry points reject a move that leaves the king in check and
  leave the board untouched; a null move in check is refused by every variant;
  and the stale-cache regression — the fast null move must agree with the
  caching one after a `play_fast` — is pinned by name.
- No behavioural change to `play`, `play_fast`, `make_move_unchecked` or
  `make_move_perft`; no allocation; no dependency change; MIT throughout.

## Capabilities

### Modified Capabilities
- `turbochess-rs-core-engine` — the make/null-move state contract becomes four
  declared combinations instead of two, and the null move stops reading a cache
  its caller may not maintain.
- `turbochess-rs-perf-cached-checkers` — the refresh cost is measured at
  4.4 ns/make on a real consumer's corpus, not the ~2 ns previously assumed,
  and is now avoidable by declaring that the caller never reads it.

### New Capabilities
- None.

## Non-Goals

- No PGN writer change. The SAN path needs `checkers` (that is how it decides
  `+`/`#`), which is the *expensive* half, so `play_checkered` drops the hash it
  does not need and recovers only 0.1–0.5 s of a 135 s whole-corpus export —
  noise. The consumer's gold PGN comparison is re-run as a gate anyway and is
  unchanged (407,350 of 419,385 exact), because "no measurable gain" is only
  worth shipping once the output is proven identical.
- No removal of the existing names. `play`, `play_fast`, `make_move_unchecked`,
  `make_move_perft` and `make_move_fast` keep their behaviour and their callers.
- No flag object or runtime parameter: a caller that can say what it needs at
  the call site gets a monomorphised make, and the `checkers`/`hash` work
  disappears at compile time rather than behind a branch.

## Verification

- `cargo test --release` — 112 tests, 0 failures, including the nine new ones.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`.
- The consumer's whole-database corpus: pass D at 43.26 s against pass C at
  47.18 s, with the same keys (checksums equal) and the same games.
- The consumer's gold PGN comparison, re-run against the same binaries with the
  SAN path now taking `play_checkered`: **407,350 / 419,385 exact, 12,035
  classified diffs, 0 annotation read errors, 0 decode errors** — byte-identical
  to before this change.