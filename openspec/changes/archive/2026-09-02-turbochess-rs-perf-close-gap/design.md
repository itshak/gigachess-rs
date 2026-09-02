## Context

`turbochess-rs` after `perf-ultrachess-staged` is `387 Mnps` perft (vs `836` ultrachess, `2.16×` gap) with `BENCH.md` micro `77ns FEN write` (win), `0.47ns isCheck/hash` (vs `0.32/0.34`), `3.68µs SAN 48` (vs `1.43`), `1.08µs make+unmake 48` (vs `503ns`), `65ns movegen` (vs `25`). `Board 368B Copy` + `Chess960` kept (`clone 430 vs 3.3ns` deliberate). C++ `Gigantua/Gigantua` (`2.1 Gnps` claim, `Perft aggregate 18.9B 9967ms 1906 Mnps` on Ryzen 5000 `PEXT`, `~1.4 Gnps` after tuning) and `Stockfish` (`GPL-3`) are **not MIT**; `Gigantua` repo has **no LICENSE** (`CPOL` via CodeProject article `5313417`) — cannot copy. Only `ultrachess` (`MIT`) and `cozy-chess` (`MIT`) are reusable.

## Goals / Non-Goals

**Goals:**
- Explain the 2.16× gap (BENCH.md gap report) and inventory Gigantua/Stockfish techniques with MIT compliance.
- Design MIT-safe path to `≥ ultrachess in most rows`: visitor (2×), colour-templated movegen, compact board probe, SAN visitor-free.
- Keep `Board:Copy`, `MIT` stays `MIT`, `LTO=fat codegen-units=1` already set, `±3%` gate vs frozen baseline.

**Non-Goals:**
- Copying `GPL`/`CPOL` code (Gigantua `Chess_Base.hpp`, `Gigantua.cpp`, Stockfish `movegen.cpp`, ferrum `Apache-2.0` without attribution).
- `SIMD`, published magics (128 `u64` WASM init), incremental checker update (error-prone), `WASM` bulk.
- Keeping `368B` default when `blind-base` can be rewritten — compact `128B Copy` will become default (still `Chess960` via `rook_sq[4]`), with `mailbox` removal + `castle_mask` function (D3).

## Decisions

### D1: Visitor Pattern (Gigantua's 2×)

- **Decision:** `trait MoveVisitor { visit_targets(from: u8, mask: u64); visit_pawn_offset(targets: u64, offset: i8); visit_promotion_offset(...); visit_one(mv) }` parallel to `MoveSink`, with `CountingVisitor { count: u32 }` (`popcount` without `pop_lsb` or `Move` construction). `Board::generate_visitor<S:MoveVisitor>` shares bulk pawn + `compute_pinned_split` code with `MoveSink` via macro/generic, monomorph `×2` (visitor vs sink), `LTO` mitigates bloat.
- **Why:** Gigantua's summary: *Make/Unmake and a Movelist is not needed and 2x slower than a visitor pattern — which is even more powerful*. Visitor removes `Move::new` + `ArrayVec` writes for leaf bulk (`depth==1`), the `geomean 1.23× vs cozy` win's next step. Ultrachess already uses `MoveCounter` (popcount); visitor goes one step further (no `Move` at all).
- **Alternative:** Keep `MoveCounter` only — leaves `pop_lsb` for materialised path but visitor still materialises per `push_targets` loop; `MoveList` still does `pop_lsb`. Visitor for counting avoids `Move` entirely, so perft leaf should be `>15%` faster vs `MoveCounter`.
- **Measured outcome (2026-09-02, M1 Max, gate run):** `perft_visitor(1)` leaf is **1.0× vs `MoveCounter`** (47.8 vs 47.3 ns, −1.4%; d5 full 388.7 vs 385.5 Melem/s, +0.8% noise) — the `>15%` win is **unattainable in this codebase** because our `MoveCounter` was already a pure `count += popcount` sink (no `pop_lsb`, no `Move` construction); the D1 premise described an older materialising counter. Per user decision the visitor is **kept as a zero-cost additive API** (adapter forwards into the same generator body; parity tests green on all 6 CPW positions) — it is the hook the templated/compact paths and SAN counting build on, not a standalone win. Recorded honestly in `BENCH.md` perft section.

### D2: Colour-Templated Movegen

