## Why

`move_to_san_body` answers SAN disambiguation with a **full legal movegen of the whole position** — every move, each with its own king-safety test — and then filters that list for the two or three candidates it wanted. The pre-filter that guards it (`attackers_bb = same_type & !from & attacks_from_target(to)`) is already exact pseudo-legality, so the movegen is paying for legality the pre-filter has decided: on a position where two knights can reach d7, gigachess generates all ~35 legal moves to learn that one of the two knights is pinned or not.

The branch is rare — 4.0 % of moves over a 13,908,447-ply slice of ChessBase's Mega Database 2025 (558,028 moves) — but when it fires it dominates the SAN body. Measured on that slice by the `cbh-parser` consumer, rendering the same moves both ways: **13.5 ns per SAN body today, 8.1 ns asking each candidate directly, 1.68× — about 4.8 s off a whole-database export of 11.1 M records** (a 136 s single-threaded run).

The output does not change. Over the same slice the two forms produce **0 differing moves** (1,055,149 of them carry a disambiguation), and against ChessBase's own gold export the direct form still matches **407,350 of 419,385** games with the same 12,035 differences. This change is therefore a pure cost reduction, and the task is to get it without changing a byte.

## What Changes

- **`move_to_san_body`'s ambiguous branch asks each candidate directly.** Where it today runs `generate_moves_into` and filters the `MoveList`, it now walks `attackers_bb` and, per candidate, does one `make_move_unchecked` on a 144-byte stack copy, one king-safety query for **the mover's** king against **the side that moves next**, and one `unmake_move` — the same make/unmake the movegen performed, once per candidate instead of once per legal move.
- **The same candidate set, by construction.** The pre-filter is exact pseudo-legality for the four roles that reach the branch (a knight or king cannot be blocked; a slider on the set has an unobstructed ray to `to`; pawns are disambiguated by file and never reach it), the candidate is built with `mv.promotion()` so promotion equality is preserved, and the eight-candidate cap stays.
- **The `debug_assert` in `move_to_san_body` already spells out the exact question** (`tmp.attackers_to(tmp.king_square(board.turn()).0, tmp.turn(), tmp.occupied()) == 0`, with `board.turn()` read from the *caller's* board). The implementation adopts that expression, so the assertion and the code can no longer drift.
- **New tests**: the disambiguation property test (a movegen-based oracle over 200,000 positions from random playouts, standard and Chess960), and unit cases for the positions where the two forms are easiest to confuse — a pinned twin, a defender on the king's own file, a three-knight case that needs a rank, a promotion, and a Chess960 start.
- No API change, no new public function, no allocation beyond the `ArrayVec<Square, 8>` that already existed, no dependency change, MIT throughout.

## Capabilities

### New Capabilities
- None.

### Modified Capabilities
- `turbochess-rs-perf-fen-san`: the *SAN Write SHALL Reuse Tables Toward 1.43µs* requirement's disambiguation clause changes from "single `generate_moves_into` `MoveList`" to the direct per-candidate query, with the equivalence it rests on and the measured per-move cost.
- `turbochess-rs-core-engine`: a new requirement pins what the disambiguation *means* — that the candidate set is the set a legal-move query would produce, decided by the mover's king and the side to move next — so a later optimisation cannot quietly weaken the notation.

## Impact

- **Code:** `src/san.rs` — one branch of `move_to_san_body`, roughly twenty lines.
- **API:** unchanged. `move_to_san`, `move_to_san_body`, `check_mate_suffix`, `san_to_move`, `play_san` keep their signatures and their bytes.
- **Deps:** none.
- **Perf:** the SAN body 13.5 ns → 8.1 ns per move on the reference slice (1.68×); `cargo bench --bench micro`'s `SAN 48` row and the `SAN write` rows are the in-repo gate for no regression elsewhere.
- **Validity:** `tests/san_disambiguation_property.rs` (new, a movegen oracle over 200,000 positions) plus the existing `tests/san_parity.rs` against `shakmaty::SanPlus`, `tests/san_disambiguation.rs`, and the Chess960 suite.
- **Consumers:** `cbh-parser` picks this up by bumping `gigachess`; its gold comparison (419,385 games against ChessBase's own export) is the end-to-end gate.
