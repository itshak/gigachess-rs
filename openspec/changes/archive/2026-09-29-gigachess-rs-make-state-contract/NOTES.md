# Archive notes — gigachess-rs-make-state-contract

## What landed

The make declares the state it maintains. A make keeps two derived facts beyond
the position — the incremental Polyglot `hash` and the cached `checkers`
bitboard — and the crate previously offered only "both" (`play`) and "neither"
(`play_fast`). The two missing corners are now public (`play_hashed`,
`play_checkered`, plus `make_null_move_hashed`/`_checkered`/`_fast`), chosen by
const generics so the half a caller does not read is removed at compile time.

## Measurements (cbvault consumer, Mega Database 2025)

11,149,374 games, 883,141,466 positions, one thread, identical record plumbing:

| pass | make | wall clock | ns/ply |
|------|------|-----------|--------|
| moves only | `play_fast` | 42.92 s | 48.6 |
| + key + 8 B/position | `play` | 47.18 s | 53.4 |
| + key + 8 B/position | `play_hashed` | 43.26 s | 49.0 |

The `checkers` refresh costs 3.92 s over 883 M positions — **4.4 ns/ply**, about
twice the ~2 ns the source comment assumed — and the incremental hash is noise.
An indexed conversion therefore drops from +9.9 % to **+0.8 %** of a moves-only
pass. The SAN path needs `checkers`, so the export does not benefit; its gold
parity is unchanged (407,350 of 419,385 exact, 0 read errors).

## Two defects found and fixed

1. **A pass could be decided by a stale cache.** `make_null_move_with` — the body
   behind three of the four null-move variants — tested legality with
   `in_check()`, a read of the `checkers` cache. After a cache-free make, three
   of four variants allowed a pass while the side to move was in check. All four
   now use a fresh `attackers_to`.
2. **The regression test for (1) was vacuous.** Its fixtures never left the mover
   in check, so it compared two agreeing `true`s. Rebuilt around two positions
   that do, with the premise asserted, and verified to fail if the fix is
   reverted.

## Carried forward (deliberately not closed)

- **3.3 — publish 0.1.7 to crates.io.** An owner step: a public release is not
  automatic. Blocked only on that decision, not on work.
- **3.4 — a dedicated tree-wide `cargo fmt` pass.** The repository carried 86
  rustfmt diffs at HEAD (20 in `src/board.rs`) before this change. This change
  added none, and kept the reformat out so the functional diff stays reviewable.

## Follow-on

`null-move-first-class` builds on this contract and re-verifies it. Its delta
carries this change's scenarios forward, so archiving it will not drop them —
the overlap between the two changes on the null-move requirement was real and
`openspec validate` caught it.
