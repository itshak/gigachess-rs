# Rust competitive benchmark matrix

Baseline: GigaChess 0.1.11, main 066db0e. Benchmark crate is private and
standalone, not part of the shipped library. Dependencies are locked; GPL
Shakmaty is a benchmark/test oracle only. No competitor source is copied.
UltraChess native core and Ferrum are pinned to exact git revisions.
Ferrum Cargo metadata says MIT while its README says Apache-2.0 and the
checkout contains no license file; do not reuse source until clarified.

## Reproduction

    cargo test --locked --release
    cargo run --release --locked --manifest-path benchmarks/competitive/Cargo.toml > results.tsv
    python3 benchmarks/competitive/summarize.py results.tsv

Use RUSTFLAGS="-C target-cpu=native" for native, and add --features pext
only on x86 with BMI2. All implementations share release LTO and flags.
CI runs portable Linux x64, native Linux x64/ARM64, x64 PEXT, Windows
and macOS; raw measurements and metadata are artifacts.

Six standard chess perft positions gate every fixed-position comparison:
bulk d4 matches reference totals and full d3 must match every implementation.
A deterministic generated game corpus validates normalized legal UCI sets
and exercises count, materialization, sorted UCI, trusted and checked replay.
It is synthetic, not a real-game corpus. Five timed samples after warmup;
all library-native results are consumed by black_box. Results are ns per
call; divide by units_per_call for per-position/per-move costs. Ratios use
matched medians, not pooled timings. Runner variability and fixed ordering
limit confidence in small differences; repeat counterbalanced runs before
claiming a narrow win. Native output types and maintained state differ
(e.g. history, hash semantics, clocks, validation); native FEN and SAN
rendering do not promise identical allocation behavior. No cross-library
hash-value comparison except where semantics match.

## Transfer audit: JS 0.5 to 0.6

Rust already has packed u16 moves, bulk pawns and leaf popcount, const
generic color dispatch, direct SAN disambiguation, slim perft make/unmake,
trusted packed replay and native 64-bit magic/PEXT. JS split-32 tables are
not a Rust optimization. Rust startpos reparses FEN; private immutable
initial position template is directly transferable. Next experiments:
standard-only const specialization and no-castling/EP fast paths; king
target attack checks vs whole danger map; precomputed compile-time slider
tables or hoisted table references; compact role/color bitboards vs mailbox
and move metadata; copy-make vs undo; depth-2 fused leaf traversal; SAN
suffix early-out and direct legality. Keep Chess960, null, FIDE/Polyglot,
moves2 and shared contract fixtures intact. Benchmark before changing.
Ferrum offers copy-make, mailbox and compile-time tables; Cozy offers
piece-group iteration/PEXT; UltraChess offers tagged moves, mailbox and
specialized perft. Shakmaty and Jordan Bray are correctness/performance
references, not a source for MIT production code.

## Outstanding scope

Real public-game replay corpus comparable to JS 300-game/20k-position
matrix; Chess960 head-to-head where supported, dedicated hash/repetition
lanes, memory/allocation and cold-start metrics, confidence intervals,
counterbalanced sampling, and baseline-versus-candidate runs for each
optimization. Do not label this initial matrix as universal leadership.
