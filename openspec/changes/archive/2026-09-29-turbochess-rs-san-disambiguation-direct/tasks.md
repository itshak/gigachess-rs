## 1. The direct candidate query

- [x] 1.1 In `src/san.rs`, replace the `generate_moves_into` + `MoveList` filter inside `move_to_san_body`'s `attackers_bb != 0` branch with a loop over `attackers_bb` that, per candidate, copies the board to the stack, `make_move_unchecked`s `Move::new(cand, to, mv.promotion())`, tests `tmp.attackers_to(tmp.king_square(us).0, us.other(), tmp.occupied()) == 0` with `us` read from `board.turn()` before the copy, `unmake_move`s, and collects the square — keeping the existing `ArrayVec<Square, 8>` cap and the `same_file`/`same_rank` qualifier logic untouched. Verify: `cargo test --lib san` passes and `cargo fmt --check` is clean.

## 2. Tests

- [x] 2.1 Add `tests/san_disambiguation_property.rs`: a reference implementation of the old filter (`generate_moves_into` + the same three tests) beside a call to `move_to_san_body`, run over 200,000 positions from random playouts across the standard start and Chess960 starts 284 and 518, asserting equality for **every** legal move of every position, and asserting the body never ends in `+`/`#`. Verify: `cargo test --test san_disambiguation_property` passes.
- [x] 2.2 Extend `tests/san_disambiguation.rs` with the cases the two forms are easiest to confuse: the defender-on-the-king's-own-file position (the h1 rook, `Nbd7` not `Nd7`), three candidates needing a rank, a promotion whose twin promotes to a different role, and a pinned twin that must carry no qualifier. Verify: `cargo test --test san_disambiguation` passes.
- [x] 2.3 `cargo test --all-features` green (lib units, `san_parity` against `shakmaty::SanPlus`, `san_disambiguation`, `san_split_property`, `perft`, `chess960`, `zobrist`, `replay`, `null_move`, `play_fast_property`, `fuzz-differential`) and `cargo test --features pext` green. Verify: every suite reports `ok`, zero failures.

## 3. Performance

- [x] 3.1 `cargo bench --bench micro` before and after (`--save-baseline before` / `--baseline before`, 20 samples), Criterion-paired. **`san_48` per 48 renders: startpos 1.699 → 1.736 µs (+1.9 %, an opening line where the branch almost never fires), kiwipete 1.911 → 1.858 µs (−3.2 %, a busy middlegame), 960-284 and the `san_visitor` rows inside the band.** In-crate the effect is position-dependent rather than a clear win; the consumer is where the branch rate is 4.0 % over 883 M plies of real games, and that is where the change is measured. Verify: no row outside the ±3 % band.

## 4. Regression gates

- [x] 4.1 `cargo clippy --all-targets --all-features` and `cargo check --all-features` clean. Verify: no warnings.

## 5. Specs, version, release

- [ ] 5.1 `openspec validate turbochess-rs-san-disambiguation-direct --strict`, then archive, so the two spec deltas land in `turbochess-rs-perf-fen-san` and `turbochess-rs-core-engine`. Verify: `openspec validate <both capabilities> --strict` passes afterwards and the change is under `openspec/changes/archive/`.
- [ ] 5.2 CHANGELOG entry under `## [0.1.6]` and `Cargo.toml` `version = "0.1.6"`. Verify: `cargo read-manifest --format-version 1 | jq -r .version` reports `0.1.6`.
- [ ] 5.3 Commit with the change-ID prefix, tag `v0.1.6` and push branch + tag so `release.yml` publishes. Verify: `git tag --points-at HEAD` names `v0.1.6` and the push reports `main -> main`.
