## 1. Core Move word + Board dispatch

- [x] 1.1 `Move::NULL` (`0xffff`) + `is_null()` in `src/moves.rs`, `Display` renders UCI `0000`. Verified by `the_null_word_is_ffff`, `no_ordinary_move_collides_with_the_null_word` (all 65,536 square×square×promotion combinations, proving no legal word collides) and `the_null_word_renders_as_uci_0000`.
- [x] 1.2 Dispatch in all eight make/play entries, `debug_assert!(!is_null)` in the shared generic body and on the three unmakes, `is_pseudo_legal(NULL)=true`, `is_legal(NULL)` on its own fresh branch. **`is_legal` is special-cased before the generic body** — the generic body asks whether the *opponent's* king survives, which returns `true` on exactly the positions that must return `false`; verified on a stale-cache check position (generic body `true`, correct fresh test `false`) and pinned by `null_legality_is_decided_for_the_mover_not_the_opponent`, which **fails if the branch is removed** (verified by reverting it: 8 of 33 tests fail). Stale-cache refusal pinned by `a_pass_after_a_cache_free_make_is_refused_while_in_check`, which asserts its premise first (the cache reads "not in check" while a fresh computation reads "in check") and **fails if the fresh test is reverted to the cache** (verified: 2 tests fail).

## 2. SAN / UCI null spelling

- [x] 2.1 `move_to_san`/`move_to_san_body` render `--` with no suffix via an early return before any make, so neither the hash nor the `checkers` cache is touched. Pinned by `a_null_never_takes_a_check_or_mate_suffix` and `rendering_a_null_is_cache_neutral` (asserts a deliberately stale hash stays exactly as stale).
- [x] 2.2 `san_to_move` accepts exactly `--` and `Z0`, refuses in check, and rejects every other spelling — `null`, `pass`, `NIL`, `0000`, `skip`, `-`, `z0`, `Z1` are each pinned as rejected. UCI `0000` is `Move::NULL`'s `Display` (1.1).

## 3. Database codecs end to end

- [x] 3.1 `parse_movetext_to_moves2` and `moves2_to_san_movetext` carry a pass, round-tripping byte-identically (`a_null_word_round_trips_through_movetext`, `a_null_renders_as_dashes_and_is_not_dropped`, `a_pass_encoded_from_a_compact_word_round_trips`).
- [x] 3.2 `replay_*` and `position_stats` carry a pass with incremental-hash parity at every ply (`a_pass_bearing_game_keeps_hash_parity_at_every_ply`, `replaying_a_game_with_a_pass_yields_hashes`, `a_pass_produces_its_own_indexed_position`).
- **No per-caller branches were added to the codecs.** Every codec reaches the transition through `play`, so the dispatch carries them for free — which is design goal 2 ("generic replay loops work without per-caller null branches") rather than a deviation from it. The tests above are what prove it.

## 4. Regression + parity gates

- [x] 4.1 `cargo test --all-features`, `cargo clippy --all-targets --all-features` (0 warnings) and the SAN parity suites all green. No perf-alloc regression: a pass adds no allocation, and the null path is a single `u16` compare.
- [x] 4.2 A CBH-realistic fixture (`a_pass_encoded_from_a_compact_word_round_trips`) mirrors cbvault's null game: raw `0xffff` → render → reparse, with hash parity.
- [x] 4.3 **Substituted for the reference-corpus replay**, which needs the Mega Database and is not available locally: a 200,000-position property test (`tests/null_move_property.rs`) over random playouts from the standard start and two Chess960 starts, injecting passes and deliberately breaking the caches with `play_fast` before sampling. Measured coverage: 668,700 plies, 198,873 legal passes, 1,127 refused passes, **1,364 positions where the cached `checkers` disagrees with a fresh computation**, 397,746 refreshed caches verified, 232,164 hash parities checked. Verified non-vacuous — reverting the fresh pass test to the cached one makes it fail.
- [x] 4.4 Coverage assertions at the end of the property test, so it cannot pass by reaching none of the states it claims to test. One counter was **removed as unreachable rather than weakened**: "a legal pass that leaves the new side to move in check" cannot happen, because a pass changes no pieces, so the opponent's check status is unchanged, and a position where the side *not* to move is in check is unreachable by legal play. That is a property of the transition, not a missing fixture; what is verified instead is that the refreshed cache matches a fresh computation for the new side to move.

## 5. Cross-repo contract + spec sync (no code here)

- [x] 5.1 Per-repo follow-up checklist recorded, each naming its target spec file. Landed: TS `turbochess` (`packedMove.ts` `isNull`, `NULL_MOVE_SANS` narrowed to `["--","Z0"]`, `chesstree.ts` pass branch advances pos/ply/path and exports `--`), `gigaboard` (pass-transition tolerance, never originates one), `blind-base` (frontend `playMove` + `next_move_at_ply`).
- [x] 5.2 Archived with the three deltas synced into `openspec/specs`, reconciling with `gigachess-rs-make-state-contract`: the scenario it contributed on shared requirements was carried into this change's delta first, so archiving dropped nothing.

## 6. Carried forward (not done here)

- **`cbvault` re-export and de-dupe.** `cbvault` should re-export `gigachess::Move::NULL`, keep `NULL_MOVE` as an alias so nothing downstream breaks, and delete its manual `--`/no-suffix/fullmove branches in favour of `move_to_san_body` plus the shared numbering rule. This **requires gigachess 0.1.8 to be published**, which has not happened — `cargo publish` is an owner decision. Nothing was faked to work around that.
- **Reference-corpus replay** of null-bearing CBH games (task 4.2's original wording), pending access to the database.

