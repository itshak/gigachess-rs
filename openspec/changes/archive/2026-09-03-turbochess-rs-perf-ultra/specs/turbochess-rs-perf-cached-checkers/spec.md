## MODIFIED Requirements

### Requirement: Undo SHALL Cache prev_checkers + Perft Slim

The system SHALL extend `Undo` with `prev_checkers:Bitboard` + `prev_zobrist:u64` and maintain `Board.checkers` + `history` in `make`/`unmake` (`checkers !=0` is `in_check` 0.32ns); `make_move_perft`/`unmake_move_perft` slim now also maintains `checkers` (`attackers_to` after turn flip) so `generate_moves_templated` can use cached `self.checkers` (saves 5 attacks ~20ns per movegen) for both `legal_moves()` and perft.

#### Scenario: in_check O(1) toward 0.32ns
- **WHEN** `board.in_check()` after `make_move` is called
- **THEN** it returns `self.checkers !=0` via `unsafe` load, `cargo bench --bench micro` `isCheck` 0.48→0.43 toward 0.32/0.33

#### Scenario: Make+unmake cycle vs ultrachess
- **WHEN** `make/unmake 48-ply` micro bench is run
- **THEN** turbo median < ultrachess 736ns on M1 Max (x86 1903 vs 2073 0.92×) via `piece_code_at_color` 6-scan + `pawn_code_at` + `CASTLE_CLEAR_STD` table, `perft` still wins
