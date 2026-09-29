## The equivalence argument

The movegen filter answers two questions at once: *which squares could hold an
alternative* (the pre-filter) and *which of those alternatives is legal* (the
list). The pre-filter is exact for the first, so the direct form only has to
answer the second, and it can do so one candidate at a time.

**The pre-filter is exact pseudo-legality.** `attacks_from_target(to)` computed
over the live occupancy gives, for a slider, exactly the squares from which the
piece has an unobstructed ray to `to` (the ray stops at the first blocker, so a
piece behind one is excluded — correctly, it cannot reach `to`). For a knight
and a king the table is blocker-free by definition. A pawn never reaches the
branch: a pawn capture is disambiguated by its origin file and written before
any candidate work. So every square in `attackers_bb` yields a pseudo-legal
move, and no square outside it can.

**Legality is the mover's king, against the side that moves next.** A candidate
is legal iff the move is pseudo-legal and the mover's king is not attacked
afterwards. That is exactly what the legal-move generator filters on, so the
two sets coincide. The colours come from `board.turn()` — read from the
caller's board, *before* the candidate is made — and the attacker's colour is
`us.other()`.

> This is the one detail that is easy to get wrong, and the `cbh-parser` study
> of this branch got it wrong first: asking about the king of the side to move
> *after* the candidate, attacked by that same side's own pieces, is a question
> with no meaning — the rook on h1 "defends" its own king on e1 — and it drops
> hints the notation requires. Measured there: 83,337 of 419,385 games exact
> against 407,350, all extra differences of the over-disambiguation kind. The
> `debug_assert` in `move_to_san_body` already spells the right form out, which
> is why the implementation now uses that exact expression.

**Promotion equality** is carried by constructing the candidate with
`mv.promotion()`: a promotion candidate only ever matches a promotion move of
the same role, and no promotion can arise for the roles that reach the branch
except through a promoted piece already on the board, which `same_bb` catches.

**Castling** needs no care: a castling move is encoded king-to-own-rook and is
handled by the `is_castle` branch before any disambiguation, so no rook square
can be a disambiguation target.

**Chess960** is covered because the query is expressed in the same
`attacks_from_target` vocabulary as the pre-filter that already runs there, and
the property test includes two Chess960 starts.

## The cost model

| | per ambiguous move | per SAN body (4 % hit rate) |
|---|---|---|
| movegen filter (today) | one full legal movegen: ~35 candidates × pseudo-legality + king safety, plus `MoveList` fill | 13.5 ns |
| direct query | ≤ 8 × (144B copy + `make_move_unchecked` + one `attackers_to` + `unmake_move`) | 8.1 ns |

The historical reason for the movegen form (a comment in `san.rs`) was that
per-candidate testing "pays hash + checkers per candidate". In a *replay* the
caller has already made every move with `make_move_unchecked`, so the hash and
the `checkers` cache are warm and the per-candidate make is the same
maintenance the movegen already performed; the win is that we ask about two
squares instead of thirty-five.

## Alternatives considered

- **Keep the movegen and add a second public entry point** (`san_body_cached` for
  callers that keep the cache warm). Rejected: it leaves two code paths that must
  agree, and the direct form is not slower for a cold caller either — at most
  eight makes, each cheaper than one movegen, and the common case never enters
  the branch.
- **Keep the movegen only for positions with more than two candidates.** Rejected:
  two is where the movegen already costs more than two makes; the branch is
  cheap enough that a special case adds a path to test for no gain.
- **Drop the eight-candidate cap.** Rejected: a cap is already part of the
  contract, and the qualifier decision does not need more than eight squares.

## Risks and gates

| risk | gate |
|---|---|
| the candidate set drifts from the movegen's | `tests/san_disambiguation_property.rs`: a movegen oracle over 200,000 positions (standard + two Chess960 starts), every legal move |
| a notation regression in an edge case | existing `tests/san_parity.rs` (against `shakmaty::SanPlus`), `tests/san_disambiguation.rs`, `tests/chess960.rs`, perft |
| a performance regression elsewhere | `cargo bench --bench micro` before and after, with the frozen baseline in `benches/results/` |
| the consumer's bytes change | `cbh-parser`'s gold comparison (419,385 games) after the version bump |

## Change surface

`src/san.rs`, one branch of `move_to_san_body`. No public API, no new type, no
dependency. `ArrayVec<Square, 8>` is already imported for the cap; the loop
reads bits with `trailing_zeros`, the same idiom the rest of the crate uses.
