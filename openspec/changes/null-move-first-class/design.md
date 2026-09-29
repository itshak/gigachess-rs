## Context

See `proposal.md` for motivation. Current state: `Board::make_null_move*`/`unmake_null_move` exist (`src/board.rs`), but `Move` (`src/moves.rs`), SAN (`src/san.rs`), and codecs (`src/database.rs`, `src/replay.rs`) have no null path — `0xffff` errors everywhere. `cbvault` carries its own `NULL_MOVE = 0xffff` const plus manual `--`/no-suffix/fullmove branches (`decode.rs`, `pgn/mod.rs`). TS `turbochess` knows null only in `chesstree.ts` (and mishandles it: no position/ply advance, drops `--` on export); `gigaboard` and `blind-base` have no null path. Zero-alloc (`ArrayVec`, no per-move heap) and 100% MIT constraints stand.

## Goals / Non-Goals

**Goals:**

- One word-level null definition (`0xffff`) shared by engine, codecs, and (by re-export) `cbvault`.
- Generic replay loops (`play`/`replay`/walkers) work without per-caller null branches.
- SAN/UCI spellings fixed to ChessBase reality: `--`/`Z0` in, `--`/`0000` out, never a suffix.
- A cross-repo contract precise enough that each repo's follow-up change is mechanical.

**Non-Goals:**

- No new move-generation semantics (null stays out of `legal_moves`, perft unchanged).
- No PGN writer rewrite here beyond the `moves2_to_san_movetext` numbering rule; no `cbvault`/`blind-base`/TS/`gigaboard` code edits in this change (those are separate per-repo changes implementing this contract).
- No additional accepted spellings (`null`, `pass`, …) — explicitly rejected per user decision.

## Decisions

