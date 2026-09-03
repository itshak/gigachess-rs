## MODIFIED Requirements

### Requirement: Move Generation SHALL Support MoveSink with Split Pins / Bulk Pawns

The system SHALL expose `generate_legal_moves` via `MoveSink` where `sink.push_targets(from, mask)` receives whole masks, `MoveCounter` sums `popcount`, `compute_pinned_split` returns `(hv,diag)` 16B (was `PinnedSplit {hv,diag,line[64]}` 512B) with `their_occ` blocking (`bishop_attacks(king, their_occ) & their_bq` fewer snipers, matches ultrachess) and `LINE[king][from]` only for pinned, pawns use bulk shifts `WHITE` const-folded, `ArrayVec` sink direct `push_unchecked` + `copy_nonoverlapping` for `into_arrayvec`, `pinned==0` fast path (no second loops), `cached checkers` for `legal_moves()`.

#### Scenario: Bulk perft leaf
- **WHEN** `perft(board,1)` via `MoveCounter` is called
- **THEN** it uses `popcount` never `pop_lsb`, node count equals materialising path, `cargo bench --bench micro` `movegen_one_shot` wins vs ultrachess 42ns (x86 86 vs 93 0.92×)

#### Scenario: ArrayVec direct
- **WHEN** `board.legal_moves()` is called
- **THEN** it generates directly into `ArrayVec` via `MoveSink for ArrayVec` with 0 heap and no `MoveList→ArrayVec` copy (40B bulk)
