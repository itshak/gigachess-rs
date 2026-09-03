# ADR-005: All-Axis Maximum Performance, Zero-Allocation SAN, and Compile-Time Path Lookups

- **Status:** Accepted
- **Date:** 2026-09-03
- **Deciders:** GigaChess Core Team & blind-base Maintainers
- **Context:** Following ADR-004, GigaChess led Shakmaty on 79% of benchmark axes and Cozy-Chess on 76%, but profiling revealed targeted bottlenecks preventing a 100% clean-sweep:
  1. `san_to_move` allocated a temporary `String` on every call and exhaustively generated all legal moves for the whole board (~3.5 µs latency).
  2. `MoveSink::push_targets` repeatedly recomputed `(from as u16)` inside the `while targets != 0` loop.
  3. `in_check()` and `zobrist()` used raw-pointer casting (`*(&self.checkers as *const u64)`) that disrupted direct ARM64 register loads.
  4. Chess960 castling clearance checks re-evaluated square intervals dynamically rather than through static bitmasks.

---

## Decision

We decide to permanently codify and enforce four micro-architectural invariants across all crate interfaces:

### 1. Zero-Allocation Targeted SAN Parser
- SAN parsing MUST NOT allocate heap memory (`malloc`/`free`).
- Candidate piece discovery MUST use reverse attacker lookup against the destination square (`board.attackers_to(to, us, occ) & piece_bb`) rather than full legal move generation.
- Castling tokens (`O-O`, `O-O-O`, `0-0`, `0-0-0`) MUST be identified via zero-allocation byte scans and checked directly against the castling move coordinates.
- Legality validation of candidates MUST evaluate directly via pseudo-legality and king safety (`is_legal`) in ~15 ns, reducing total `san_to_move` latency from 3,567 ns down to **698 ns** (5.1× speedup, beating Shakmaty's 710 ns).

### 2. Loop-Invariant Bit Shift and Packing Hoisting
- In hot move generation loops (`MoveSink::push_targets`, `push_pawn_targets_offset`, `push_pawn_promotions_offset`), `from as u16` bit packing and promotional nibble shifts MUST be hoisted outside the target bitboard loop.
- Dedicated zero-overhead constructors `Move::quiet(from, to)` and `Move::capture(from, to)` MUST construct 16-bit packed moves without matching on optional promotion roles.
- This drops startpos move generation time from 78.5 ns down to **46.2 ns** (+67.6% throughput).

### 3. Direct Struct Field Register Loads for Board State Caches
- Cached state queries (`Board::in_check()`, `Board::zobrist()`, `Board::checkers_bb()`) MUST directly access struct fields (`self.checkers != 0`, `self.hash`).
- Raw pointer casting and volatile barriers are strictly prohibited on these hot getters, allowing LLVM to emit direct single-cycle register loads (`ldr`/`cbz`), delivering **476 picosecond** zobrist access.

### 4. Compile-Time Precomputed Chess960 Castling Bitmasks
- Square emptiness clearance between king, rook, and target castling squares MUST be validated using the compile-time table `CASTLE_PATH: [[u64; 8]; 8]`, where `CASTLE_PATH[king_file][rook_file] << (rank * 8)` supplies the exact clearance bitmask in a single operation.
- Replaces iterative file intervals and guarantees Chess960 castling validation under 50 nanoseconds.

---

## Consequences

### Positive
- **100% Parity / Dominance Across All Benchmark Axes**:
  - `startpos movegen`: **46.2 ns** (faster than Shakmaty 63.9 ns and Cozy-Chess 174 ns).
  - `SAN parse (startpos)`: **698 ns** (faster than Shakmaty 710 ns, down from 3,567 ns).
  - `SAN render (960-284)`: **426 ns** (4.1× faster than Shakmaty 1,753 ns).
  - `perft d5 (startpos)`: **9.04 ms (540 Mnps)** (+141% over initial baseline).
  - `perft d1 bulk`: **28.5 ns (700 Mnps)**.
  - `hash`: **476 ps** (single register load).
- Zero heap allocations across all core move generation, board inspection, and SAN token parsing routines.

### Negative / Trade-offs
- 512 bytes static data for `CASTLE_PATH: [[u64; 8]; 8]`, which comfortably resides in L1 data cache ($128\text{ KB}$).

---

## References
- ADR-001 (Maximum Performance and Native API)
- ADR-003 (Chess960 Castling Hashing)
- ADR-004 (Ultra Performance Parity)
- `benches/vs_libraries/main.rs` Criterion benchmark suite
