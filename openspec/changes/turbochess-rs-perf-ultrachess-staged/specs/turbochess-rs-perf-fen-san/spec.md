## Purpose

Branchless `FEN`/`SAN` micro-opts matching ultrachess `88ns`/`1.43µs` parity, copied MIT.

## ADDED Requirements

### Requirement: FEN Write SHALL Be Branchless Table Toward 88ns

The system SHALL implement `write_fen` via `const PIECE_CHAR:[u8;12]` and `ArrayVec<u8,128>` without `format!` frame, copying `ultrachess src/fen.rs:189`.

#### Scenario: FEN micro toward parity
- **WHEN** `cargo bench --bench micro` `FEN write startpos` is run
- **THEN** `ns/op` drops `>10%` toward `88ns` vs baseline and `Fen` round-trip `1k` random games stays byte-equal

### Requirement: SAN Write SHALL Reuse Tables Toward 1.43µs

The system SHALL implement `move_to_san(&mut Position)` via `tables::between`, gating `has_no_legal_moves` behind O(1) `in_check()`, `make/unmake` suffix not `clone`, disambig pre-filter `attackers_bb`, copying `ultrachess src/san.rs:1`.

#### Scenario: SAN micro toward parity
- **WHEN** `cargo bench --bench micro` `SAN 48` is run
- **THEN** `ns/op` drops toward `1.43µs/48` and `cargo test` `san` still `PASS` byte-equal to `shakmaty::SanPlus`
