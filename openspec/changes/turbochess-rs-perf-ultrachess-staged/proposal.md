# Proposal: Benchmark-First Incremental Perf from ultrachess (RS) — At Least Parity

## Why

`turbochess-rs` (`0.1.0`, `PEXT`/`Magic`, `moves2` `u16`, `replay_moves2_batch` `~1.48M games/s` on `M1 Max`, `~224 Mnps` perft d5 bulk) is correct but `ultrachess` (`MIT`, `rust/core 6252 LOC`) is the fastest `MIT` Rust engine (`836 Mnps` startpos d6, `geomean 3.7× shakmaty 0.30`, `1.23× cozy-chess 0.3.4`, `BENCH.md` M4 Max). Ultrachess also leads the micro table (`FEN write 88ns` `2.5× shak/5.1× cozy`, `SAN 1.43µs/48` `31%`, `hash 0.34ns`, `isCheck 0.32ns` both states) and documents the 4 losses as deliberate (`FEN parse 144ns vs shak 125ns`, `movegen one-shot 25ns vs cozy 19ns`, `make+unmake 48 503ns vs 353ns`, `clone 3.3ns vs 1.7ns`).

Today `turbochess-rs` has `cargo bench` (`perft_bench`, `replay_bench`, `vs_libraries`) but **no gate that refuses to publish from a broken tree**, no `BENCH.md` scoreboard like `ultrachess/BENCH.md:1`, and no single-call micro harness (`ns/op`) that tells whether `MoveSink` bulk or `Undo.prev_checkers` helps. Goal of this change is **at least parity with ultrachess in most — preferably all — micro + perft metrics** on `Apple M1 Max/M4 Max` (`LTO=fat, codegen-units=1, panic=abort`), and we may **copy ultrachess code directly** (both `MIT`): `MoveSink` bulk, split pinned masks, bulk pawn shifts, `Undo.prev_checkers` cache, `make_move_perft` slim path, branchless `write_fen`/`move_to_san` — all survive `u64` translation. Every patch is gated `>3%` median vs frozen baseline and revertible.

This change does **benchmark-first, one-patch-at-a-time** for Rust: extend harness to match `ultrachess/BENCH.md` rows, freeze baseline, land structural wins incrementally with hard parity target.

## What Changes

- **Benchmark extension (must be first, no engine changes):**
  - Add `benches/micro.rs` `criterion` `single-call` group matching `ultrachess/BENCH.md: micro.rs` rows: `FEN write`, `FEN parse`, `movegen one-shot`, `make+unmake 48-ply`, `isCheck in/out`, `hash`, `SAN 48`, `clone` on `startpos`/`kiwipete`/`960-284` with `Throughput::Elements`.
  - Make `just bench` gate on `cargo test` + in-binary perft sanity (like `ultrachess` `just bench` `gate + perft NPS`) and write `benches/results/turbochess-rs-baseline.json` + `BENCH.md` table. Keep existing `perft_bench`, `replay_bench`, `vs_libraries` and add `bench-wasm` stub if needed.
  - Freeze baseline tag before any `src/` edit; every later PR diffs `±3%` median (not absolute Mnps).

