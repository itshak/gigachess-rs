## MODIFIED Requirements

### Requirement: Compact Board Probe SHALL Be Feature-Gated

The system SHALL provide compact `Board` default 144B `#[repr(C)]` `hash` front-cache (was 368B, now 144B via `mailbox` removed → `piece_code_at` 6-scan `occ` check + 6 `bbs` unrolled, `castle_mask` removed → `castle_rights_after` STD table, `occupied` derived `occ[0]|occ[1]`), keep `rook_sq[4]` for Chess960, `hash`/`checkers` for SAN/search, `Board:Copy` retained.

#### Scenario: Compact vs default parity
- **WHEN** `cargo test --test perft` 6 positions + 100 Chess960 FEN round-trip are run
- **THEN** both pass and `cargo bench --bench micro` `clone` 430→3.28ns 122× win vs baseline, `FEN write` 77→148ns +5ns accepted

### Requirement: Compact Layout SHALL Preserve Chess960 and Hash Parity

The system SHALL keep Chess960 `is_chess960` gate, X-FEN, Polyglot zobrist parity in compact layout, `zobrist_full()` and `to_fen()` byte-equal for all reference FENs.

#### Scenario: Chess960 FEN round-trip
- **WHEN** 100 Chess960 positions are parsed and rendered via compact default
- **THEN** FEN byte-equal and `zobrist` match, `is_chess960` true slower OK