- **Decision:** `fn generate_moves_templated<const WHITE: bool, S: MoveSink>(sink: &mut S)` (or `Colour` generic) with `if WHITE` branches const-folded, called via `if self.turn == White { generate_templated::<true> } else {::<false>}`. Extract pawn shifts (`north = if WHITE {<<8} else {>>8}`) and `PAWN_ATT` index to compile-time. Keep runtime `generate_moves_into` as dispatch wrapper (no API break).
- **Why:** Gigantua template expansion for `color`, `enpassant`, `castling` squares is its *most important definition for performance* (`namespace Lookup = Lookup_Pext`). Branch predictor friendly but `if white` per pawn still costs. Colour templates remove branches and enable `Lookup_Pext` specialization per colour (already `pext` feature). Ultrachess `BENCH.md: Optimisations we've declined` lists colour templates as metaprogramming, but we now prototype behind non-breaking wrapper; `cozy-chess` precedent: compile-time keys from documented seed.
- **Alternative:** Keep runtime `if white` — simpler, but leaves `movegen one-shot 65 vs 25ns` gap.
- **Measured outcome (2026-09-02, M1 Max, same-session A/B):** templated dispatch measures **77.7 ns vs 79.4 ns runtime path (−2.2%)** — a real but small win; the `>10%` movegen gate was **not met** (absolute numbers drift ±5% across sessions: 65 ns baseline was recorded in a cooler session; both variants measure ~78-79 ns now). Per user policy (visitor precedent): **templated path kept** — A/B-verified faster, perft parity green on all 6 CPW positions, `.text` delta 0.0% (`size` page-rounded, gate <5%). The remaining `movegen one-shot` gap is attributed to `Board` size (368B), addressed by D3 compact. `Board:Copy` retained (derive unchanged) and `pext` feature still runtime-detects `bmi2` with Fancy fallback (`attacks.rs:62`, `cargo test --features pext` green on M1 Max).

### D3: Compact Board Default 128B (blind-base rewrite allowed)

- **Decision:** **Make `128B Copy` the default** (since `blind-base` can be rewritten for max perf, per user). Remove `mailbox[64] 64B` → `scan 12 bbs` for `piece_at` (12× bit test, +3ns `FEN`/`SAN` but perft never calls `piece_at` — net win), and `castle_mask[64] 64B` → `fn castle_mask(from,to)` derived from `rook_sq[4]` (like ultrachess). Keep `rook_sq[4]` (4B) for `Chess960` `X-FEN`/`Shredder`/`rook-file` hash, keep `hash`/`checkers` for `SAN`/search (`perft` slim skips it, like Gigantua No Hashing). New layout: `bbs 96 + occ 8 + occupied 8 + king_sq 2 + rook_sq4 4 + castling/ep/hash/checkers` ≈ `128B`, still `Copy` (arrays of `u8`), still `Chess960`. Provide `just bench-stockfish` alongside to prove `Stockfish GPL-3` `~170 Mnps` vs turbo `~900 Mnps` on same `M1 Max`.
- **Why:** `blind-base` rewrite removes the `mailbox` need (import → `moves2` via `scan`, not `piece_at` hot). `Gigantua` *recalculates* > remembering `from|to` (2× win) and `Chess_Movegen` shows compact tables matter. `ultrachess ~100B` vs `368B` is the `clone 430 vs 3.3ns` root; `chessgen` `1680→160B` (`piece_cache[64]` removal) is same lesson. With rewrite allowed, compact is not a probe — it's the default to **beat 836**.
- **Trade-off:** `FEN write` `77→82ns` (+5ns) but `perft 387→620 Mnps` (+60%) and `movegen 65→30ns` — net win for `perft`/`SAN` dominated workloads. `FEN parse` still `X-FEN` outermost-rook search (parse-only, not perft).

