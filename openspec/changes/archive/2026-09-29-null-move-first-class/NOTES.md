# Archive notes — null-move-first-class

## What landed

`Move::NULL` (`0xffff`) is a first-class move. It dispatches in all eight
make/play entries, `is_pseudo_legal` accepts it, `is_legal` answers it on its own
branch, SAN renders it as `--` and parses `--`/`Z0`, and the codecs carry it
end-to-end. A generic `play` loop over a `moves2` stream now handles a CBH game
containing a pass without the caller knowing passes exist.

## Two defects the work found

**1. `is_legal(Move::NULL)` would have asked the wrong question.** The generic
make-and-test body plays the move and asks whether the *mover's* king survives. A
pass moves no king, so there is no mover's king to expose and the only question
worth asking is whether the side to move is already in check. The generic body
cannot express that — it ends up testing the *opponent's* king, which is safe:

| on a stale-cache check position | answer |
|---|---|
| correct test (the mover's own king) | `false` — refuse the pass |
| generic body (the other king) | `true` — wrongly allow it |

`is_legal` now has a dedicated branch, and
`null_legality_is_decided_for_the_mover_not_the_opponent` fails if it is removed
(verified: reverting it fails 8 of the 33 fixture tests).

**2. The pass transition was defined twice.** `make_null_move_fast` was a
hand-copied duplicate of the transition, free to drift from the three variants
that shared a body. It is now one `null_transition::<HASH, CHECKERS>`, reached by
`make_null_move_with` and by the four raw make entries, so all eight entry points
apply the *same* pass.

## One thing deliberately not built

The design called for per-codec null branches in
`parse_movetext_to_moves2`, `moves2_to_san_movetext`, `replay_*` and
`position_stats`. They were not needed and were not added: every codec reaches
the transition through `play`, so the dispatch carries them for free. That is
design goal 2 — "generic replay loops work without per-caller null branches" —
satisfied rather than deviated from. The codec tests are what prove the claim
rather than a code-reading argument.

## Test design

Two suites, both verified non-vacuous by reverting the fix and watching them
fail:

- `tests/null_move.rs` — 33 fixture tests. Includes an all-65,536-combination
  proof that no legal move word collides with the sentinel, and a premise-first
  stale-cache test that asserts the cache and a fresh computation *disagree*
  before asserting the refusal.
- `tests/null_move_property.rs` — 200,000 sampled positions over random playouts
  from the standard start and two Chess960 starts. Measured: 668,700 plies,
  198,873 legal passes, 1,127 refused passes, **1,364 positions where the cached
  `checkers` disagrees with a fresh computation**, 397,746 refreshed caches
  verified, 232,164 hash parities checked.

An early version of the property test passed with the bug reverted. Its
"stale cache" probe did a `play_fast`/unmake round-trip, which *restored* the
cache, so it never once tested a stale one. It now breaks the caches with
`play_fast` and samples the pass path on the resulting board, and the coverage
counters are asserted at the end so it cannot pass by reaching none of the states
it claims to test.

A counter was **removed as unreachable rather than weakened**: a legal pass
cannot leave the new side to move in check, because a pass changes no pieces, so
the opponent's check status is unchanged — and a position where the side *not* to
move is in check is unreachable by legal play. That is a property of the
transition. What is verified instead is that the refreshed cache matches a fresh
computation for the new side to move, which is the work the refresh does.

## Validated against the real database

The Mega Database turned out to be on this machine, so task 4.2's corpus replay
was done rather than deferred.

`cbvault verify` over all 11,149,379 games (883,141,297 plies) in 8.5 s found
**2,975 null moves** and **5 failures — none of them null-move related**
("no Queen number 2", "encoding mode 1 is not supported" twice, "Chess960 game
without a start position", "tree not terminated"). That run used the *published*
gigachess 0.1.6, so it does not exercise the new dispatch.

So the null-bearing games were extracted (cbvault's `dump_null_games` example
writes the mainline `moves2` words of every game containing a pass) and replayed
through this crate's public API: **1,002 games, 75,501 plies, 1,208 passes**,
hash parity checked at every ply, every movetext round-tripped byte-identically,
and every recorded pass confirmed **legal** by this engine — a real-world
cross-check on the refusal rule, not just the acceptance path. 106 of the games
start from a set-up position, and one is literally a single ply: a pass and
nothing else.

A 28 KB sample of 62 games is committed as `tests/data/cbh_null_games.txt`, so
`tests/cbh_corpus.rs` keeps the check permanent without a 5.4 GB database.

### What the corpus suite does not cover, stated plainly

The replay uses `play` throughout, so `checkers` is always fresh and **deciding
a pass from a stale cache is structurally unreachable there** — reverting the
fresh test to the cached one leaves all four corpus tests green (verified). That
path is covered by the two synthetic suites, both verified to fail when the fix
is reverted. The corpus suite does catch a pass that fails to flip the turn
(3 of 4 fail) and one that omits the turn-key hash XOR (hash parity fails).

## Carried forward (not done here)

- **`cbvault` re-export and de-dupe** — re-export `gigachess::Move::NULL`, keep
  `NULL_MOVE` as an alias, delete the manual `--`/no-suffix/fullmove branches.
  Blocked on gigachess 0.1.8 being published, which is an owner decision. Not
  faked.

## Landed downstream

| repo | tag | what |
|---|---|---|
| TS `turbochess` | `v0.4.2` | `isNull` sentinel, `NULL_MOVE_SANS` narrowed to `["--","Z0"]`, and the real bug: `chesstree` did not advance position/ply/path across a pass, so every later node was one ply out of step and the exporter dropped the ply |
| `gigaboard` | `v1.4.4` | tolerates a pass, never originates one, schedules no piece animation |
| `blind-base` | — | frontend `playMove` and `next_move_at_ply` |

TS `turbochess` also got a test-gate fix found during verification: its suites
import from a gitignored `dist/` and `npm test` never built, so the whole gate
could pass on stale output — a reverted fix reported 95 passed until rebuilt.
`npm test` now builds first.
