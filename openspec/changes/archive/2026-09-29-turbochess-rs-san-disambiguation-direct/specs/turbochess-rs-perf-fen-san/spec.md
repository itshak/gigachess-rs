## MODIFIED Requirements

### Requirement: SAN Write SHALL Reuse Tables Toward 1.43µs

The system SHALL implement `move_to_san` via `attackers_bb = same_type & !from & attacks_from_target(to)` pre-filter that then asks each surviving candidate directly — one `make_move_unchecked` on a stack copy, one king-safety query for the mover's king against the side to move next, one `unmake_move` — rather than generating every legal move in the position and filtering the list (was per-candidate `is_pseudo_legal+make`, then a single `generate_moves_into`), gating `has_no_legal_moves` behind `in_check()` `count_legal_moves()==0` `make/unmake` suffix not `clone`, `piece_code_at` 6-scan.

The candidate query SHALL return the same set the movegen filter returned: the pre-filter is exact pseudo-legality for the roles that reach the branch, a candidate is built with `mv.promotion()` so promotion equality holds, and the query stops after eight candidates. Both values the king-safety test needs — the king's colour and the attacker colour — SHALL be read from the caller's board, before the candidate is made.

The suffix clause SHALL be a callable seam rather than a private block: `check_mate_suffix(after: &Board) -> Option<char>` SHALL hold the O(1) `in_check()` gate and the `count_legal_moves()` mate test, so a caller that has already made the move appends the suffix without a board copy, a `make_move_unchecked` and an `unmake_move`; `move_to_san` SHALL remain the composition of `move_to_san_body` and that same rule applied to a position it reaches by making the move on a copy.

#### Scenario: SAN micro toward parity
- **WHEN** `cargo bench --bench micro` `SAN 48` is run
- **THEN** turbo median < ultrachess 1.47µs/48 on M1 Max (x86 3548 vs 6687 0.53×) and `cargo test` `san` byte-equal to `shakmaty::SanPlus`

#### Scenario: The suffix seam costs a fast caller nothing
- **WHEN** a replay or export walk makes a move with `make_move_unchecked` and calls `check_mate_suffix` on the result
- **THEN** the suffix matches what `move_to_san` derives internally, with no additional `make`/`unmake` and no `Board` copy, as asserted across 100,000 positions by `tests/san_split_property.rs`

#### Scenario: The direct candidate query beats the movegen it replaces
- **WHEN** the SAN body is rendered for every move of a 13,908,447-ply slice of a real game database, once through `move_to_san_body` and once through a per-candidate query
- **THEN** the two forms differ on no move, and the direct form costs no more per SAN body than 8.1 ns against the movegen form's 13.5 ns
