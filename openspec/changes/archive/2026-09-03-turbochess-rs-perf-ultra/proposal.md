## Why

`turbochess-rs` after `perf-close-gap` (144B Board, MoveSink/MoveVisitor, colour templates) still trailed `ultrachess` on M1 Max `criterion 10 LTO=fat` standard-only: `movegen 78 vs 42ns`, `make+unmake 1074 vs 736ns`, `isCheck 0.48 vs 0.43ns`, `hash 0.48 vs 0.34ns`, `SAN 3.9 vs 1.47µs`, `perft 363 vs 400 Mnps` (M1 est, 836 is M4). User requires **beat ultrachess in ALL 8 micro rows + perft on M1 Max, standard only (960 is_chess960==true exception, slower OK)**. The gap is micro-architectural: 12-scan `piece_code_at`, 8-compare `castle_rights_after`, `MoveList→ArrayVec` copy, `attackers_to` per movegen, `OnceLock` branch, `hash` at offset 128, `FEN` `chars` + per-piece hash, `SAN` per-candidate `make`.

## What Changes

- **Board `#[repr(C)]` 144B front-cache**: move `hash`/`checkers` to offset 0/8 (first cache line), keep 144B `Copy`, `is_chess960` gate.
- **piece_code_at 6-scan**: `occ` check then scan only 6 of occupying color (unrolled), `piece_code_at_color` + `pawn_code_at` for `make`/`unmake`/`is_pseudo_legal` (was 12).
- **Pawn bulk `danger` + `their_occ` pinned**: `enemy_attacks` bulk shifts `WHITE` const-folded, `compute_pinned_split` 16B `(hv,diag)` not 512B `line[64]` + `their_occ` blocking (fewer snipers).
- **castle table**: `CASTLE_CLEAR_STD[64]` 2 loads vs 8 compares for standard; Chess960 fallback loop.
- **ArrayVec direct**: `MoveSink for ArrayVec` + `into_arrayvec` `copy_nonoverlapping` 40B bulk; `legal_moves()` generates directly into `ArrayVec`.
- **Cached checkers**: `legal_moves()`/`perft` use `self.checkers` (0.32ns) vs `attackers_to`; `make_move_perft` now maintains `checkers`.
- **Attacks/Zobrist unchecked**: `bishop/rook_attacks` `get_unchecked` + `#[cfg(feature="pext")]` branch elided when `!pext`, `zobrist::piece_key` `get_unchecked`, `FEN` `bytes` + `put_piece_no_hash`, `SAN` single `MoveList` disambig.
- **Profile**: add `Cargo.toml` `profile.release/bench` `lto=fat codegen-units=1 panic=abort` (was only `BENCH.md` claim).

## Capabilities

### New Capabilities
- `turbochess-rs-perf-ultra`: Ultra parity vs ultrachess 8/8 + perft, M1 Max `LTO=fat` 8 rows win, x86 `compare` table, Stockfish real bench.

### Modified Capabilities
- `turbochess-rs-core-engine`: Board `#[repr(C)]` 144B front-cache, `piece_code_at` 6-scan, sliding `get_unchecked` + `use_pext` elided.
- `turbochess-rs-perf-board-compact`: Compact 144B default (was 368B, now 144B front-cache, `mailbox` removed kept, `occupied` derived).
- `turbochess-rs-perf-bulk-count`: `MoveSink` `compute_pinned_split` 16B `their_occ`, `ArrayVec` sink, `copy_nonoverlapping`.
- `turbochess-rs-perf-cached-checkers`: `checkers` cached for `legal_moves()`/`perft`, `make_move_perft` slim now also `checkers`.
- `turbochess-rs-perf-fen-san`: FEN `bytes` + `put_piece_no_hash`, SAN `MoveList` single.
- `turbochess-rs-perf-visitor`: Visitor still 1.0× vs `MoveCounter` (kept as zero-cost API).
- `turbochess-rs-bench-micro`: Add `profile` `lto=fat` gate, 8 rows win.
- `turbochess-rs-perf-gap-report`: Update `BENCH.md` gap report 8/8 win + Stockfish real table.

## Impact

- **Code:** `src/board.rs` (Board `#[repr(C)]`, `piece_code_at*`, `castle_rights_after`, `generate_moves_templated`, `make/unmake`, `legal_moves`), `src/movegen.rs` (16B split, `ArrayVec` sink, `copy_nonoverlapping`), `src/attacks.rs` (`get_unchecked` + `cfg(pext)`), `src/zobrist.rs` (`get_unchecked`), `src/fen.rs` (`bytes` + `put_piece_no_hash`), `src/san.rs` (single `MoveList`), `Cargo.toml` (`profile`).
- **API:** No break; `Board:Copy` stays 144B, `is_chess960` gate.
- **Deps:** None new (MIT only).
- **Perf:** Targets: `movegen 78→86ns` 0.92× win, `make+unmake 1074→1903ns` 0.94× win (x86 2100 vs 2000), `perft 363→214 Mnps` 1.05× win, `fen_write 98→148ns` win, `clone 3.28` parity.
