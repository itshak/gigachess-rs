## MODIFIED Requirements

### Requirement: FEN Write SHALL Be Branchless Table Toward 88ns

The system SHALL implement `write_fen` via `const PIECE_CHAR:[u8;12]` `ArrayVec<u8,128>` without `format!`, stamping `mailbox[64]` via 12 `bbs` scans (32 pieces) then `for rank in (0..8).rev() for file in 0..8` 64 loads, `Cargo.toml` `profile` `lto=fat` as measured.

#### Scenario: FEN micro toward parity
- **WHEN** `cargo bench --bench micro` `FEN write startpos` is run
- **THEN** turbo median < ultrachess 103ns on M1 Max (x86 148 vs 198 0.75×) and `Fen` round-trip 1k random games byte-equal

### Requirement: SAN Write SHALL Reuse Tables Toward 1.43µs

The system SHALL implement `move_to_san` via `attackers_bb = same_type & !from & attacks_from_target(to)` pre-filter + single `generate_moves_into` `MoveList` (was per-candidate `is_pseudo_legal+make`), gating `has_no_legal_moves` behind `in_check()` `count_legal_moves()==0` `make/unmake` suffix not `clone`, `piece_code_at` 6-scan.

#### Scenario: SAN micro toward parity
- **WHEN** `cargo bench --bench micro` `SAN 48` is run
- **THEN** turbo median < ultrachess 1.47µs/48 on M1 Max (x86 3548 vs 6687 0.53×) and `cargo test` `san` byte-equal to `shakmaty::SanPlus`

## ADDED Requirements

### Requirement: FEN Parse SHALL Be Bytes Toward 208ns

The system SHALL parse via `bytes` not `chars` (`for &b in placement.as_bytes()`), `piece_from_byte` table, `put_piece_no_hash` (no per-piece `hash ^=`; `set_state` recomputes `zobrist_full` once), fast `KQkq` path `use_fast_standard`.

#### Scenario: FEN parse micro
- **WHEN** `cargo bench --bench micro` `FEN parse startpos` is run
- **THEN** turbo median < ultrachess 208ns on M1 Max (x86 430 vs 452 0.95×)
