## Context
See `proposal.md` for background. `turbochess-rs` provides a 100% MIT-licensed, high-throughput Rust chess library combining the raw speed of `cozy-chess`'s PEXT engine with TurboChess's 16-bit `moves2` packed format, batch database replaying, and Shakmaty compatibility facade.

## Goals / Non-Goals

**Goals:**
- Implement a 100% MIT-licensed native Rust bitboard engine achieving 75M+ nodes/sec.
- Adapt and reuse `cozy-chess` (MIT) PEXT intrinsics and Fancy Magic tables with full clean-room attribution.
- Implement 16-bit `moves2` packed move encoding matching `blind-base`'s binary database format.
- Implement zero-allocation `ArrayVec<Move, 256>` move generation.
- Implement parallel batch game replay (`replay_moves2_batch`) scaling with Rayon.
- Implement `turbochess_rs::compat::shakmaty` drop-in facade for `blind-base`.

**Non-Goals:**
- Implementing non-standard variants (Crazyhouse, Atomic, Antichess) to avoid variant polymorphism slowdowns.

## Decisions

### D1: Crate Structure & Modules
- `src/lib.rs`: Main public crate interface.
- `src/bitboard.rs`: 64-bit `Bitboard(pub u64)` with popcnt, ctz, and bitwise operators.
- `src/attacks.rs`: Sliding attack tables (PEXT + Fancy Magic fallback) and precomputed $64 \times 64$ ray tables.
- `src/moves.rs`: 16-bit packed `Move` struct with `from`, `to`, `promotion` getters and bitwise packing.
- `src/board.rs`: Board state tracking pieces, active turn, castling rights, and en-passant square.
- `src/zobrist.rs`: 64-bit incremental Zobrist hash table (Polyglot / Shakmaty parity).
- `src/fen.rs`: Branchless ASCII table FEN parser & formatter.
- `src/san.rs`: Zero-alloc SAN generator and parser.
- `src/replay.rs`: Batch `moves2` binary stream replayer.
- `src/compat/shakmaty.rs`: Drop-in Shakmaty 0.30 facade.

### D2: Clean-Room & Code Reuse Policy
- Core bitboards, PEXT, and Fancy Magic tables can be derived directly from `analog-hors/cozy-chess` (MIT License) with attribution in `LICENSE-THIRD-PARTY`.
- `moves2` binary encoding and batch replay logic are ported from TurboChess and `blind-base`.
- No GPL code or text is permitted anywhere in the repository.

### D3: Zero-Allocation Stack Discipline
All move generation APIs return `ArrayVec<Move, 256>`, ensuring that move generation in hot loops remains 100% stack-allocated with zero dynamic heap allocations.

## Risks / Trade-offs

- **[Risk]** PEXT instruction is slow on older AMD CPUs (Zen 1 / Zen 2).
  - **Mitigation:** The `pext` feature is optional and enabled via `cargo build --features pext`. Default build uses cache-compact Fancy Magic tables which run at 65M+ nodes/s on all CPUs.
- **[Risk]** API discrepancies with `shakmaty` in `blind-base`.
  - **Mitigation:** The `turbochess_rs::compat::shakmaty` module wraps internal structs to provide identical function names (`Position::play`, `Chess::legal_moves`, `Fen::from_ascii`).
