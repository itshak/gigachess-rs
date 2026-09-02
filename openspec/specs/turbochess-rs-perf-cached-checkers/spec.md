# turbochess-rs-perf-cached-checkers Specification

## Purpose
Branch-free `in_check()` via cached `checkers` in `Undo` + perft slim path — `0.32ns`/`0.34ns` ultrachess parity.

## Requirements

### Requirement: Undo SHALL Cache prev_checkers + Perft Slim

The system SHALL extend `Undo` with `prev_checkers:Bitboard` + `prev_zobrist:u64` and maintain `Board.checkers:Bitboard` + `history_hashes` in `make`/`unmake`; SHALL add `make_move_perft/unmake_move_perft` slim (skip `zobrist`/`history_hashes`/`halfmove`, `position.rs:389` safe only for perft).

#### Scenario: in_check O(1) toward 0.32ns
- **WHEN** `board.in_check()` is called after `make_move`
- **THEN** it returns `self.checkers != 0` without scan and `cargo bench --bench micro` `isCheck in/out` drops `>10%` toward `0.32ns`/`0.33ns`

### Requirement: Unmake SHALL Restore Without Recompute

The system SHALL restore `checkers` + `zobrist` from `Undo` on `unmake` without `attackers` recompute; perft uses slim path.

#### Scenario: Make+unmake cycle vs ultrachess tradeoff
- **WHEN** `make/unmake 48-ply` micro bench is run
- **THEN** `ns/op` not regressed beyond known `503ns vs cozy 353ns` gap which is kept (pays `+2ns/make` for `8× isCheck` `BENCH.md: Deliberate`) and `perft position 3` depth `7` still correct
