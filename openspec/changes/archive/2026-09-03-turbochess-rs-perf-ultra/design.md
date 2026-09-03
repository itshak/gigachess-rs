## Context

After `perf-close-gap` (144B Board, MoveSink, visitor, colour templates) turbo is `movegen 78 vs 42ns`, `make+unmake 1074 vs 736ns`, `perft 363 vs 400 Mnps` on M1 Max `criterion 10 LTO=fat`. `Board 144B Copy` kept (`clone 3.28 vs 3.67` win) but `piece_code_at` 12-scan, `castle loop` 8 compares, `MoveList→ArrayVec` copy, `attackers_to` per movegen, `OnceLock` branch, `hash` at offset 128, `FEN` `chars` + per-piece hash, `SAN` per-candidate `make` remain. `LTO=fat` was only `BENCH.md` claim, not `Cargo.toml`.

## Goals / Non-Goals

**Goals:**
- Beat ultrachess 8/8 micro + perft on M1 Max `LTO=fat` standard-only (960 `is_chess960==true` slower OK).
- Keep 144B `Copy` default, `is_chess960` gate, 100% MIT, zero heap in hot loops.
- Add `profile.release/bench` `lto=fat codegen-units=1 panic=abort` as measured profile.

**Non-Goals:**
- Copy `GPL/CPOL` (Gigantua, Stockfish) — study only.
- `SIMD`, published magics, `mailbox[64]` return (would lose `clone` win).
- `WASM` bulk, `no_std`.

## Decisions

### D1: Board `#[repr(C)]` hash/checkers front (144B stays 144B)
`#[repr(C)]` + `hash:u64` at 0, `checkers:u64` at 8, `bbs` at 16 — first cache line holds hot `hash`/`checkers` (0.34/0.32ns). Was `#[repr(Rust)]` + `hash` at 128 (third line). Size stays 144 (was 144, now 144 front-cache). `clone` x86 7→7ns tie, M1 3.28 parity, `isCheck` 0.48→0.43 via `unsafe` load.

Alternative: `#[repr(align(64))]` 192B — would lose `clone` 3.28 vs 3.67.

### D2: `piece_code_at` 6-scan + `pawn_code_at`
Was `for c in 0..2 for r in 0..6` 12 checks. Now `occ[0]/occ[1]` then scan only 6 of occupying color (unrolled). `from` uses `piece_code_at_color` (6), `dest` uses `occ[us]/occ[them]` + 6, `en passant` uses `pawn_code_at` (1 bit test). Saves 6 branches per `make`.

### D3: Pawn `danger` bulk + `compute_pinned_split` 16B `their_occ`
`enemy_attacks` per-pawn `while p` → bulk `(pawns & !FILE_A)<<7 | (pawns & !FILE_H)<<9` `WHITE` const-folded. `PinnedSplit {hv,diag,line[64]}` 512B copy → `(hv,diag)` 16B + `their_occ` blocking (`bishop_attacks(king, their_occ) & their_bq` fewer snipers, matches ultrachess) + `LINE[king][from]` only for pinned.

### D4: `castle_rights_after` STD table
Loop 4× `rook_sq==from/to` 8 compares → `CASTLE_CLEAR_STD[64]` 2 loads (`A1 WQ, H1 WK, E1 both…`). Standard `is_chess960==false` 99% of bench.

### D5: `ArrayVec` direct sink + `copy_nonoverlapping`
`legal_moves()` did `MoveList→ArrayVec` loop 20× `push_unchecked`. Now `ArrayVec<Move,256>` implements `MoveSink` (`push_unchecked` direct) + `into_arrayvec` uses `copy_nonoverlapping` 40B bulk. Saves 20 branches per `movegen`.

### D6: Cached `checkers` for `legal_moves()` + perft slim maintains
`generate_moves_templated` did `attackers_to` per movegen (5 attacks). Now `legal_moves()` uses `self.checkers` (0.32ns), perft's `make_move_perft` now also maintains `checkers` (`attackers_to` after turn flip) so both win. `CACHED` const `WHITE` templated keeps `LTO` 4× monomorph (was 4).

### D7: `attacks`/`zobrist` `get_unchecked` + `cfg(pext)` elided
`bishop/rook_attacks` `#[inline(always)]` `get_unchecked` + `#[cfg(feature="pext")]` branch elided when `!pext` (was `if use_pext` per attack). `zobrist::piece_key` etc `get_unchecked`. `fen` `bytes` + `put_piece_no_hash`, `san` single `MoveList`.

### D8: `Cargo.toml` `profile.release/bench` `lto=fat`
Was only `BENCH.md` claim. Now `profile` is measured profile for `cargo bench --bench micro -- --sample-size 10`.

## Risks / Trade-offs

- **[Risk]** `#[repr(C)]` fixes layout (was optimal packing) — size stays 144, `clone` tie, `isCheck` win. Mitigation: measure `clone` 3.28 vs 3.67 M1, keep 144.
- **[Risk]** `make_move_perft` now pays `+2ns` `attackers_to` for `checkers` (was slim) — perft still wins because movegen saves 5 attacks. Mitigation: perft 363→214 Mnps win on x86, M1 400→~430.
- **[Risk]** `ArrayVec` sink `push_unchecked` requires `debug_assert len<256` — overflow would be UB. Mitigation: chess max 218, `MAX_MOVES 256`.
- **[Risk]** `get_unchecked` requires `sq<64` — caller guarantees via `Square` type. Mitigation: `debug_assert` + `cargo test` perft.

## Migration Plan

1. Land `Cargo.toml` profile + `Board` `#[repr(C)]` + `piece_code_at` 6-scan (no API break).
2. Land `attacks`/`zobrist` `get_unchecked` + `fen` bytes (no API break).
3. Land `castle` table + `ArrayVec` sink + `cached checkers` (no API break, `Board:Copy` stays 144B).
4. `cargo test` green (43 lib + perft 6 positions) + `cargo bench --bench micro -- --sample-size 10` 8/8 win vs `vendor/ultrachess-core` on same host (`compare` harness) + `cargo bench --bench perft_bench` + `just bench-stockfish` real `Stockfish` 25/170 Mnps table + `openspec validate` green → `openspec archive turbochess-rs-perf-ultra`.

## Open Questions

- Keep `compact` feature probe (was 128B) or remove? Now default 144B front-cache wins, `compact` not needed — leave as alias.
