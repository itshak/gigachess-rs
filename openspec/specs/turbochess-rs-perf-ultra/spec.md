# turbochess-rs-perf-ultra Specification

## Purpose
Achieve ultra performance parity vs ultrachess 8/8 micro rows + perft on M1 Max `LTO=fat` standard-only, via 8 micro-opts while keeping 144B `Copy` and `is_chess960` gate.

## Requirements

### Requirement: Ultra Parity SHALL Beat ultrachess 8/8 Micro + perft on M1 Max LTO=fat Standard-Only

The system SHALL beat `ultrachess` (MIT `yahorbarkouski/ultrachess`, `M1 Max criterion 10 LTO=fat` `fen_write 103, fen_parse 208, clone 3.67, isCheck 0.43, hash 0.34, SAN 1.47µs/48, movegen 42ns, make+unmake 736ns, perft ~400 Mnps`) on all 8 micro rows + perft d5 on `M1 Max` standard positions (`is_chess960==false`); 960 `is_chess960==true` is exception slower OK. `profile.release`/`bench` SHALL be `lto=fat codegen-units=1 panic=abort` as measured profile.

#### Scenario: Micro 8 rows win on M1 Max
- **WHEN** `cargo bench --bench micro -- --sample-size 10 --measurement-time 1` is run on M1 Max with `LTO=fat` on startpos standard
- **THEN** turbo median < ultrachess target for `fen_write`, `fen_parse`, `clone`, `movegen_one_shot`, `make_unmake_48`, `is_check/in`, `is_check/out`, `hash`, `san_48` (each `Throughput::Elements`)

#### Scenario: Perft win on M1 Max
- **WHEN** `cargo bench --bench perft_bench -- --sample-size 10` is run on M1 Max
- **THEN** `perft/startpos_d5` `Mnps` > ultrachess ~400 (x86 ~193 vs 210 shows 1.05× win, M1 ~430 vs 400)

#### Scenario: Chess960 exception
- **WHEN** `is_chess960==true` (king not e1/e8 or rook not a/h)
- **THEN** micro may be slower than ultrachess (documented exception) but `cargo test` 100 Chess960 FEN round-trip byte-equal + perft parity still pass
