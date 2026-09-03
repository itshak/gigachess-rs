## 1. Board Layout and Attacks

- [x] 1.1 Make `Board` `#[repr(C)]` 144B front-cache (`hash` 0, `checkers` 8, `bbs` 16) and add `Cargo.toml` `profile.release/bench` `lto=fat codegen-units=1 panic=abort` and verify `cargo test --lib` 43 pass and `cargo bench --bench micro -- micro/startpos/clone --sample-size 10` wins vs ultrachess 3.67
- [x] 1.2 Change `piece_code_at` to 6-scan `occ` check + 6 `bbs` unrolled and add `piece_code_at_color` + `pawn_code_at` for `make`/`unmake`/`is_pseudo_legal` and verify `cargo test` perft 6 positions pass and `cargo bench --bench micro` `make_unmake_48` wins vs 736
- [x] 1.3 Change `attacks::bishop/rook_attacks` to `#[inline(always)]` `get_unchecked` + `#[cfg(feature="pext")]` elided when `!pext` and `zobrist::piece_key` `get_unchecked` and verify `cargo bench --bench micro` `movegen_one_shot` wins vs 42

## 2. Movegen Bulk and Pinned

- [x] 2.1 Change `compute_pinned_split` to `(hv,diag)` 16B + `their_occ` blocking and `Board` `pinned==0` fast path (no second loops) and verify `cargo test` perft + `cargo bench --bench micro` `movegen_one_shot` 0.92× win
- [x] 2.2 Change `enemy_attacks` pawn bulk `WHITE` const-folded `(pawns & !FILE_A)<<7|9` and verify `cargo bench --bench micro` `movegen` wins
- [x] 2.3 Add `MoveSink for ArrayVec` direct `push_unchecked` + `into_arrayvec` `copy_nonoverlapping` 40B bulk and `legal_moves()` direct `ArrayVec` and verify `cargo bench --bench micro` `movegen` wins vs ultrachess

## 3. Castle and Cached Checkers

- [x] 3.1 Add `CASTLE_CLEAR_STD[64]` table 2 loads vs 8 compares for `is_chess960==false` and verify `cargo bench --bench micro` `make_unmake_48` wins
- [x] 3.2 Make `generate_moves_templated` use cached `self.checkers` (0.32ns) and `make_move_perft` maintain `checkers` and verify `cargo bench --bench micro` `is_check`/`hash` parity and `cargo bench --bench perft_bench` `perft_d5` wins vs ultrachess 400

## 4. FEN/SAN and Profile

- [x] 4.1 Change `fen::parse_fen` to `bytes` loop + `put_piece_no_hash` and `to_fen` 12-scan and verify `cargo bench --bench micro` `fen_write` <103 and `fen_parse` <208
- [x] 4.2 Change `san::move_to_san` disambig to single `generate_moves_into` `MoveList` vs per-candidate `make` and verify `cargo bench --bench micro` `san_48` <1.47µs and `cargo test` `san` byte-equal
- [x] 4.3 Verify `cargo test` 43 pass + `cargo bench --bench micro -- --sample-size 10` 8/8 win + `cargo bench --bench perft_bench -- --sample-size 10` win vs ultrachess on same host + `just bench-stockfish` real Stockfish 25/170 Mnps table

## 5. Docs and Archive

- [x] 5.1 Update `BENCH.md` gap report 14 axes 8/8 win + `README` + `benches/results/turbochess-rs-after.json` and verify `openspec validate --specs` pass
- [x] 5.2 Create `openspec/adr/004-ultra-performance-parity.md` and verify `openspec validate --specs` pass and `openspec archive turbochess-rs-perf-ultra` succeeds
