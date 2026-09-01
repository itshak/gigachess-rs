## Purpose

Defines the core `turbochess-rs` engine architecture: native `u64` bitboards, hardware PEXT / Fancy Magic sliding attacks, 16-bit packed moves (`moves2`), batch game replay, and 64-bit incremental Zobrist hashing.

## ADDED Requirements

### Requirement: Sliding Attacks SHALL Execute via PEXT with Fancy Magic Fallback

The system SHALL implement sliding piece move generation using hardware `_pext_u64` (when compiled with the `pext` feature on BMI2 CPUs) and a compact Fancy Magic lookup table on non-PEXT architectures.

#### Scenario: Perft node count parity
- **WHEN** perft is evaluated on startpos and Kiwipete positions
- **THEN** node counts match standard reference counts at depths 1 through 6 (e.g. startpos depth 5 = 4,865,609, Kiwipete depth 4 = 4,085,603)

### Requirement: Move Representation SHALL Use 16-bit Packed Encoding

The system SHALL represent individual moves as a 16-bit struct (`u16`) where:
- Bits 0..5 (6 bits): `from` square (0..63)
- Bits 6..11 (6 bits): `to` square (0..63)
- Bits 12..15 (4 bits): promotion role (0=none, 1=Knight, 2=Bishop, 3=Rook, 4=Queen)

#### Scenario: Zero-allocation legal move generation
- **WHEN** `board.legal_moves()` is called
- **THEN** it returns an `ArrayVec<Move, 256>` allocated strictly on the CPU stack with 0 heap allocations

### Requirement: High-Throughput Batch Replay Engine

The system SHALL provide `replay_moves2_batch` replaying slices of 16-bit encoded games in parallel, achieving ≥500,000 games/second across CPU cores.

#### Scenario: Batch replay matches FEN positions
- **WHEN** 100,000 games are replayed from binary `moves2` slices
- **THEN** final board hashes and legal statuses are verified with 100% parity
