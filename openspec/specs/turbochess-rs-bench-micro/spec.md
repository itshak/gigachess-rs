# turbochess-rs-bench-micro Specification

## Purpose
Real single-call micro harness before any engine win, with frozen baseline like `ultrachess/BENCH.md` and parity gate toward ultrachess `88ns FEN write / 1.43µs SAN / 0.32ns isCheck / 0.34ns hash`.

## Requirements

### Requirement: Micro Harness SHALL Cover 8 Rows

The system SHALL provide `benches/micro.rs` `criterion` group for `FEN write`, `FEN parse`, `movegen one-shot`, `make+unmake 48-ply`, `isCheck in/out`, `hash`, `SAN 48`, `clone` on `startpos`/`kiwipete`/`960-284` with `ns/op` and `Throughput::Elements`.

#### Scenario: Micro baseline is gated
- **WHEN** `just bench` (`cargo test` gate) then `cargo bench --bench micro` is run
- **THEN** it emits `ns/op` per row, writes `benches/results/micro-baseline.json`, and refuses to publish if any `perft` reference mismatches

### Requirement: Baseline SHALL Be Frozen Before Engine Patches

The system SHALL freeze `benches/results/turbochess-rs-baseline.json` + `BENCH.md` table after harness lands and before `bulk`/`cached`/`fen-san` patches, and every later engine PR diffs median `>3%`.

#### Scenario: One-patch gating
- **WHEN** a `bulk` patch PR runs `cargo bench --bench micro`
- **THEN** CI reports `±%` vs frozen baseline median and requires `>3%` win and `cargo test` pass to merge

### Requirement: Parity Target SHALL Be ≥ ultrachess in Most Rows

The system SHALL target `≥ ultrachess/BENCH.md` median on `M1/M4 Max` for most — preferably all — of `FEN write 88ns`, `SAN 1.43µs/48`, `hash 0.34ns`, `isCheck 0.32ns` and perft `836 Mnps`; any loss is documented with gap + follow-up issue and technique (like `ultrachess` 4 losses).

#### Scenario: Parity proof
- **WHEN** `vs_libraries` + `micro` after final patch vs ultrachess table is compared
- **THEN** README contains head-to-head medians and notes `5 wins 1 tie 4 losses` or better, with `+16B/Undo` vs `8× isCheck` tradeoff recorded