1. **Word-level sentinel, not a struct flag.** `Move::NULL = Move(0xffff)`, `is_null() = self.0 == 0xffff`. Alternative (boolean/enum variant) costs an extra field or branch in the hot loop and breaks the `moves2` currency (`u16` in, `u16` out). A single `u16` compare is the cheapest possible check in both Rust and TS (`word === 0xffff`), keeps `Move: Copy`, and matches the CBH wire format. No legal word collides (same square twice + promo bits > 4).
2. **Null dispatch in every entry point, decided by a fresh test, never the cache.** All eight make/play entries (`play`, `play_fast`, `play_hashed`, `play_checkered`, `make_move_unchecked`, `make_move_hashed`, `make_move_checkered`, `make_move_fast`) check `is_null()` first and take the null path; `is_pseudo_legal(NULL)` is true (the sentinel is structurally well-formed — the check test belongs to legality, not geometry) and `is_legal(NULL)` is a fresh `attackers_to(king_square(turn), turn.other(), occupied()) == 0`, never `in_check()`. Alternatives considered and rejected: (a) dispatching only in `play`/`play_fast` — leaves the raw makes silently corrupting (verified: `make_move_unchecked(0xffff)` falls into generic from=63/to=63 logic, aborting on an incidental assert in debug and destroying castling rights in release); (b) deciding legality via cached `in_check()` — re-opens the bug `gigachess-rs-make-state-contract` fixed (verified: after `play_fast` leaves `checkers` stale, the cached test allows a pass while in check and diverges from `make_null_move_fast`). `is_legal(Move::NULL)` is answered by its own branch rather than the generic make-and-test body: the generic body asks whether the *opponent's* king survives the move, which is the wrong question for a pass and returns the wrong answer on a stale-cache check position (verified — generic body `true`, correct fresh test `false`). Validating entries refuse a null in check; raw unchecked entries apply the null transition (unchecked, like any unchecked move) with `debug_assert`s, and the shared generic body asserts `!is_null()` so any future bypass fails in debug. `Move::NULL` pairs exclusively with `unmake_null_move`; `unmake_move` (and fast/perft unmakes) on a null word is undefined and asserts — the current accidental round-trip through phantom h1h1-style squares is a layout coincidence, not a contract. This also conforms the non-fast `make_null_move*` variants to 0.1.7's own "all four refuse in exactly the same positions" rule at the cost of one `attackers_to` scan on a path taken roughly once per ten thousand plies.
3. **SAN `--`, unconditionally suffix-free and cache-neutral.** `move_to_san[_body](NULL)` returns `--` before touching pieces or making any move; `check_mate_suffix` is never consulted for null (a pass gives no check — matches Mega export and `cbvault`'s `suffix_pending = false`). Rendering a null therefore needs *and* disturbs neither the hash nor the `checkers` cache, which reconciles this change with 0.1.7's state contract: no `wants_checkers`-style obligation attaches to a SAN null. Alternative (suffix after pass) contradicts ChessBase output and would corrupt gold comparisons.
4. **Fullmove always advances (the "pass is a move" rule).** Reuse the existing `make_null_move` semantics (`fullmove += 1` whoever passed) rather than inventing half-move accounting. `moves2_to_san_movetext` numbers `--` with the same rule `cbvault`'s writer uses (White pass leaves the following Black at the same printed number only because the counter already advanced — no special-casing in the board, only in number *rendering* if needed).
5. **Codecs treat null as an ordinary ply.** `parse_movetext_to_moves2` resolves `--`/`Z0` against the current position (error in check); `moves2_to_san_movetext`, `replay_*`, `position_stats` match on word `== 0xffff` first, then the normal path. Keeps each loop O(1) per ply with no movegen scan.
6. **Cross-repo contract (this proposal is the single source; each repo lands its own spec delta on implementation):**
   - *TS `turbochess`*: same `0xffff` word with `isNull` in `packedMove.ts` (which today masks the sentinel through as a normal word); `NULL_MOVE_SANS` narrowed to `["--", "Z0"]` (it currently also accepts `"null"`); `makeMove` flips turn, clears ep, bumps clocks (mirror Rust); `parseSan` accepts `--`/`Z0`; `chesstree.ts` null branch computes the null-move result FEN and advances `pos`/`ply`/`path` (today `node.fen` stays at the parent FEN and every later node is one ply out of step), descends the mainline, and the exporter emits `--` instead of dropping the ply.
   - *`gigaboard`*: tolerate `0000`/`--` as a pass transition (no piece animation, update turn, keep position stack consistent); never originate null from gestures; no legal-target ring for it.
   - *`blind-base`*: backend delegates PGN/moves2/hash paths to the fixed `gigachess::database` codecs (a dedup/perf-consistency change, not a bug fix — its `board.play` replay accepts null for free once dispatch lands); `next_move_at_ply` returns `("--", "0000")` on null; frontend `playMove` accepts `--`/`Z0`/`0000`.
   - *`cbvault`*: re-export `gigachess::Move::NULL`, keeping `NULL_MOVE` as an alias so downstream code does not break, and delete the manual SAN/numbering branches in favor of `move_to_san_body`/`check_mate_suffix` + the shared numbering rule (after a release carrying `Move::NULL`).

## Risks / Trade-offs

- [Risk] `0xffff` errored on validating entries but silently corrupted through raw makes in release → Mitigation: dispatch in all eight entries plus `debug_assert(!is_null)` in the shared body and on `unmake_move`; tests pin both the defined behavior and the refusal paths.
- [Risk] Non-fast null variants now pay a fresh `attackers_to` scan instead of reading the cache → Mitigation: negligible (nulls occur roughly once per ten thousand plies) and required for 0.1.7's own "all four refuse the same positions" rule to hold after cache-skipping makes.
- [Risk] TS `chesstree` trees built while null didn't advance are silently corrupt → Mitigation: TS fix replays from scratch; no migration of cached trees — they are re-derived.
- [Risk] Suffix-free `--` could mask a real check if a caller appends suffixes blindly → Mitigation: null path returns before suffix logic; unit test asserts `--` never ends in `+`/`#`.
- [Risk] Per-repo spec sync forgotten at archive → Mitigation: tasks.md ends with explicit sync/archive checklist naming every target spec file.

## Migration Plan

1. Land Rust engine + codecs + tests (this change); `cargo test`, `clippy`, SAN parity suites green.
2. Open one change per downstream repo implementing the contract section above, each with its own spec delta.
3. `cbvault` de-dupe last (it can consume the new API only after release).
4. Archive this change with `openspec-sync-specs` so the three modified specs absorb the deltas; verify downstream spec files reference the same spellings (`--`/`Z0` in, `--`/`0000` out, `fullmove + 1`, no suffix).

## Open Questions

None — spellings (`--`/`Z0`), all-entries dispatch with a fresh cache-free pass test and `unmake_null_move` pairing, fullmove semantics, word-level representation for performance, and single-proposal/multi-spec recording were all decided before planning (the last three tightened by review).
