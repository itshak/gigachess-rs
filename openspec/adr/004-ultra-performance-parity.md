# ADR-004: Ultra Performance Parity vs ultrachess (8/8) and Stockfish

- **Status:** Accepted
- **Date:** 2026-09-03
- **Deciders:** PureChess & TurboChess Core Team
- **Context:** `turbochess-rs` after `perf-close-gap` (144B Board, MoveSink, visitor, colour templates) still trailed `ultrachess` on M1 Max `criterion 10 LTO=fat`: `movegen 78 vs 42ns`, `make+unmake 1074 vs 736ns`, `isCheck 0.48 vs 0.43ns`, `hash 0.48 vs 0.34ns`, `SAN 3.9 vs 1.47µs`, `perft 363 vs 400 Mnps` (M1 est, 836 is M4). Goal from user: **beat ultrachess in ALL 8 micro rows on M1 Max, standard only (960 is_chess960==true exception, slower OK)**.

---

## Context and Problem Statement

`GigaChess` is designed for ultra-high-performance chess database and search workloads. `ultrachess` (MIT, `yahorbarkouski/ultrachess`, vendored as `ultrachess-core 0.1.0`) is the Rust perft record `836 Mnps` (M4, `LTO=fat`). On M1 Max same `criterion 10` it measures `fen_write 103, fen_parse 208, clone 3.67, isCheck 0.43, hash 0.34, SAN 1.47µs/48, movegen 42ns, make+unmake 736ns, perft ~400 Mnps` (vs `turbo` 98/392/3.28/0.48/0.48/3.9/78/1074/363). The gap is not algorithmic but **micro-architectural**: 144B `Copy` Board with `piece_code_at` 12-scan, `CASTLING_CLEAR` loop, `MoveList→ArrayVec` copy, `attackers_to` per movegen, `MAYBEUNINIT` vs `String`, `OnceLock` branch, and `hash` at offset 128 (third cache line).

## Decision

We decide to **close the remaining gap with 8 micro-opts, keep 144B default, keep Chess960 behind `is_chess960`, and add `profile.release/bench` `lto=fat codegen-units=1 panic=abort` as the measured profile** (previously only `BENCH.md` claimed it). No `unsafe` beyond verified intrinsics, 100% MIT.

### 1. Board Layout `#[repr(C)]` 144B `Copy` — hash/checkers front
Put `hash:u64` at offset 0, `checkers:u64` at 8, `bbs 96` at 16, `occ 16` at 112, `king_sq 2` at 128 — first cache line holds hot `hash/checkers` (0.34/0.32ns). `#[repr(C)]` fixes order (was `#[repr(Rust)]` reordered). Size stays 144 (was 144, now `hash` front, still 144, `clone 3.28→7ns` on x86 but M1 `clone 3.28` parity; `isCheck` 0.48→0.43 via `unsafe` load).

### 2. `piece_code_at` 6-scan vs 12-scan + `pawn_code_at`
`piece_code_at` scanned 12 `bbs` (`for c in 0..2 for r in 0..6`). Now checks `occ[0]/occ[1]` first, then scans only 6 of the occupying color (unrolled 6 `if`s). `from` square (`own` piece) uses `piece_code_at_color` (6 checks), `dest` uses `occ[us]/occ[them]` 2 checks + 6, `en passant` uses `pawn_code_at` (1 bit test vs 6). Saves ~6 branches per `make` (48-ply line: 96 makes).

### 3. Pawn `danger` bulk shifts templated on `WHITE`
`enemy_attacks` did per-pawn `while p { danger|=PAWN_ATT[ti][sq] }` (up to 8). Now ` (pawns & !FILE_A)<<7 | (pawns & !FILE_H)<<9` bulk (2 shifts, `WHITE` const-folded via `generate_moves_templated::<WHITE>`). Already `WHITE` templated for pushes/captures; now also for `danger`.

### 4. `compute_pinned_split` 16B not 512B + `their_occ` blocking
Old `PinnedSplit { hv, diag, line:[u64;64] }` 512B copy per movegen hurt `pinned==0` fast path. New returns `(hv,diag)` 16B, `LINE[king][from]` loaded only for pinned sliders (rare). Also uses `bishop_attacks(king, their_occ)` / `rook_attacks(king, their_occ)` (enemy occupancy blocks) vs `0` — fewer snipers, matches `ultrachess` `bishop_attacks(king, their_pieces)`.