- **Measured outcome (2026-09-02, M1 Max, close-gap task 4.1):** compact layout landed as the **default**: `Board` = **144B** (`size_of::<Board>()`, from 368B; ultrachess ~100B) via `mailbox` removal (`piece_code_at` scans 12 bbs with occupancy early-out), `castle_mask` removal (`castle_rights_after` derives rights-clearing from `castle_rook_sq[4]` + the mover's role — one subtle bug found and fixed: `castle_right_bit` returns a bit *position*, not a mask), and `occupied` derivation (`occ[0] | occ[1]`). **Clone 430ns → 3.5ns (122×, ultrachess parity 3.3ns)**; movegen unchanged (A/B ≤ old); fen_write 77→~100ns (12-scan bitboard stamp in `to_fen` restores most of the naive cost; in-window A/B ~+12% vs old — accepted and documented, `blind-base` rewrite will absorb it). Full `cargo test` green incl. all 6 CPW perft suites, fuzz-differential, and a new 100-position Chess960 FEN round-trip (byte-equal + zobrist parity).

### D4: SAN Visitor-Free Mate Check

- **Decision:** `move_to_san` already gates `has_no_legal_moves` behind `in_check()` (D3 `0.32ns`), but still does `legal_moves().is_empty()` (full generation) when in check. Replace with `count_legal_moves()==0` (bulk `popcount` without `pop_lsb`) or visitor count, reusing the same `in_check` cache. Disambig already uses `attackers_bb` pre-filter + `is_pseudo_legal` + `make/unmake` attackers check (no `legal_moves()` scan unless `attackers_bb !=0`). Together SAN should drop `3.68→≤2.0µs/48`.
- **Alternative:** Keep `legal_moves().is_empty()` — correct but pays full generation on every check position (rare but SAN bench includes many checks).
- **Measured outcome (2026-09-02, M1 Max, close-gap task 5.1):** `count_legal_moves()==0` landed (strictly ≤ the old `legal_moves().is_empty()` — same generator, no `Move` materialisation). Measured san_48 **≈ neutral** (3.91 µs vs 3.69 µs baseline band; startpos line contains few in-check positions, so the mate check was rarely the bottleneck — the residual SAN cost is disambiguation + suffix, not the mate scan). The `≤2.0 µs` target is **not met** and the remaining gap is documented as deliberate (disambiguation correctness work, follow-up). SAN round-trip byte-equal to shakmaty still PASS (`fuzz-differential`, `san_parity`).

### D5: Gigantua/Stockfish Study — MIT-Safe

- **Decision:** `BENCH.md` stretch row adds Gigantua with note `CPOL (not MIT, study only)` citing `CodeProject 5313417` and `Gigantua/Chess_Base.hpp: Lookup_Pext` (PEXT vs Fancy, `is_constant_evaluated` fallback for Zen1/2 microcode, `Agner Fog` `reciprocal throughput 0.25`). Stockfish noted as `GPL-3`. Technique inventory in `design.md: D1-D4` cites Gigantua's visitor, template expansion, no hashing/incremental bitboards, `Perft aggregate` numbers, but **no code is copied** — only MIT techniques (`ultrachess` `MoveSink`, `cozy-chess` keys) are reused clean-room. `Chess_Movegen` repo (comparison of sliding lookups) is referenced for `Lookup_Pext` vs `Lookup_Fancy` choice.
- **Why:** Users ask to close gap via C++; we must answer legally. `turbochess-rs` is `100% MIT` — `GPL`/`CPOL` copy would violate `AGENTS.md: NEVER copy GPL`. Study-only answers the curiosity while keeping permissive license.

## Risks / Trade-offs

- **[Risk]** Visitor + colour templates increase monomorphisation (`×4` combos) and compile time (`Gigantua.cpp -flto` 10 min on Zen1).
  - **Mitigation:** `lto=fat` already, `codegen-units=1`; gate template behind `#[inline(always)]` and measure `.text` bloat `<5%`.
- **[Risk]** Compact nibble adds per-access shifts, may hurt `SAN`/`FEN` (hot `piece_at`).
  - **Mitigation:** Feature-gated, default off; `BENCH.md` reports `±%` and we keep default `368B` if not `>3%` win.
- **[Risk]** Baseline frozen on `M1 Max` single host, `±3%` not absolute.
  - **Mitigation:** `min of N` per `BENCH.md: Methodology caveat 2`, report `median ±3%` not `Mnps`.
- **[Risk]** Gigantua `PEXT` fast on Ryzen 5000 (Zen 3) but slow microcode on Zen1/2 (Richard Delorme report `151 vs 555 Mnps`); `Lookup_Fancy` fallback needed.
  - **Mitigation:** Keep `pext` feature with runtime `is_x86_feature_detected!("bmi2")` fallback to `Fancy Magic` (already in `attacks.rs`).

## Migration Plan

1. Land `BENCH.md` gap report + `C++ Study` docs (no `src/` change) — frozen baseline stays.
2. Land `MoveVisitor` + `perft_visitor` (additive API, `LTO` already).
3. Land colour-templated `generate_moves_templated::<WHITE>` behind wrapper (no API break).
4. Add `compact` feature probe (default off, `Copy` retained).
5. Each PR diffs `cargo bench --bench micro` median `>3%` vs `turbochess-rs-baseline.json` + `cargo test` green, else revert (like `ultrachess` gate).

## Open Questions

- Keep `BENCH_ITERS` as `criterion` default or fix `10k` like `ultrachess` `micro.rs`?
- Accept `movegen 65 vs 25` / `clone 430 vs 3.3` as `wont-fix` (ultrachess does) or chase `compact` further (we now measure, not chase by default)?
