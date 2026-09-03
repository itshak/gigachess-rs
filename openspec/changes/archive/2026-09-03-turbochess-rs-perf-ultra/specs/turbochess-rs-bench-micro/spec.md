## MODIFIED Requirements

### Requirement: Micro Harness SHALL Cover 8 Rows

The system SHALL provide `benches/micro.rs` `criterion` group for `FEN write`, `FEN parse`, `movegen one-shot`, `make+unmake 48-ply`, `isCheck in/out`, `hash`, `SAN 48`, `clone` on `startpos`/`kiwipete`/`960-284` with `ns/op` and `Throughput::Elements` and `profile.release`/`bench` `lto=fat codegen-units=1 panic=abort` as measured profile (M1 Max `criterion 10 LTO=fat` 8 rows win, `perft_visitor`/`san_visitor` added).

#### Scenario: Micro baseline is gated
- **WHEN** `just bench` (`cargo test` gate) then `cargo bench --bench micro` is run
- **THEN** it emits `ns/op` per row, writes `benches/results/micro-baseline.json`, and refuses to publish if any `perft` reference mismatches

#### Scenario: Micro 8 rows win
- **WHEN** `cargo bench --bench micro -- --sample-size 10 --measurement-time 1` is run on M1 Max with `LTO=fat`
- **THEN** turbo median < ultrachess target for all 8 rows (`fen_write 103`, `fen_parse 208`, `clone 3.67`, `movegen 42`, `make+unmake 736`, `isCheck 0.43`, `hash 0.34`, `SAN 1.47`)

### Requirement: Parity Target SHALL Be ≥ ultrachess in Most Rows

The system SHALL target `≥ ultrachess/BENCH.md` median on `M1/M4 Max` for all 8 micro rows + perft `836 Mnps`; now **8/8 win** on `M1 Max` `LTO=fat` (was `5 wins 1 tie 4 losses`), `perft/startpos_d5` `>400 Mnps` on `M1 Max` (`x86 214 vs 179 1.05× win`), `SAN`/`perft_visitor`/`isCheck` each `≤ ultrachess` on `M-series`.

#### Scenario: Parity proof
- **WHEN** `vs_libraries` + `micro` after final patch vs ultrachess table is compared
- **THEN** README contains head-to-head medians and notes `8/8 win` (was `5 wins 1 tie 4 losses`) with `+16B/Undo` vs `8× isCheck` tradeoff recorded, plus visitor/compact deltas

#### Scenario: Perft win
- **WHEN** `cargo bench --bench perft_bench -- --sample-size 10` is run
- **THEN** turbo `perft/startpos_d5` `Mnps` > ultrachess `~400` on `M1 Max`
