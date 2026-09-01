## 1. Core Bitboard & PEXT Sliding Attacks

- [x] 1.1 Implement `src/bitboard.rs` (native `u64` bitboard operations with CTZ/popcnt) and verify with unit tests
- [x] 1.2 Implement `src/attacks.rs` (hardware PEXT and Fancy Magic tables, precomputed ray tables) and verify attack sets match reference tables
- [x] 1.3 Implement `src/moves.rs` (16-bit packed `Move` struct) and verify round-trip packing tests

## 2. Board State, Movegen & Zobrist Hashing

- [x] 2.1 Implement `src/board.rs` (Board representation, make/unmake move, check & pin masking) and verify legal movegen on startpos and Kiwipete
- [x] 2.2 Implement `src/zobrist.rs` (64-bit Polyglot/Shakmaty Zobrist hashing) and verify 100% hash parity against test vectors
- [x] 2.3 Implement perft suite (`tests/perft.rs`) and verify node counts match standard reference suites (startpos d6 = 119,060,324, Kiwipete d4 = 4,085,603)

## 3. High-Throughput Batch Replayer & Codecs

- [x] 3.1 Implement `src/replay.rs` (`moves2` binary stream replayer) and verify 100,000 game batch replay
- [x] 3.2 Implement `src/fen.rs` (branchless FEN parser/formatter) and verify round-trip on `samplefen1000.epd`
- [x] 3.3 Implement `src/san.rs` (zero-alloc SAN parser and disambiguator) and verify against real-game PGN streams

## 4. Benchmarking, Licensing & Verification

- [x] 4.1 Add Criterion benchmarks (`benches/perft_bench.rs`, `benches/replay_bench.rs`) verifying ≥75M nodes/s perft and ≥500k games/s replay
- [x] 4.2 Verify `cargo check`, `cargo test`, `cargo clippy`, and MIT license attribution
