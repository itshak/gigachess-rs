## 1. The SAN split

- [x] 1.1 Split `san::move_to_san` into `pub fn move_to_san_body(board, mv) -> Option<San>` (today's lines 28-135 verbatim) and `pub fn check_mate_suffix(after: &Board) -> Option<char>` (`in_check()` → `'+'`, `count_legal_moves() == 0` → `'#'`), with a private `check_mate_suffix_after_make` holding today's copy/make/read/unmake, and `move_to_san` as their composition. Verify: `cargo test --lib san` passes with the pre-existing SAN tests untouched.
- [x] 1.2 Add `tests/san_split_property.rs`: over 100,000 positions from random playouts across the standard start and Chess960 starts 284 and 518, assert `move_to_san == move_to_san_body + check_mate_suffix(after)`, that the body never ends in `+`/`#`, and that make/unmake is clean. Verify: `cargo test --test san_split_property` passes.
- [x] 1.3 Add the `src/san.rs` unit tests: quiet / `+` / `#` / promotion-with-check / disambiguation shapes, the body-carries-no-suffix property, the composition equals the monolith, and `a_stale_checkers_cache_would_be_a_stale_suffix` pinning the one sharp edge (the fast makes leave `checkers` stale, so `check_mate_suffix` must not be fed a fast-made board). Verify: `cargo test --lib` 49 pass.

## 2. Regression gates

- [x] 2.1 `cargo test --all-features` green: 49 lib unit tests, `san_parity` against `shakmaty::SanPlus`, `san_disambiguation`, `perft` (6 positions), `chess960`, `zobrist`, `replay`, `null_move`, `play_fast_property`, `fuzz-differential`. Verify: every suite reports `ok`, zero failures.
- [x] 2.2 `cargo clippy --all-targets --all-features` and `cargo check --all-features` clean. Verify: no warnings.

## 3. Specs, version, release

- [x] 3.1 Add the requirement *SAN Rendering SHALL Be Splittable Into Body and Post-Move Suffix* to `turbochess-rs-core-engine` and the seam clause to `turbochess-rs-perf-fen-san`, with Given/When/Then scenarios over the property test and the unit tests. Verify: `openspec validate turbochess-rs-san-split --strict` passes.
- [x] 3.2 CHANGELOG entry under `## [0.1.5] - 2026-09-28` and `Cargo.toml` `version = "0.1.5"`. Verify: `cargo read-manifest --format-version 1 | jq -r .version` reports `0.1.5`.
- [x] 3.3 Commit with the change ID prefix, tag `v0.1.5` and push branch + tag so `release.yml` publishes. Verify: `git tag --points-at HEAD` names `v0.1.5` and the push reports `main -> main`.
