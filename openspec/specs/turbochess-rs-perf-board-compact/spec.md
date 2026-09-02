# turbochess-rs-perf-board-compact Specification

## Purpose

Explores a compact `Board` layout probe behind a `compact` feature to reduce the 368B `Copy` board (mailbox + 4 rook squares + 64B mask) toward ultrachess ~100B, measuring clone and movegen gains while keeping Chess960 compat. (Close-gap outcome: landed as the **default** 144B layout, no feature gate needed.)

## Requirements

### Requirement: Compact Board Probe SHALL Be Feature-Gated

The system SHALL provide a `compact` Cargo feature that, when enabled, replaces `Board.mailbox: [u8;64]` with a nibble-packed `[u8;32]` and/or compresses `castle_mask: [u8;64]` to `[u8;4]` derived, without breaking `Board:Copy` (`Copy` retained in both layouts).

#### Scenario: Compact vs default parity
- **WHEN** `cargo test --features compact` and `cargo test` (default) are run on the perft suite (6 positions)
- **THEN** both pass and `cargo bench --bench micro --features compact` reports `clone` and `movegen_one_shot` medians with `±%` vs default, documented in `BENCH.md` compact row

### Requirement: Compact Layout SHALL Preserve Chess960 and Hash Parity

The system SHALL keep Chess960 castling (rook squares), X-FEN, and Polyglot zobrist parity in compact layout, with identical `zobrist_full()` and `to_fen()` outputs for all reference FENs.

#### Scenario: Chess960 FEN round-trip
- **WHEN** 100 Chess960 positions are parsed and rendered via `compact` and default builds
- **THEN** FEN strings are byte-equal and `zobrist` keys match
