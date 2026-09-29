## 1. Core Move word + Board dispatch

- [ ] 1.1 Add `Move::NULL` (`0xffff`) + `is_null()` in `src/moves.rs`, render UCI `0000` via `Display`, and verify `cargo test moves` passes including a null-word unambiguity test
- [ ] 1.2 Dispatch `Move::NULL` in all eight make/play entries (`play`, `play_fast`, `play_hashed`, `play_checkered`, `make_move_unchecked`, `make_move_hashed`, `make_move_checkered`, `make_move_fast`) with the pass/refuse test always fresh (`attackers_to`, never cached `in_check()`), `debug_assert(!is_null)` in the shared generic body, `is_pseudo_legal(NULL)=true`, `is_legal(NULL)` fresh, `unmake_null_move` pairing with `unmake_move(NULL)` debug-asserting, and verify with tests: null through every entry leaves castling rights untouched, a pass after a `play_fast`-given check is refused by all entries (stale-cache scenario), and `cargo test board null_move make_variants` passes. **`is_legal(Move::NULL)` must be special-cased before the generic body, not inherit it**: the generic body asks whether the *opponent's* king is safe after the move, which is the wrong question for a pass (there is no mover's king to expose) and returns the wrong answer — verified on a stale-cache check position, where the generic body reports `true` and the correct fresh test reports `false`. Pin it with a scenario asserting `is_legal(Move::NULL) == false` on a position where the side to move is in check and the cache is stale, and `true` on the same position with the cache fresh

## 2. SAN / UCI null spelling

- [ ] 2.1 Render `Move::NULL` as `--` with no suffix in `move_to_san`/`move_to_san_body` (early return, no make/unmake, touches neither the hash nor the `checkers` cache) and verify `--` never ends in `+`/`#` via unit test, including on a board with stale caches
- [ ] 2.2 Accept exactly `--` and `Z0` in `san_to_move` (fresh check test, `None` in check, reject all other invented spellings), wire `parseUci("0000")` ↔ `Move::NULL` and `makeUci` → `0000`, and verify SAN round-trip tests pass

## 3. Database codecs end to end

- [ ] 3.1 Carry null through `parse_movetext_to_moves2` (`--`/`Z0` → `0xffff`, error in check) and `moves2_to_san_movetext` (`0xffff` → `--`, fullmove advances, round-trip byte-identical), and verify with a null-bearing game test
- [ ] 3.2 Carry null through `replay_*` (incremental hash parity across the pass) and `position_stats` (sequential-reference parity), and verify `cargo test database replay` passes

## 4. Regression + parity gates

- [ ] 4.1 Run `cargo test --all-features`, `cargo clippy --all-targets --all-features`, and the SAN parity suites (`san_parity`, `san_split_property`, `san_disambiguation_property`), and verify all green with no perf-alloc regression
- [ ] 4.2 Add a CBH-realistic fixture test (compact code 0 / `by_squares` zero-word → `0xffff` → `--` → reparse) mirroring `cbvault`'s null game, replay it for hash parity, and verify it passes; additionally replay null-bearing games from the reference database corpus (not only fixtures) and verify hash parity there too

## 5. Cross-repo contract + spec sync (no code here)

- [ ] 5.1 Record the per-repo follow-up checklist, and verify each names its target spec file: TS `turbochess` (`packedMove.ts` `isNull`, narrow `NULL_MOVE_SANS` to `["--","Z0"]`, `chesstree.ts` null branch computes the null-move FEN and advances pos/ply/path, exporter emits `--`); `gigaboard` pass-transition tolerance (never originates); `blind-base` backend delegation + frontend `playMove` null (scoping the backend part honestly as dedup/perf-consistency, not a bug fix — `board.play` accepts null for free after dispatch); `cbvault` re-export of `Move::NULL` keeping `NULL_MOVE` as an alias
- [ ] 5.2 After implementation, run `openspec validate --change null-move-first-class`, sync the three deltas into `openspec/specs` via archive (reconciling explicitly with the `gigachess-rs-make-state-contract` deltas: SAN null is cache-neutral, all null variants share one fresh pass test), and verify downstream spec files document the identical contract (`--`/`Z0` in, `--`/`0000` out, `fullmove + 1`, never suffixed, `0xffff` reserved)
