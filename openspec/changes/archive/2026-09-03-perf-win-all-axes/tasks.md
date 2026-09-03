# Tasks: All-Axis Maximum Performance & Zero-Overhead Optimization

## 1. Movegen & Invariant Hoisting

- [x] 1.1 Add `Move::quiet(from, to)` and `Move::capture(from, to)` constructors in `src/moves.rs` and verify unit tests with `cargo test --lib moves`
- [x] 1.2 Hoist `(from as u16)` bit packing outside the `while targets != 0` loop in `MoveSink for ArrayVec` within `src/movegen.rs` and optimize `push_pawn_targets_offset`
- [x] 1.3 Verify startpos move generation correctness and micro-benchmark with `cargo test --lib movegen` and `cargo bench --bench micro -- movegen_one_shot`

## 2. Direct Register Board Cache Access

- [x] 2.1 Clean up `src/board.rs` by replacing raw-pointer casting in `Board::in_check()`, `Board::zobrist()`, and `Board::checkers_bb()` with direct struct field accesses (`self.checkers != 0`, `self.hash`)
- [x] 2.2 Verify zero regression with `cargo test --test zobrist` and `cargo bench --bench micro -- hash`

## 3. Zero-Allocation Targeted SAN Parser

- [x] 3.1 Eliminate the `String` heap allocation (`s.chars().filter(...).collect()`) in `san_to_move` within `src/san.rs` using zero-alloc stack byte slices
- [x] 3.2 Implement reverse attacker candidate lookup (`board.attackers_to(to, turn, occ) & piece_bb`) in `san_to_move` instead of exhaustive `board.legal_moves()` generation
- [x] 3.3 Verify byte-identical compatibility with Shakmaty across random games via `cargo test --test san_parity` and benchmark with `cargo bench --bench vs_libraries -- san_parse`

## 4. Chess960 Static Castling Path Lookup

- [x] 4.1 Define compile-time `CASTLE_PATH: [[u64; 8]; 8]` bitmask table in `src/board.rs` to replace the iterative file loop in Chess960 castling clearance checks
- [x] 4.2 Verify Chess960 castling rules and perft with `cargo test --test chess960`

## 5. Benchmark Verification & Documentation Refresh

- [x] 5.1 Run `cargo test --all-features` to ensure 100% test pass rate across all suites
- [x] 5.2 Re-run the full head-to-head Criterion benchmark suite (`cargo bench --bench vs_libraries`) and perft suite (`cargo bench --bench perft_bench`) on local hardware
- [x] 5.3 Update `BENCH.md` and `README.md` comparison tables with the newly achieved benchmark numbers and win rates