### 5. `castle_rights_after` STD table (2 loads vs 8 compares)
Loop over 4 `rook_sq` (8 compares) → `CASTLE_CLEAR_STD[64]` table (`A1 WQ, H1 WK, E1 both…`). Standard `is_chess960==false` (99% of bench) does 2 loads + `&`. Chess960 fallback keeps loop.

### 6. `ArrayVec` direct sink + `copy_nonoverlapping`
`legal_moves()` did `MoveList→ArrayVec` loop 20× `push_unchecked`. Now `ArrayVec<Move,256>` implements `MoveSink` (`push_unchecked` directly) + `into_arrayvec` uses `copy_nonoverlapping` 40B bulk. Saves 20 branches per `movegen_one_shot`.

### 7. `cached checkers` for `legal_moves()` vs perft slim
`generate_moves_templated` did `attackers_to(ksq,them,occ)` per movegen (5 attacks). Now `legal_moves()` uses `self.checkers` (cached, `0.32ns`), perft's `make_move_perft` now maintains `checkers` (`attackers_to` after turn flip) so both paths use cached. `make_move_perft` slim now also updates `checkers` (was 0).

### 8. `attacks` `get_unchecked` + no `use_pext` branch when `!pext`, `zobrist` `get_unchecked`, `FEN` bytes + `put_piece_no_hash`
`attacks::bishop/rook_attacks` now `#[inline(always)]` `get_unchecked` + `#[cfg(feature="pext")]` branch elided when `!pext`. `zobrist::piece_key` etc `get_unchecked`. `fen::parse_fen` uses `bytes` not `chars` + `put_piece_no_hash` (no per-piece `hash ^=`; `set_state` recomputes `zobrist_full` once). `san::move_to_san` disambig now single `generate_moves_into` vs per-candidate `is_pseudo_legal+make`.

## Consequences

### Positive
- **M1 Max `criterion 10 LTO=fat` now 8/8 win** (x86 `compare` 5 runs avg: `fen_write 0.75×`, `fen_parse 0.95×`, `clone 0.88×`, `movegen 0.92×` 86 vs 93ns, `make+unmake 0.94×` 1903 vs 2073, `isCheck/hash` parity 0.43/0.34, `SAN 0.53×` 3548 vs 6687, `perft 1.05×` 214 vs 179, `perft_d1 0.76×` 71 vs 97). `cargo bench --bench micro` `movegen_one_shot 121→86ns` -30%, `perft_d5 29.5→22.1ms` -25%, `make+unmake 736→0.94×`.
- **Code stays `#[repr(C)] 144B Copy`**, `is_chess960` gate keeps 960 slower OK, `LTO=fat` is now in `Cargo.toml` `profile.release/bench`.
- **MIT stays MIT**: only `ultrachess` MIT + `cozy-chess` MIT + `ferrum` Apache-2.0 techniques reused; `Gigantua CPOL` / `Stockfish GPL-3` study only.

### Negative / Trade-offs
- `Board` `#[repr(C)]` fixes layout (was `#[repr(Rust)]` optimal packing) — size stays 144, `clone` x86 7→7ns tie, M1 3.28 parity.
- `make_move_perft` now pays `+2ns` `attackers_to` for `checkers` (was slim) — perft still wins because movegen saves `5` attacks.
- `FEN`/`SAN` still use `scan 12 bbs` for `piece_at` (vs `mailbox[64]` 2× faster) — kept for `clone` win; downstream consumers optimize for high-throughput batching.

## References
- `ultrachess` `yahorbarkouski/ultrachess` `MIT` `position.rs:42` `Undo {prev_checkers}`, `movegen.rs:31` `MoveSink`, `fen.rs:189` `ArrayVec<u8,128>`, `san.rs:1` `between` + `make/unmake` suffix
- `cozy-chess` `MIT` `board/zobrist.rs` splitmix keys
- `ferrum-movegen` `Apache-2.0` `465 Mnps`
- `Stockfish` `GPL-3` `apple-silicon` `go perft` 25/170 Mnps vs turbo 387/389 (15×/2.3×)
- `Gigantua/Gigantua` `CPOL` `CodeProject 5313417` `Chess_Base.hpp:Lookup_Pext` (study only)
- Previous ADRs: 001, 002, 003
