## MODIFIED Requirements

### Requirement: Micro Harness SHALL Cover 8 Rows

The system SHALL provide `benches/micro.rs` `criterion` group for `FEN write`, `FEN parse`, `movegen one-shot`, `make+unmake 48-ply`, `isCheck in/out`, `hash`, `SAN 48`, `clone` on `startpos`/`kiwipete`/`960-284` with `ns/op` and `Throughput::Elements`. The system SHALL also add `san_visitor` and `perft_visitor` rows and a `compact` feature comparison (`clone`/`movegen` with `compact`).

#### Scenario: Micro baseline is gated
- **WHEN** `just bench` (`cargo test` gate) then `cargo bench --bench micro` is run
- **THEN** it emits `ns/op` per row, writes `benches/results/micro-baseline.json`, and refuses to publish if any `perft` reference mismatches

### Requirement: Baseline SHALL Be Frozen Before Engine Patches

The system SHALL freeze `benches/results/turbochess-rs-baseline.json` + `BENCH.md` table after harness lands and before `bulk`/`cached`/`fen-san` patches, and every later engine PR diffs median `>3%`.

#### Scenario: One-patch gating
- **WHEN** a `bulk` patch PR runs `cargo bench --bench micro`
- **THEN** CI reports `±%` vs frozen baseline median and requires `>3%` win and `cargo test` pass to merge

### Requirement: Parity Target SHALL Be ≥ ultrachess in Most Rows

The system SHALL target `≥ ultrachess/BENCH.md` median on `M1/M4 Max` for most — preferably all — of `FEN write 88ns`, `SAN 1.43µs/48`, `hash 0.34ns`, `isCheck 0.32ns` and perft `836 Mnps`; any loss is documented with gap + follow-up issue and technique (like `ultrachess` 4 losses). After close-gap, `SAN`/`perft_visitor`/`isCheck` SHALL each be `≤10%` of ultrachess on M-series.

#### Scenario: Parity proof
- **WHEN** `vs_libraries` + `micro` after final patch vs ultrachess table is compared
- **THEN** README contains head-to-head medians and notes `5 wins 1 tie 4 losses` or better, with `+16B/Undo` vs `8× isCheck` tradeoff recorded, plus visitor/compact deltas

### Requirement: Gap Report SHALL Be Published

The system SHALL publish a gap report in `BENCH.md` with turbo vs ultrachess deltas for all 14 axes, marking deliberate vs fixable gaps (visitor, colour-template, compact) and C++ stretch targets (Gigantua CPOL, Stockfish GPL-3) with licensing notes.

#### Scenario: Gap report exists
- **WHEN** `BENCH.md` is viewed
- **THEN** it contains the gap table and a `### C++ 2.1 Gnps Study (MIT-safe)` section stating no GPL/CPOL code is copied
