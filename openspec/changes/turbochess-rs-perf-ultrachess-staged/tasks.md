## 1. Benchmark Extension (MUST be first, no engine changes)

- [ ] 1.1 Add `benches/micro.rs` `criterion` `8 rows` (`FEN write`, `FEN parse`, `movegen one-shot`, `make+unmake 48-ply`, `isCheck in/out`, `hash`, `SAN 48`, `clone`) with `Throughput::Elements`, matching `ultrachess/BENCH.md` shape
- [ ] 1.2 Make `just bench` gate on `cargo test` pass before bench (like `ultrachess` `gate + perft NPS`) and write `benches/results/turbochess-rs-baseline.json` + `BENCH.md` table, commit as baseline tag
- [ ] 1.3 Run `just bench` + `cargo bench --bench vs_libraries` on `M-series`, freeze `benches/results/turbochess-rs-baseline.json`, verify `cargo test` green
- [ ] 1.4 `openspec validate --change turbochess-rs-perf-ultrachess-staged` green

## 2. Patch 1 — MoveSink Bulk + Split Pins / Bulk Pawns (`count_legal_moves`, parity core)

- [ ] 2.1 Introduce `MoveSink` trait (`push_targets`, `push_pawn_targets_offset`, `push_pawn_promotions_offset`, `push_one`) and `MoveList` (`MaybeUninit<Move,256>`) + `MoveCounter` sinks in `src/movegen.rs`, extract `generate_moves_into<S:MoveSink>` from `board.rs:319`, add `compute_pinned_split→(pinned_hv,pinned_diag)` + bulk pawn shifts (promo split) copying `ultrachess/movegen.rs:31`
- [ ] 2.2 Add `count_legal_moves(&self)->u32` via `MoveCounter`, wire `perft` `depth==1` to counter path (the `geomean 1.23× vs cozy` win, `BENCH.md: caveat 6`)
- [ ] 2.3 Verify `cargo test --test perft` `6 positions` depths `6/7` `position 3` `PASS` and `cargo bench --bench perft_bench` median `>3%` `Mnps` vs baseline **and** `vs_libraries` perft bulk `geomean` moves toward ultrachess `836 Mnps`; else revert. Check `count_legal_moves == legal_moves.len()` on all refs.

## 3. Patch 2 — Cached Checkers + Perft Slim (`Undo.prev_checkers`)

- [ ] 3.1 Extend `src/board.rs`/`src/moves.rs` `Undo { prev_checkers:Bitboard, prev_zobrist:u64, prev_castling, prev_ep, prev_halfmove, captured }` and `Board { checkers:Bitboard, zobrist:u64, history_hashes:Vec<u64> }` maintained in `make`/`unmake` + add `make_move_perft/unmake_move_perft` slim (skip `zobrist`/`history_hashes`/`halfmove`) like `ultrachess/position.rs:42/389`, make `in_check()` `checkers !=0`, `hash()` load `0.34ns`
- [ ] 3.2 Verify `position 3` `en-passant discovered-check` still correct, `cargo test` pass, and `cargo bench --bench micro` `isCheck in/out` `ns/op` drops `>10%` to `≈0.32ns` and `make+unmake 48-ply` not regressed vs baseline **beyond known `BENCH.md: Deliberate 503ns vs cozy 353ns` gap which is kept**; else revert. Verify `perft` uses slim path and still `PASS`.

## 4. Patch 3 — FEN/SAN Branchless

- [ ] 4.1 Branchless `src/fen.rs` `write_fen` via `const PIECE_CHAR:[u8;12]` `ArrayVec<u8,128>` without `format!` and `src/san.rs` `move_to_san(&mut Position)` via `tables::between` + `make/unmake` suffix + disambig `attacks_from_target` pre-filter, copying `ultrachess/src/fen.rs:189` `88ns` and `src/san.rs:1` `1.43µs/48` paths
- [ ] 4.2 Verify `cargo bench --bench micro` `FEN write` `ns/op` drops `>10%` toward `88ns` and `SAN 48` toward `1.43µs` and `1k` random `FEN` round-trip byte-equal vs `shakmaty`; else revert

## 5. Hardening & Archive — Parity Gate

- [ ] 5.1 Add `tests/fuzz-differential.rs` `1k` random games vs `shakmaty` lockstep `FEN`+`legal`+`check` per ply, enforce `cargo llvm-cov --fail-under-lines 95` on `movegen`+`zobrist` (like `ultrachess TESTING.md: just coverage`)
- [ ] 5.2 Re-run `just bench` + `cargo bench --bench vs_libraries` + `cargo bench --bench micro` after kept patches, update `benches/results/turbochess-rs-after.json` and `%` vs baseline `md`; verify **≥ ultrachess in most — preferably all — perft 6 positions + micro 8 rows** on `M-series` (`LTO=fat`, `codegen-units=1`), else document gap with follow-up issue and technique per `vs_libraries` docs (like `ultrachess BENCH.md: Deliberate 4 losses`); `bench-wasm` stub if needed
- [ ] 5.3 `cargo test && cargo bench --bench perft_bench` green, `openspec validate` green, `openspec archive turbochess-rs-perf-ultrachess-staged` with `BENCH.md` update (README head-to-head table refreshed with parity proof, non-Rust Stockfish `400-500 Mnps`/`Gigantua 2.1Gnps` stretch targets)
