## Why

ChessBase (.cbh) databases store pass turns as null moves (`0xffff` in `moves2`, `Z0`/`--` in PGN). `Board::make_null_move` exists as a primitive, but no `Move`/SAN/codec path carries it, so any CBH game containing a null fails to replay, render, or parse — and downstream consumers (`blind-base` backend/frontend, TS `turbochess`, `gigaboard`) either error out or silently corrupt the tree.

## What Changes

- `Move::NULL = 0xffff` becomes the first-class null-move word (the `moves2` currency): `is_null()`, `Display`/UCI `0000`, word-level sentinel (single `u16` compare, zero alloc).
- Null dispatch in **all** make/play entry points (`play`, `play_fast`, `play_hashed`, `play_checkered`, `make_move_unchecked`, `make_move_hashed`, `make_move_checkered`, `make_move_fast`), plus `is_legal`/`is_pseudo_legal`. The pass/refuse test is always a fresh `attackers_to` computation, never the cached `in_check()` (verified: after `play_fast` leaves `checkers` stale, the cached test wrongly allows a pass while in check). Raw unchecked entries apply the null transition with `debug_assert`s; validating entries refuse in check. `Move::NULL` pairs with `unmake_null_move`; `unmake_move` on a null word is undefined and asserts. `legal_moves()` still excludes null.
- SAN: `move_to_san[_body]` renders null as `--` with never a `+`/`#` suffix; `san_to_move` accepts exactly `--` and `Z0` (ChessBase spellings, nothing invented) and returns `Move::NULL`; `parseUci("0000")` ↔ `Move::NULL`, `makeUci(NULL)` → `0000`.
- Database codecs (`parse_movetext_to_moves2`, `moves2_to_san_movetext`, `replay_*`, `position_stats`) carry null end-to-end with cbvault's numbering rule (a pass is a full move: `fullmove += 1` whoever passed; `--` takes no check/mate suffix).
- Cross-repo contract (single proposal, per-repo follow-through): TS `turbochess` mirrors the word-level sentinel (`packedMove.ts` `isNull`), narrows `NULL_MOVE_SANS` to `["--", "Z0"]`, and fixes `chesstree.ts` to advance position/ply/FEN on null and emit `--`; `gigaboard` tolerates `0000`/`--` as a pass transition (never originates it); `blind-base` backend/frontend delegate to the fixed codecs (a dedup/perf-consistency change — its `board.play` path accepts null for free once dispatch lands); `cbvault` re-exports `gigachess::Move::NULL` (keeping `NULL_MOVE` as an alias) and drops its manual branches. Each repo records its own spec delta when its change lands; on archive of this change the precise null-move handling is synced into the main specs so the behavior is obvious.

## Capabilities

### New Capabilities
- None — this change promotes the existing null-move primitive to first-class `Move`/SAN/codec status within existing capabilities.

### Modified Capabilities
- `turbochess-rs-core-engine`: `Move::NULL` word, null dispatch in all make/play entries + `is_legal`/`is_pseudo_legal` with a cache-free pass test, `unmake_null_move` pairing.
- `turbochess-rs-perf-fen-san`: SAN/UCI null rendering and parsing (`--`/`Z0` in, `--`/`0000` out, no suffix).
- `turbochess-rs-database-codecs`: null carried through movetext↔moves2, hash replay, and position stats.

## Impact

- `src/moves.rs`, `src/board.rs`, `src/san.rs`, `src/database.rs`, `src/replay.rs` (+ tests); zero-alloc and MIT constraints unchanged.
- Downstream (tracked here, implemented per-repo): `turbochess` TS (`san.ts`, `chess.ts`/`board.ts`, `packedMove.ts`, `chesstree.ts`), `gigaboard` (`engine-adapter`, `animation-planner`, `board-model`), `blind-base` (`gigabase_moves.rs`, `src/lib/chess.ts`), `cbvault` (`decode.rs`, `pgn/mod.rs` de-dupe). No breaking change to legal-move streams (null was never in them); `0xffff` previously errored on validating entries (`play`, `is_legal`, `is_pseudo_legal`, SAN, codecs) but silently corrupted through the raw makes in release (debug aborts on an incidental empty-square assert) — dispatch closes that hole, turning both failures into defined behavior.
