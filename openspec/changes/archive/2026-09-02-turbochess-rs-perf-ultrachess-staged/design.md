## Context

`turbochess-rs` is the `PEXT`-capable `u64` engine for `blind-base` batch replay (`replay_moves2_batch` via `rayon` `~1.48M games/s`, `~224 Mnps` perft d5) + `Chess960` `Copy Board` `zobrist file keys`. `ultrachess` is the fastest `MIT` Rust engine (`rust/core 6252 LOC`, `836 Mnps` startpos d6 `geomean 3.7× shakmaty 1.23× cozy`, `BENCH.md` M4 Max `FEN write 88ns 2.5×/5.1×`, `SAN 1.43µs/48` `31%`, `hash 0.34ns`, `isCheck 0.32ns`, `clone 3.3ns` vs `cozy 1.7ns`). We may copy ultrachess code directly (`MIT`→`MIT` with attribution) where it survives `u64`/`Chess960` translation; goal is **≥ ultrachess in most — preferably all — metrics**.

## Goals / Non-Goals

**Goals:**
- Real micro harness before any engine edit, frozen baseline, `>3%` median gate.
- **Parity target:** `≥ ultrachess` median on 6 perft positions + 8 micro rows (`FEN write/parse`, `movegen one-shot`, `make+unmake 48-ply`, `isCheck in/out`, `hash`, `SAN 48`, `clone`) on `M1/M4 Max` `LTO=fat, codegen-units=1, panic=abort`. Most→all rows must win; any loss is documented with follow-up issue (like `ultrachess` `4 losses` table).
- Keep `MIT`, `no SIMD` (not portable to WASM), `no WASM`, `criterion` `Throughput::Elements`, `Board:Copy` plain data + `Chess960` compat.

**Non-Goals:**
- `cozy` `19ns` `movegen one-shot` / `1.7ns` `clone` arena extraction (`Board ~100B` vs ours `~368B` with `Chess960` `castle_rook_sq[4]` + `mailbox` — large refactor low ROI, `BENCH.md: Deliberate clone`).
- `PEXT` new feature (already `pext`).
- Colour-templated movegen / published magic numbers / incremental checker update (ultrachess `BENCH.md: Optimisations we've declined` — metaprogramming / bloat / error-prone).
- `WASM` bulk.

## Decisions

### D1: Harness First, Baseline Frozen

- **Decision:** Land `benches/micro.rs` (8 rows) + `BENCH.md` update in one PR, run `just bench` (`cargo test` gate + `criterion` `3 passes` median) on `M-series`, write `benches/results/turbochess-rs-baseline.json`. Every later `src/` PR diffs vs baseline with `±3%` band (like `ultrachess` `just bench` refuses to publish from broken tree).
- **Why:** Today `vs_libraries` compares `shakmaty`/`cozy` but no single-call `ns/op` table to tell whether `MoveSink` helps.

### D2: MoveSink Bulk + Split Pins / Bulk Pawns

- **Decision:** Generic `MoveSink` (`push_targets(from,Bitboard)`, `push_pawn_targets_offset`, `push_pawn_promotions_offset`, `push_one`) with `MoveList { buf:[MaybeUninit<Move>;256], len }` (`512B` `MaybeUninit` skip `memset`) and `MoveCounter { count:u32 }` `count+=popcount` without `pop_lsb`. Add `compute_pinned_split→(pinned_hv,pinned_diag)` to avoid per-slider `tables::line()` (`ultrachess/movegen.rs:214`) and bulk pawn shifts `north(pawns)&!occ&check_mask` split `promo/non-promo`. `perft depth==1` uses counter path — `BENCH.md: caveat 6` *this is what produces the perft lead*.
- **Trade-off:** Adds generic over `movegen.rs:852` but same code path — no duplication; monomorph `×2` (mitigated `lto=fat`).

### D3: Cached Checkers + Perft Slim

- **Decision:** `Undo { prev_checkers:Bitboard, prev_zobrist:u64, prev_castling, prev_ep, prev_halfmove, captured }` and `Board { checkers:Bitboard, zobrist:u64, history_hashes:Vec<u64> }` maintained in `make`/`unmake` (`ultrachess/position.rs:42` shape). `in_check()=checkers!=0` `0.32ns`, `hash()` load `0.34ns`, `unmake` restores without `attackers` scan. Add `make_move_perft/unmake_move_perft` slim (skip `zobrist` XORs, `history_hashes` push, `halfmove/fullmove`; `position.rs:389` *Safe only for perft*). Perft uses slim, UI/search uses caching path.
- **Cost:** `+16B/Undo` (50-move bound → `<800B` history), `+2ns/make` `attackers_to` to refresh `checkers`. **ROI:** `8× isCheck`/`isCheckmate`/`has_no_legal_moves` dominate search/UI; perft doesn't pay (slim). Kept per `BENCH.md: Deliberate make+unmake tradeoff`.

### D4: FEN/SAN Branchless

- **Decision:** `fen.rs` `write_fen` `const PIECE_CHAR:[u8;12]` `ArrayVec<u8,128>` `write!` without `format!` frame (`fen.rs:189` `88ns`); `san.rs` `move_to_san(&mut Position)` `String::with_capacity(8)` reusing `tables::between`, `append_check_suffix` gates `has_no_legal_moves` behind O(1) `in_check()`, `make/unmake` not `clone`, disambig pre-filter `attackers_bb = same_type &!from & attacks_from_target(to)` skips movegen. `1.43µs/48` target.

### D5: Parity Target & Gating

- **Decision:** Micro harness is **gate**; every patch diffs vs frozen `turbochess-rs-baseline.json` median `±3%` band (like `ultrachess` `just bench` refuses to publish from broken tree). Goal `≥ ultrachess` on 6 perft + 8 micro rows; if any axis loses (e.g. `movegen one-shot 25ns vs cozy 19ns`, `clone 3.3ns vs 1.7ns`), document gap + follow-up issue and technique in `benches/vs_libraries` + `codec_bench` docs — like `ultrachess BENCH.md: Deliberate 4 losses` table. `make+unmake` loss vs `cozy` is *kept* because `isCheck` dominates (see D3).
- **Why:** Today `vs_libraries` compares `shakmaty`/`cozy` but no single-call `ns/op` to tell whether `MoveSink` helps; parity target makes the ambition explicit.

## Risks / Trade-offs

- **[Risk]** Generic `MoveSink` may monomorphise and bloat `codegen-units`.
  - **Mitigation:** `lto=fat`, `codegen-units=1` already in `Cargo.toml`; `bench` size gate `<5%` `.text` bloat; same path no duplication.
- **[Risk]** Baseline frozen on one host (`M1/M4 Max`).
  - **Mitigation:** `±3%` median gate, not absolute `Mnps`; report `min of N` per `BENCH.md: Methodology caveat 2`.
- **[Risk]** `make_move_perft` private slim path could be misused outside perft.
  - **Mitigation:** `#[inline]` + `debug_assert` + `_perft` suffix, `unmake_move_perft` zeroes `prev_halfmove/zobrist` so mismatched `unmake` panics in invariants.
- **[Risk]** `checkers` cache adds per-`make` `attackers_to` cost.
  - **Mitigation:** Per D3, perft bypasses; `BENCH.md` shows net win on `isCheck`/`SAN`/`has_no_legal_moves`.

## Open Questions

- Keep `BENCH_ITERS` as `criterion` default or fix `10k` like `ultrachess` `criterion` `micro.rs`?
- Accept `movegen one-shot -6ns` / `clone -1.6ns` gaps as wont-fix (ultrachess does) or chase `colour-templated` (rejected per Non-Goals)?