- **Incremental engine patches (each gated, revert if no gain; copy MIT with attribution, same tradeoffs as ultrachess/D1-D4):**

  1. **`MoveSink` bulk count + split pins/bulk pawns:** Generic `MoveSink` (`push_targets(from,Bitboard)`, `push_pawn_targets_offset`, `push_pawn_promotions_offset`, `push_one`) with `MoveList {buf:[MaybeUninit<Move>;256], len}` `512B` `MaybeUninit` skip `memset` and `MoveCounter {count}` `count+=popcount` without `pop_lsb`. Reuse `tables::between/line`, `compute_pinned_split → (pinned_hv,pinned_diag)` to avoid per-piece `line()` dependent load, bulk pawn shifts `north(pawns)&!occ&check_mask` split promo/non-promo. Same split as `ultrachess/rust/core/src/movegen.rs`. `perft depth==1` uses counter path — the `geomean 1.23× vs cozy` win; `BENCH.md: Methodology caveat 6` states *this is what produces the perft lead*. Also add `count_legal_moves(&self)->u32`.
  2. **Cached checkers + perft slim (`Undo.prev_checkers`):** `Undo {prev_checkers:Bitboard, prev_zobrist:u64, prev_castling, prev_ep, prev_halfmove, captured}` and `Board {checkers:Bitboard, zobrist:u64, history_hashes:Vec<u64>}` maintained in `make`/`unmake` like `ultrachess/position.rs:Undo` (`checkers !=0` → `0.32ns` branch-free `in_check()`, `hash()` load `0.34ns`). Add `make_move_perft/unmake_move_perft` slim (skip `zobrist` XORs, `history_hashes` push, `halfmove/fullmove` — `position.rs:389` `Safe only for perft`); perft uses slim, UI/search uses caching path. Cost `+16B/Undo` (50-move bound) for `8× isCheck`.
  3. **FEN/SAN branchless:** `write_fen` via `const PIECE_CHAR:[u8;12]` `ArrayVec<u8,128>` without `format!` (`ultrachess src/fen.rs:189` `88ns`), `move_to_san(&mut Position)` `String::with_capacity(8)` reusing `tables::between`, gating expensive `has_no_legal_moves` behind `in_check()` O(1), check-suffix via `make/unmake` not `clone`, disambig pre-filter `attackers_bb = same_type &!from & attacks_from_target(to)` skips movegen. Copy `ultrachess src/san.rs:1` `1.43µs/48`.
  4. **Harness parity:** add `tests/fuzz-differential.rs` `1k` random games vs `shakmaty` lockstep (`FEN`+`legal`+`check` per ply) with `95%` cov gate (`cargo llvm-cov`), like `ultrachess TESTING.md: just coverage`.

- **Explicit non-goals (ultrachess Deliberate trade-offs `BENCH.md:Deliberate`):** No `SIMD`, no `colour-templated movegen` (metaprogramming), no `published magic numbers` (128 `u64` WASM init), no `incremental checker update` (error-prone `castling/promotion/EP`), no `WASM` bulk; no `cozy 1.7ns clone` arena extraction (`~100B Board` vs our `~368B Copy + Chess960 rook squares` — large refactor low ROI).

## Capabilities

### New Capabilities
- `turbochess-rs-bench-micro`: Single-call `criterion` micro group with baseline freeze and `>3%` gate.
- `turbochess-rs-perf-bulk-count`: `MoveSink`/`MoveCounter` bulk perft (incl. split pinned/bulk pawns, `count_legal_moves`).
- `turbochess-rs-perf-cached-checkers`: `Undo.prev_checkers` branch-free `isCheck` + `make_move_perft` slim.
- `turbochess-rs-perf-fen-san`: Branchless `FEN`/`SAN` (`piece char` table, `tables::between`, make/unmake suffix).

### Modified Capabilities
- `turbochess-rs-core-engine`: Perft `depth==1` wired to `MoveCounter`, micro table in gate, parity target vs ultrachess head-to-head on 6 perft positions + 8 micro rows.

## Impact

- **Goal:** **≥ ultrachess in most — preferably all — perft (6 positions, d6/d5) + micro (8 rows × 3 FENs) medians** on `M1/M4 Max` (`vs_libraries` `geomean 1.23×` + micro `88ns/1.43µs/0.32ns`). If any axis loses, gap is documented with follow-up issue and technique (like `ultrachess` `4 losses` table); `make+unmake` cache is *kept* because `8× isCheck` dominates real workloads (`BENCH.md: tradeoff`).
- **Public API:** Additive `count_legal_moves(&self):u32` via bulk sink aside `legal_moves()->ArrayVec<Move,256>` (unchanged). `Board:Copy` retained (plain data, `Chess960` `rook_file` keys `MIT`); `perft` internally uses slim path.
- **Perf:** Expected `1.2-1.4×` perft at bulk alone, `~0.3ns` `isCheck`/`hash`, `FEN write` halving to `88ns`; only patches `>3%` median stay. Parity with ultrachess lifts us `3.7×` vs `shakmaty` baseline.
- **Risk:** Low — gated, revertible, `MIT` stays `MIT` (copy attribution). Perft slim is `unsafe` to call outside perft — gated by `#[inline]` private API + `debug_assert`.
