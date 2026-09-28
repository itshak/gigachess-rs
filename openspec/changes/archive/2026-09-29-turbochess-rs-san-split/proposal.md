## Why

`move_to_san(board, mv)` renders a SAN in two halves that need different inputs: the **body** (piece letter, minimal disambiguation, capture `x`, target squares, `=Q`) needs only the position *before* the move, while the **check/mate suffix** (`+` / `#`) needs the position *after* it. Today the function hides that split, so it has to fabricate the after-position itself: it copies the whole 144B `Board` onto the stack, calls `make_move_unchecked` (which recomputes `checkers` at +2 ns/make), reads `in_check()`, and unmakes — **per SAN**.

Every caller that already *has* the after-position therefore pays for a second make/unmake it did not need. The ChessBase reader (`cbh-parser`) is exactly that caller: its walker must make each move anyway to reach the next ply, and its `MoveSink::played(&Board)` hook already receives the post-move board. Over the 883,141,297 plies of the reference database that redundant copy + make + unmake is the single largest cost in an export that is otherwise allocation-free — `macOS sample(1)` of the whole-database export attributes ~13 % to `Board::make_move_unchecked` and a further ~9.5 % to `memmove` of SAN text.

The upstream MIT reader solved this long ago by splitting SAN into `write_san_body` (in the pre-move hook) and `write_check_suffix(after)` (in the post-move hook). We cannot copy their code (different license lineage, and their `write_san_body` is their own disambiguation); we can expose gigachess's existing, already-optimal body and suffix as two functions and let callers that hold the after-position skip the make/unmake.

## What Changes

- **`san::move_to_san_body(board, mv) -> Option<San>`** — the body, verbatim from today's `move_to_san` (lines 28-135): castling word, `attackers_bb` pre-filter + single `generate_moves_into` disambiguation, capture flag, target, promotion.
- **`san::check_mate_suffix(after: &Board) -> Option<char>`** — the suffix rule for a position the caller has *already* reached: `in_check()` (O(1) `checkers != 0`) then `count_legal_moves() == 0` → `'#'`, `'+'` otherwise, `None` when not in check.
- **`san::check_mate_suffix_after_make(board, mv)`** (private) — today's copy + make + `in_check` + count + unmake, so `move_to_san` keeps its exact behaviour and both entry points share one expression for the rule and cannot drift.
- **`move_to_san` = body + `check_mate_suffix_after_make`** — same bytes, same decisions, same signature. No existing caller or test changes.

Nothing about movegen, legality, castling, Zobrist or the notation rules changes. No new dependency, no allocation, MIT throughout.

## Capabilities

### New Capabilities
- None. The split extends existing capabilities rather than opening a new one.

### Modified Capabilities
- `turbochess-rs-core-engine`: add the requirement that SAN rendering SHALL be consumable as body + post-move suffix, so a caller holding the after-position never pays a second make/unmake.
- `turbochess-rs-perf-fen-san`: the `SAN Write` requirement's suffix clause is now a *callable seam* (`check_mate_suffix`) rather than a private block inside `move_to_san`.

## Impact

- **Code:** `src/san.rs` (three functions where there was one; the body and the suffix expression move, the rules do not).
- **API:** additive. `move_to_san`, `san_to_move`, `play_san` keep their signatures and output; two new public functions.
- **Deps:** none new (MIT only).
- **Perf:** `move_to_san` itself is unchanged in cost; `move_to_san_body` + `check_mate_suffix` on an already-made board removes one 144B copy, one `make_move_unchecked` and one `unmake_move` per ply for such callers. Measured by the consumer (`cbh-parser`), not here.
- **Validity:** byte-identical output is enforced by a 100,000-position property test (`tests/san_split_property.rs`) and by the existing `tests/san_parity.rs` oracle against `shakmaty::SanPlus`.
