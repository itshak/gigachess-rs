# Perft workspace integrity

## Requirement: isolated context ownership
A movegen analysis SHALL NOT mutate or overwrite castling plans of another
live analysis context, even during nested Chess960 calculations.

## Requirement: exact position preservation
Optimized recursive perft SHALL retain legal move sets, piece placement,
turn, castling, EP, FEN, clocks, Zobrist/Polyglot and moves2 data.
Public move execution SHALL maintain hashes even when perft skips them.

## Requirement: fair reproducibility
Each benchmark SHALL compare reference-matching node counts before timings.
When testing JavaScript hash skipping, BOTH libraries SHALL load Zobrist
tables or BOTH leave them unloaded. Rust release benchmark SHALL use
the same board and the same release compiler optimization level.
Neither repository SHALL run GitHub CI to produce these measurements.
