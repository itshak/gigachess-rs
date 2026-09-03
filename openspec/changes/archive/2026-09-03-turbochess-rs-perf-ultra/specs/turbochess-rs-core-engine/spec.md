## MODIFIED Requirements

### Requirement: Sliding Attacks SHALL Execute via PEXT with Fancy Magic Fallback

The system SHALL implement sliding attacks via `hardware _pext_u64` when `pext` feature on BMI2 else compact Fancy Magic fallback, with `colour-templated` `generate_legal_templated::<const WHITE: bool>` and `MoveVisitor` path, both monomorphised `LTO=fat`. `attacks::bishop/rook_attacks` SHALL be `#[inline(always)]` `get_unchecked` and `#[cfg(feature="pext")]` branch elided when `!pext`.

#### Scenario: Perft node count parity
- **WHEN** perft is evaluated on startpos and Kiwipete via `MoveVisitor` or `MoveSink`
- **THEN** node counts match standard reference at depths 1..6 (e.g. startpos d5=4865609)

#### Scenario: Attacks unchecked
- **WHEN** `cargo bench --bench micro` `movegen_one_shot` is run
- **THEN** median drops >10% vs baseline without `pext` branch (was 121ns) toward ultrachess 42ns

### Requirement: Move Representation SHALL Use 16-bit Packed Encoding

The system SHALL represent moves as `u16` (`from|to<<6|promo<<12`) and `Board` SHALL be `#[repr(C)]` 144B `Copy` with `hash:u64` at offset 0, `checkers:u64` at 8, `bbs 96` at 16, `occ 16` at 112, `king_sq 2` at 128 (first cache line hot `hash/checkers`), `profile.release`/`bench` `lto=fat codegen-units=1 panic=abort`.

#### Scenario: Zero-allocation legal move generation
- **WHEN** `board.legal_moves()` is called
- **THEN** it returns an `ArrayVec<Move, 256>` allocated strictly on the CPU stack with 0 heap allocations

#### Scenario: Zero-allocation visitor perft
- **WHEN** `board.perft_visitor(depth)` via `CountingVisitor` is called
- **THEN** it counts leaf nodes without materialising `Move` values and without `pop_lsb`

### Requirement: Board SHALL Be Copy and Support Pseudo-Legal Generation

The system SHALL make `Board` a `Copy` type (bit-for-bit copy semantics for engine hot paths) and SHALL expose `pseudo_legal_moves()` generating moves without king-safety filtering, for engines that apply their own legality handling. The system SHALL keep `Board:Copy` in both default and `compact` feature layouts.

#### Scenario: Copy semantics
- **WHEN** a `Board` is copied before making a move
- **THEN** the copy is a bit-for-bit snapshot and the original is unchanged after the move is applied to the copy

#### Scenario: Chess960 gate
- **WHEN** `board.is_chess960()==false` (standard)
- **THEN** `castle_rights_after` uses `CASTLE_CLEAR_STD[64]` table (2 loads) not 4-loop
