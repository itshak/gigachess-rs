# turbochess-rs-perf-bulk-count Specification

## Purpose
Bulk counting at perft leaves via `MoveSink` generic, avoiding `pop_lsb` — the `geomean 1.23× vs cozy` win (`BENCH.md: caveat 6`) and core of `836 Mnps` parity.

## Requirements

### Requirement: Move Generation SHALL Support MoveSink with Split Pins / Bulk Pawns

The system SHALL expose `generate_legal_moves(&self, sink: &mut impl MoveSink)` where `sink.push_targets(from, mask:Bitboard)` receives whole masks, `MoveCounter` sums `popcount`, `compute_pinned_split→(pinned_hv,pinned_diag)` avoids per-slider `line()` load, and pawns use bulk shifts `north(pawns)&!occ&check_mask` split promo.

#### Scenario: Bulk perft leaf
- **WHEN** `perft(board,1)` is called
- **THEN** it uses `MoveCounter` sink, never pushes to `ArrayVec`, and node count equals materialising path

### Requirement: count_legal_moves SHALL Be Alloc-Free

The system SHALL expose `count_legal_moves(&self)->u32` via `MoveCounter`, `0` heap allocs.

#### Scenario: Perft d6 bulk toward ultrachess parity
- **WHEN** `perft(board,6)` is run
- **THEN** its `depth==1` leaves use counter path, `cargo test --test perft` stays green, `>3%` `Mnps` vs baseline and `geomean` moves toward ultrachess `836 Mnps`
