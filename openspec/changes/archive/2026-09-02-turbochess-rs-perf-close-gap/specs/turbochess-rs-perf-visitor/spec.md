## Purpose

Provides a `MoveVisitor` trait for perft/analysis that counts or processes moves without materialising 16-bit `Move` values, removing `pop_lsb` and `ArrayVec` costs (Gigantua visitor is 2× vs movelist, MIT-safe).

## ADDED Requirements

### Requirement: Move Generation SHALL Support MoveVisitor Without Move Materialisation

The system SHALL expose `generate_legal_visitor(&self, visitor: &mut impl MoveVisitor)` where `visitor.visit_targets(from, mask)` receives whole bitboards, `visit_pawn_offset` / `visit_promotion_offset` handle bulk pawn shifts, and perft leaf `depth==1` uses a `CountingVisitor` (`count += popcount`) without constructing `Move` or calling `pop_lsb`.

#### Scenario: Visitor perft leaf equals MoveCounter
- **WHEN** `perft_visitor(board,1)` via `CountingVisitor` and `perft(board,1)` via `MoveCounter` are run on startpos, kiwipete, and position 3
- **THEN** counts are equal and visitor is `>15%` faster median on `cargo bench --bench perft_bench` (vs `MoveCounter`) due to eliminated `Move` materialisation

### Requirement: Visitor SHALL Be Zero-Allocation and Monomorphised

The system SHALL keep `MoveVisitor` generic (`impl MoveVisitor`) with `#[inline]`, monomorphised `×2` (like `MoveSink`), zero heap allocations, and `LTO=fat codegen-units=1` already set.

#### Scenario: No alloc in visitor perft
- **WHEN** `cargo test` runs with `#[deny(alloc)]` harness on `perft_visitor`
- **THEN** no heap allocation occurs for leaf counting
