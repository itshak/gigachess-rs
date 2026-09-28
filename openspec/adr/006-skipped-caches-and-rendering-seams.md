# ADR-006: A Skipped Cache Is Part of the API, and Rendering Is Split at the Seam

- **Status:** Accepted
- **Date:** 2026-09-28
- **Deciders:** GigaChess Core Team
- **Applies to:** `Board::play_fast` / `make_move_fast` (v0.1.4),
  `san::move_to_san` / `move_to_san_body` / `check_mate_suffix` (v0.1.5), and any
  future "fast" primitive.

---

## Context and Problem Statement

Two related hazards showed up in a single consumer, a ChessBase database reader
walking 883,141,297 plies.

**1. The cached checkers is load-bearing, and `make_move_fast` does not maintain
it.** `in_check()` is `self.checkers != 0` — a cache read, 0.32 ns both ways. It
is refreshed in `make_move_unchecked` for +2 ns per make, and the move
generator reads it too (`generate_moves_templated`, `legal_moves`, perft, and
SAN disambiguation through `generate_moves_into`). A caller that walks with
`play_fast` and then asks *any* of those a question gets an answer computed
from a stale cache.

That is not hypothetical: the reader's SAN came out under-disambiguated in
**five games of 11,149,374** — `Rg6` where two rooks reach g6, and four more of
the same shape, eight bytes of wrong notation in total. Nothing in that reader's
test suite could see it, because the bug lived in a cache no test inspected.

**2. `move_to_san` hid a seam it should not have hidden.** The body needs the
position before the move; the check/mate suffix needs the position after it. The
monolith therefore fabricated the after-position for every SAN: a 144B `Board`
copy, a `make_move_unchecked`, an `in_check()` read, an `unmake_move`. For a
caller that already made the move — any replay engine, any database exporter —
all of it was redundant work on the single hottest path in the consumer.

---

## Decision

1. **A primitive that skips a maintained field declares it, and the field is
   documented as unreadable afterwards.** `make_move_fast` / `play_fast` skip the
   Zobrist update and the checkers refresh, save ~2 ns per make, and leave
   `checkers` **stale**. Their doc comments say so in those words, and
   `Board::in_check` names the cache and the makes that maintain it. A stale
   `checkers` reads as "not in check" — not as a conservative answer.

2. **Consumers declare what they read.** The reader's move sink now answers
   `wants_checkers()` (and `wants_zobrist()`), and the walk makes moves with
   `Board::play` whenever either is wanted. The contract is a method on the
   consumer's trait, not a comment in this crate's source: the hazard is only
   knowable by the caller, who knows whether it will generate moves later.

3. **Rendering is split at the seam, and the monolith is their composition.**
   `san::move_to_san_body(board, mv)` renders everything but the suffix;
   `san::check_mate_suffix(after)` answers `'+'` / `'#'` / `None` for a position
   the caller already reached; `move_to_san` is the two composed and is
   byte-identical to v0.1.4 for every legal move. One implementation of the
   suffix rule, shared by both entry points, so they cannot drift.

4. **Every seam and every fast twin gets a property test.** `move_to_san` vs
   `move_to_san_body` + `check_mate_suffix` over 100,000 positions from random
   playouts across the standard start and Chess960 284/518
   (`tests/san_split_property.rs`); `play_fast` vs `play` over 100,000 positions
   (`tests/play_fast_property.rs`); the stale-cache edge is pinned by
   `a_stale_checkers_cache_would_be_a_stale_suffix` in `src/san.rs`.

---

## Consequences

### Positive

- The reader renders correct minimal SAN over its whole reference database, and
  its export got ~10 % faster single-threaded and ~25 % faster at ten threads
  for `+2 ns` per make.
- A consumer in a hot loop that needs neither the hash nor the checkers keeps
  `play_fast`'s full saving, with the hazard now written down at the primitive.
- The split is general: any walker with a post-move hook (search, verification,
  export) stops paying for a second make/unmake per node it visits.

### Negative / Trade-offs

- Two public entry points where there was one, and a sharp edge on the suffix
  function: it is only valid on a board reached through a checkers-maintaining
  make. The doc comment is emphatic, and a test holds it.
- Consumers that do read the cache now pay +2 ns per make. That is the point of
  the decision, not an accident of it.

---

## References

- ADR-001 (Maximum Performance and Native API) — zero-allocation, compact
  primitives
- ADR-002 (Parallel Replay with Rayon)
- ADR-005 (All-Axis Maximum Performance) — the cached-checkers decision that
  makes the generator's O(1) read possible
- `CHANGELOG.md` 0.1.5 (the split) and 0.1.4 (the fast makes)
- `tests/san_split_property.rs`, `tests/play_fast_property.rs`
