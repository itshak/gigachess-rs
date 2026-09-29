# Review of `null-move-first-class` against gigachess 0.1.7 — resolved

- **Reviewed:** 2026-09-29, against `main` at `004c449` (tag `v0.1.7`)
- **Status:** all four findings accepted and closed. This file is the record of
  what the review found and where each point landed, so the reasoning survives
  next to the change it shaped. (It replaces a longer review of the same
  findings, whose sections interleaved during editing; nothing was lost.)

---

## What the change got right

1. **The problem is real and correctly scoped.** `0xffff` is `from=63, to=63,
   promo=15` — no legal move can produce it, which is what makes a word-level
   sentinel safe. `is_pseudo_legal(0xffff)` is `false` today, so the word is
   unreachable from legal enumeration and reserving it costs nothing.
2. **`legal_moves()` stays null-free**, so perft, search loops and every
   move-count budget are untouched.
3. **`--` renders with no suffix, always.** A pass gives no check, so
   `check_mate_suffix` must never be consulted. Matches the gold corpus, so it
   cannot move the 407,350/419,385 parity number.
4. **"A pass is a full move"** (`fullmove += 1` whoever passed) reuses the
   existing primitive rather than inventing half-move accounting.
5. **Exactly two input spellings** (`--`, `Z0`), nothing invented.
6. **The cross-repo contract is the point of the change** — four libraries must
   agree on one word and two spellings in / two out.

---

## Finding 1 — `is_legal(NULL) = !in_check()` re-opened the bug 0.1.7 had just fixed

`in_check()` reads the cached `checkers` bitboard, so that specification would
have decided a pass by reading a cache a preceding `play_fast` left stale.

**Measured** on `f3 e5 g4 Qh4#` walked with `play_fast` — cached `in_check()`
false, fresh `attackers_to` true:

| variant | allows the pass | correct? |
|---|---|---|
| `make_null_move` (cached) | yes | no |
| `make_null_move_hashed` (cached) | yes | no |
| `make_null_move_checkered` (cached) | yes | no |
| `make_null_move_fast` (fresh) | no | yes |

Three variants, not one. The 0.1.7 regression test could not have caught it:
its fixtures (`e2e4`, `g1f3`, `d2d4`) never leave the mover in check, so it
compared two agreeing `true`s — **vacuous coverage in a test written to guard
exactly this**.

**Landed:** gigachess 0.1.7 now runs the fresh `attackers_to` test in *every*
variant, and the test is rebuilt around two positions that do leave the mover in
check, asserts that premise, and covers all four variants on independent boards.
Verified to fail if the fix is reverted. The proposal carries the same rule
forward, and additionally requires `is_legal(Move::NULL)` to be answered by its
own branch — the generic body asks whether the *opponent's* king survives, which
returns `true` on exactly the positions that must return `false`.

## Finding 2 — the raw makes got no null dispatch, so `0xffff` corrupted silently

The proposal covered `play` and `play_fast`, not the four raw primitives that
performance-sensitive callers use.

**Measured** on `main`: `make_move_unchecked(0xffff)` aborts in debug on an
incidental empty-square assert and, in release, returns a board that has silently
lost Black's `k`-side castling right — with `zobrist() == zobrist_full()`, so
nothing downstream can detect it. The proposal's risk line "`0xffff` previously
errored everywhere" was false for the raw makes: they never errored, they
corrupted.

**Landed:** dispatch in all eight make/play entries plus `is_legal` and
`is_pseudo_legal`, with `debug_assert(!is_null)` in the shared generic body, plus
a castling-survival scenario.

## Finding 3 — the `unmake` side was unspecified

`unmake_move(0xffff)` returns a board equal to the start — by reading and
writing squares 63 as a phantom move. A layout coincidence, not a contract.

**Landed:** `Move::NULL` pairs exclusively with `unmake_null_move`;
`unmake_move` on a null word is undefined and asserts.

## Finding 4 — the TypeScript already accepted a third spelling, and its defect was worse than described

- `turbochess/src/chesstree.ts:15` is `new Set(["--", "Z0", "null"])` — the
  proposal rejects `null`, so the two libraries disagreed about what a legal
  token is, and the TS is what the frontend parses real PGN with.
- The null branch left `node.fen` at the *parent* FEN and never advanced
  `ctx.pos`/`ply`/`path`, so every later node in such a tree is one ply out of
  step with its FEN — not merely "no ply advance".
- `packedMove.ts:30` masks with `& 0xffff` and would pass the sentinel through
  as a normal word.

**Landed:** all three in the proposal's task 5.1 — narrow the set, add
`isNull`, compute the null-move FEN and advance the counters, emit `--`.

---

## Also reconciled

- **Delta overlap with 0.1.7.** 0.1.7 says the null-move primitive maintains
  hash and checkers; this change says `move_to_san_body(NULL)` returns `--`
  without making a move. Both hold, and the proposal now states the resolution:
  a SAN null is **cache-neutral** — it needs and disturbs neither. Task 5.2
  carries the reconciliation into archive.
- **blind-base's part is not a bug fix.** Its `replay_moves2_hashes` calls
  `board.play(Move::from_word(word))`, so null works there for free once Rust
  dispatches; the delegation is a dedup and a consistency change. The proposal
  now says so, so it is not prioritised as a correctness issue.
- **No interaction with the in-flight bridge work.** This change adds a `Move`
  word, not a make strategy. A pass maintains the hash, so the position index
  stays correct across one.
