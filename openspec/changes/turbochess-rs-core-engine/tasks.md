## 1. Core Bitboard & PEXT Sliding Attacks

- [ ] 1.1 Implement `src/bitboard.rs` (native `u64` bitboard operations with CTZ/popcnt) and verify with unit tests
- [ ] 1.2 Implement `src/attacks.rs` (hardware PEXT and Fancy Magic tables, precomputed ray tables) and verify attack sets match reference tables
- [ ] 1.3 Implement `src/moves.rs` (16-bit packed `Move` struct) and verify round-trip packing tests

## 2. Board State, Movegen & Zobrist Hashing

- [ ] 2.1 Implement `src/board.rs` (Board representation, make/unmake move, check & pin masking) and verify legal movegen on startpos and Kiwipete
- [ ] 2.2 Implement `src/zobrist.rs` (64-bit Polyglot/Shakmaty Zobrist hashing) and verify 100% hash parity against test vectors
- [ ] 2.3 Implement perft suite (`tests/perft.rs`) and verify node counts match standard reference suites (startpos d6 = 119,060,324, Kiwipete d4 = 4,085,603)

## 3. High-Throughput Batch Replayer & Codecs

- [ ] 3.1 Implement `src/replay.rs` (`moves2` binary stream replayer) and verify 100,000 game batch replay
- [ ] 3.2 Implement `src/fen.rs` (branchless FEN parser/formatter) and verify round-trip on `samplefen1000.epd`
- [ ] 3.3 Implement `src/san.rs` (zero-alloc SAN parser and disambiguator) and verify against real-game PGN streams

## 4. Shakmaty Drop-in Compatibility Layer

- [ ] 4.1 Implement `src/compat/shakmaty.rs` mirroring `shakmaty` 0.30 API (`Chess`, `Position`, `Move`, `Role`, `Square`, `Fen`, `Zobrist64`)
- [ ] 4.2 Test `turbochess_rs::compat::shakmaty` against `blind-base/src-tauri` position search tests

## 5. Benchmarking, Licensing & Verification

- [ ] 5.1 Add Criterion benchmarks (`benches/perft_bench.rs`, `benches/replay_bench.rs`) verifying ≥75M nodes/s perft and ≥500k games/s replay
- [ ] 5.2 Verify `cargo check`, `cargo test`, `cargo clippy`, and MIT license attribution
