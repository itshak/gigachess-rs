# Proposal: All-Axis Maximum Performance Optimization & Clean-Sweep Parity

## Why

GigaChess currently outperforms Shakmaty (winning 79% of axes) and Cozy-Chess (winning 76% of axes), and ties or beats Ultrachess in 6 out of 9 core axes. However, detailed micro-architectural profiling reveals targeted opportunities where GigaChess still incurs unnecessary CPU cycles:
1. `san_to_move` incurs a heap allocation (`malloc`/`free` of a `String` on every call) and generates all legal moves for the whole board rather than querying target attacks.
2. `push_targets` re-evaluates `(from as u16)` and matches `promo: Option<Role>` inside tight loops for every generated move.
3. `in_check()` and `zobrist()` use raw pointer casts (`*(&self.checkers as *const u64)`) that prevent clean single-cycle ARM64 register loads.
4. Chess960 legal movegen dynamically loops through files to check path clearance rather than using precomputed path bitmasks.

Eliminating these bottlenecks will establish GigaChess as the uncontested fastest chess library across 100% of benchmarked axes.

## What Changes

- **Zero-Allocation Targeted SAN Parser**: Refactor `san_to_move` in `src/san.rs` to parse without heap-allocating `String` buffers, and filter candidate movers via `board.attackers_to(target_sq)` rather than generating all legal moves across the whole board.
- **Loop-Invariant Hoisting & Fast Move Constructors**: Add `Move::quiet(from, to)` and `Move::capture(from, to)` in `src/moves.rs`. Hoist `from as u16` bit packing out of `push_targets` loops in `src/movegen.rs`.
- **Direct Register Access for Board Caches**: Remove raw pointer casting from `Board::in_check()`, `Board::zobrist()`, and `Board::checkers_bb()` in `src/board.rs` to allow LLVM to generate direct 1-cycle instructions.
- **Precomputed Chess960 Castling Masks**: Replace dynamic file traversal with a precomputed `CASTLE_PATH[king_file][rook_file]` bitmask table in `src/board.rs`.
- **Compiler Optimization Flags & Branchless Hints**: Add `#[cold]` annotations to error and checkmate paths, leverage branchless conditional moves (`csel`), and ensure aggressive inlining on hot inner loops.
- **Full Benchmark Validation & Table Refresh**: Re-run Criterion benchmarks (`vs_libraries`, `micro`, `perft_bench`), verify 100% test passing, and update `BENCH.md` and `README.md`.

## Capabilities

### New Capabilities
- `perf-all-axes`: Sub-nanosecond check and hash access, zero-allocation SAN parsing (<500 ns target), sub-40 ns startpos legal movegen, and lookup-based Chess960 castling validation.

### Modified Capabilities
<!-- None: No spec-level public contract or functional behavioral requirements are changing; all changes are strictly non-breaking performance optimizations. -->

## Impact

- **APIs**: Fully backwards-compatible. Public signatures of `Board`, `Move`, `move_to_san`, `san_to_move`, and `parse_fen` remain unchanged.
- **Memory & Allocation**: Heap allocations during SAN parsing reduced to exactly zero.
- **Verification**: `cargo test --all-features` must pass 100% and `cargo bench --bench vs_libraries` must verify wins/parity across all axes.
