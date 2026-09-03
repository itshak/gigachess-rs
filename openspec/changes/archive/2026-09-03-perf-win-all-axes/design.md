# Design: All-Axis Maximum Performance & Zero-Overhead Optimization

## Context

See `proposal.md` for motivation. Profiling and Criterion benchmarks against Ultrachess, Shakmaty 0.30, and Cozy-Chess 0.3 revealed 4 micro-architectural bottlenecks:
1. Heap allocation (`String`) and exhaustive legal move generation in `san_to_move`.
2. Redundant bit shifts and `Option<Role>` matching inside `MoveSink::push_targets`.
3. Raw pointer casts in `Board::in_check()` and `Board::zobrist()`.
4. Iterative square checking for Chess960 castling paths.

## Goals / Non-Goals

**Goals:**
- Eliminate all heap allocations in SAN parsing (`san_to_move`), achieving <500 ns parse latency.
- Hoist loop-invariant calculations in movegen, dropping `startpos` legal movegen under 40 ns.
- Restore single-cycle direct register accesses for `in_check()` and `zobrist()` (matching 0.31–0.34 ns).
- Replace iterative Chess960 castling file traversal with static bitmask lookups (<60 ns target).
- Win or achieve parity on 100% of benchmark axes across Ultrachess, Shakmaty, and Cozy-Chess.

**Non-Goals:**
- Changing wire formats or external serialization formats (`moves2` format remains untouched).
- Changing public API signatures of `Board`, `Move`, `Square`, or `san`.
- Rewriting slider attack generators (PEXT and Fancy Magic algorithms remain untouched).

## Decisions

### Decision 1: Targeted Reverse Attacker Lookup in `san_to_move`
- **Design**:
  1. Replace `String` normalization with zero-allocation stack byte-slice parsing (`&[u8]`).
  2. Parse destination square `to`, moving piece `role`, disambiguation hints (`file`/`rank`), and promotion role directly from the slice.
  3. Instead of `board.legal_moves()`, identify candidate pieces using `board.attackers_to(to, board.turn(), board.occupied()) & board.pieces_of_role(role)`.
  4. For the candidates (typically 1, at most 2 in disambiguated positions), filter against disambiguation hints and verify move legality via `is_legal(mv)`.
- **Alternatives Considered**:
  - *Full legal move scan (current)*: Generates 20–50 moves for the whole board; costs 1,500–2,500 ns.
  - *Compile-time perfect hash table (Shakmaty approach)*: Adds build complexity, increases binary size by dozens of kilobytes, and doesn't handle dynamic Chess960 positions cleanly.
  - *Decision*: Targeted reverse attacker lookup is zero-alloc, dynamic, supports standard & 960 equally, and runs in <400 ns.

### Decision 2: Invariant Hoisting & Specialized `Move` Constructors
- **Design**:
  1. Add specialized constructors in `src/moves.rs`:
     ```rust
     #[inline(always)]
     pub const fn quiet(from: u8, to: u8) -> Move {
         Move((from as u16) | ((to as u16) << 6))
     }
     #[inline(always)]
     pub const fn capture(from: u8, to: u8) -> Move {
         Move((from as u16) | ((to as u16) << 6))
     }
     ```
  2. In `src/movegen.rs`, update `MoveSink::push_targets`:
     ```rust
     #[inline(always)]
     fn push_targets(&mut self, from: u8, mut targets: u64) {
         let from_bits = from as u16;
         while targets != 0 {
             let to = pop_lsb(&mut targets);
             unsafe { self.push_unchecked(Move(from_bits | ((to as u16) << 6))) };
         }
     }
     ```
  3. In `push_pawn_targets_offset`, compute `from` offset arithmetic using 16-bit registers directly.
- **Alternatives Considered**:
  - Relying on LLVM auto-vectorization and loop-invariant code motion (LICM): LLVM fails to hoist `from as u16` across the `pop_lsb` loop due to pointer aliasing in `push_unchecked`. Explicit hoisting guarantees optimal assembly generation.

### Decision 3: Direct Register Reads for Position Caches
- **Design**:
  Replace raw pointer dereferences in `src/board.rs`:
  ```rust
  #[inline(always)]
  pub fn in_check(&self) -> bool {
      self.checkers != 0
  }

  #[inline(always)]
  pub fn zobrist(&self) -> u64 {
      self.hash
  }
  ```
- **Rationale**: Direct field access lets LLVM keep `&self` in a register and emit direct `ldr`/`cbz` instructions (1 cycle, ~0.31 ns on M1 Max) without emitting redundant stack spill or pointer reload sequences.

### Decision 4: Static Bitmask Lookup for Chess960 Castling Paths
- **Design**:
  Define a static table `CASTLE_PATH: [[u64; 8]; 8]` where `CASTLE_PATH[kf][rf]` contains the bitmask of all squares between King and Rook (and between King and final G/C square) that must be unoccupied.
- **Rationale**: Replaces an iterative `for f in min_file..=max_file` loop with a single table lookup `(self.occupied() & CASTLE_PATH[kf][rf]) == 0`, cutting Chess960 castling validation time from ~118 ns down to ~50 ns.

## Risks / Trade-offs

- **[Risk] Reverse SAN lookup might miss edge-case pin legality**:
  - *Mitigation*: Maintain full test parity against `tests/san_parity.rs` (which differential-tests random game SAN parsing against Shakmaty) and `tests/fuzz-differential.rs`.
- **[Risk] Precomputed Chess960 table memory footprint**:
  - *Mitigation*: 8×8 array of `u64` is only $8 \times 8 \times 8 = 512\text{ bytes}$, easily fitting in L1 data cache ($128\text{ KB}$).
