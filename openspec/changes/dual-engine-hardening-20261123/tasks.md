# Implementation tasks — gigachess-rs

## Correctness (first slice)
- [x] Confirm compile-time Zobrist keys and value-copy Board prevent JS bugs
- [x] Add `Board::repetition_key`, preserve `zobrist()` Polyglot semantics
- [x] Harden exact side token, FEN arity and opposite side king legality
- [x] Preserve synthetic invalid analysis FENs via explicit `parse_fen_allow_illegal`
- [x] Correct two illegal Chess960 test fixtures without changing castling cases
- [x] Add pinned/legal EP, Polyglot, moves2 and castling regression tests
- [x] Create shared normative OpenSpec contract identical to JS counterpart
- [x] Re-run entire Rust `cargo test --locked` suite after changes (all passed)
- [x] Verify parity with JS golden vectors, incl. Chess960 and EP; 10 FENs/8 words
- [ ] Extend cross-language clock, SAN and replay fixtures

## Extended correctness
- [ ] Expand fuzz with random Chess960 positions and invalid FEN mutations
- [ ] Validate trusted/untrusted replay and make variant documented cache rules
- [ ] Test arbitrary `u16` packed streams, unmake LIFO, null move in check
- [ ] Confirm API compatibility with existing Rust downstream consumers
- [ ] Compare legal moves, SAN and complete FEN across Shakmaty & cozy-chess

## Benchmarking / maintenance
- [ ] Run reproducible Rust `benches/vs_libraries` with pinned versions
- [ ] Add ferrum-movegen, chessie, owlchess as licensed/compatible benchmarks
- [ ] Measure BMI2 PEXT and fancy-magic fallback separately, including startup
- [ ] Analyze memory allocs and cache misses in FEN/replay, not only perft
- [ ] Update README, release notes and fastest claims with independent data
