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
Two deterministic corpora (32 seeded games, 2,048 plies; 300 real Lichess games,
18,954 plies) validate exact normalized legal UCI sets and exercise count,
materialization, sorted UCI, trusted and checked replay. The Giga/Shakmaty
Polyglot and FIDE repetition keys are checked at every corpus position;
two Chess960 cases check exact move lists and depth-3 perft across Giga,
Shakmaty and Cozy. Five timed samples after warmup;
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
not a Rust optimization. Rust previously reparsed startpos FEN; the immutable compile-time template
is now implemented and tested against the full FEN-derived state. Next experiments:
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

Broader Chess960 corpus; memory/allocation and cold-start metrics, confidence intervals,
counterbalanced sampling, and baseline-versus-candidate runs for each
optimization. Do not label this initial matrix as universal leadership.

## Corpus provenance

The 300 nonempty standard mainlines are extracted from the January 2013
Lichess open database (https://database.lichess.org/standard/lichess_db_standard_rated_2013-01.pgn.zst,
CC0). No player metadata is retained. Source SHA256:
aa40b3671fa3cf1072eb182892cd90b0e1e003a4a5943492f64b77e7f3fd1635.
Reproduce with python-chess==1.999: decompress with zstd -dc and pipe into
make_corpus.py. Fixture SHA256:
8c286bb3ce9432389232d2ac8669e69a5a189b42f23a453f4194fd98b4548e40.

## Evidence (2026-11-26)

Baseline six-runner GitHub Actions run 45763305461: all jobs passed.
Raw results, environment and parity logs are in results/baseline-ci.
Local Xeon E5-1620 v2, native flags, Rust 1.97.1: smoke.tsv is the initial
baseline; real.tsv and expanded.tsv include the startpos template. These
are short exploratory runs, not substitutes for hosted CI. Paired
startpos template vs legacy FEN parsing in real.tsv: 4.865 ns vs 421.789 ns
(86.7x); full release tests passed. expanded.tsv additionally validates
Chess960 perft and Polyglot/repetition keys. No universal fastest claim:
full-tree perft and trusted replay remain important weaknesses.
