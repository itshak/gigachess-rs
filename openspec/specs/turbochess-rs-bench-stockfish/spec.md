# turbochess-rs-bench-stockfish Specification

## Purpose

Provides a real `Stockfish` (C++) perft bench compiled on this M1 Max (`apple-silicon`) to compare `turbochess-rs` Mnps vs Stockfish on same hardware, replacing hypothetical `250-350 Mnps` with measured `go perft` numbers for `README`/`BENCH.md`.

## Requirements

### Requirement: Stockfish SHALL Be Compiled and Benched on M1 Max

The system SHALL provide a `just bench-stockfish` (or `benches/vs_stockfish`) that does `make -C /tmp/stockfish_src/src build ARCH=apple-silicon` (produces `95 MB` `stockfish` binary) and runs `echo "position startpos\ngo perft 6\nquit" | stockfish` timed via `/usr/bin/time -p`, reporting `Nodes searched` and `Mnps` alongside `cargo bench --bench perft_bench` turbo `387 Mnps` and `vs_libraries` results.

#### Scenario: Stockfish real bench runs
- **WHEN** `just bench-stockfish` is run on M1 Max
- **THEN** it prints `Stockfish startpos d5 4.86M` `Mnps` and `d6 119M` `Mnps` (measured `~25 Mnps d5`, `~170 Mnps d6` on this host) and `BENCH.md`/`README` stretch row shows `Stockfish GPL-3 146-197 Mnps (real, M1 Max apple-silicon)` vs turbo `387 Mnps`

### Requirement: Stockfish Bench SHALL Be GPL-Compliant and Docs-Only

The system SHALL keep `Stockfish` (`GPL-3`) as an external binary in `/tmp/stockfish_src` (not vendored), docs-only bench; `turbochess-rs` stays `100% MIT`, no `GPL` code is linked or copied.

#### Scenario: No GPL linkage
- **WHEN** `cargo test` and `cargo bench` are run
- **THEN** no `Stockfish` source is compiled as a Rust dependency; `stockfish` is only invoked as a subprocess for `BENCH.md` numbers
