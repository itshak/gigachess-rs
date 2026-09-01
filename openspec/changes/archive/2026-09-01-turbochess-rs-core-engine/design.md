## Context
See `proposal.md` and `openspec/adr/001-maximum-performance-and-native-api.md`. `turbochess-rs` provides a 100% MIT-licensed, high-throughput Rust chess library designed strictly for maximum performance, zero-allocation move generation, and parallel database replay.

## Goals / Non-Goals

**Goals:**
- Implement a 100% MIT-licensed native Rust bitboard engine achieving 75M+ nodes/sec.
- Adapt and reuse `cozy-chess` (MIT) PEXT intrinsics and Fancy Magic tables with clean-room provenance.
- Implement 16-bit `moves2` packed move encoding matching `blind-base`'s binary database format.
- Implement zero-allocation `ArrayVec<Move, 256>` move generation.
- Implement parallel batch game replay (`replay_moves2_batch`) scaling with Rayon (>500,000 games/sec).

**Non-Goals:**
- Emulating legacy `shakmaty` 24-byte enums (rejected per ADR-001 to avoid a 35% speed penalty).
- Implementing non-standard variants (Crazyhouse, Atomic, Antichess) to avoid variant polymorphism slowdowns.

## Decisions

### D1: Pure Native Architecture (ADR-001)
- Moves are represented strictly as a 16-bit packed `Move(pub u16)` struct:
  $$\text{word} = (\text{from} \mathbin{\&} 0x3f) \mid ((\text{to} \mathbin{\&} 0x3f) \ll 6) \mid ((\text{promo} \mathbin{\&} 0x0f) \ll 12)$$
- Move generation returns `ArrayVec<Move, 256>`, occupying only 512 bytes on the CPU stack.

### D2: Hardware PEXT & Fancy Magic Sliding Attacks
- Uses `_pext_u64` for 1-cycle sliding attacks on x86-64 BMI2 CPUs.
- Automatically falls back to cache-compact Fancy Magic tables on ARM / Apple Silicon.

### D3: Precomputed 64x64 Ray Tables
- `BETWEEN[sq1][sq2]` and `ALIGNED[sq1][sq2]` precalculated into flat continuous memory for instant $O(1)$ check and pin verification.

## Risks / Trade-offs

- **[Risk]** PEXT instruction is slow on older AMD CPUs (Zen 1 / Zen 2).
  - **Mitigation:** The `pext` feature is optional and enabled via `cargo build --features pext`. Default build uses cache-compact Fancy Magic tables which run at 65M+ nodes/s on all CPUs.
