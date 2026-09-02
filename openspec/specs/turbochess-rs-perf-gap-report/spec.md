# turbochess-rs-perf-gap-report Specification

## Purpose

Documents the parity gap versus ultrachess and the C++ 2.1 Gnps engines (Gigantua, Stockfish) with MIT compliance, technique inventory, and a staged close-gap plan.

## Requirements

### Requirement: Parity-Gap Report SHALL Quantify Each Axis vs ultrachess

The system SHALL publish in `BENCH.md` a gap report table with for each of the 8 micro rows + 6 perft positions: turbo median, ultrachess target, delta %, technique causing the gap (deliberate vs fixable), and follow-up issue. Deliberate losses (FEN parse, movegen, make+unmake, clone) SHALL be marked with `BENCH.md: Deliberate` and the compact/colour-template trade-off.

#### Scenario: Gap report completeness
- **WHEN** `BENCH.md` is viewed after this change
- **THEN** it contains a table with all 14 axes, deltas vs ultrachess, and notes `5 wins / 4 deliberate losses` (or current) plus visitor/compact/template next steps

### Requirement: C++ 2.1 Gnps Study SHALL Be MIT-Compliant

The system SHALL document Gigantua (`github.com/Gigantua/Gigantua`, **no LICENSE, CodeProject CPOL — not MIT**), Stockfish (`GPL-3`), and `Chess_Movegen` comparison (MIT-safe) with how Gigantua achieves `Perft aggregate 18.9B 9967ms 1906 Mnps`: visitor pattern (2× vs movelist), colour/EP/castling template expansion, no hashing/make+unmake, `Lookup_Pext` (`Chess_Base.hpp`) vs `Lookup_Fancy`, Agner Fog reciprocal throughput 0.25, and SHALL state **no GPL/CPOL code is copied** — only MIT techniques (ultrachess, cozy-chess) are reused clean-room.

#### Scenario: Licensing note
- **WHEN** `BENCH.md` stretch-target section mentions Gigantua/Stockfish
- **THEN** it labels Gigantua as `CPOL (not MIT, study only)` and Stockfish as `GPL-3 (study only)` and cites the CodeProject article and `Gigantua/Chess_Base.hpp: Lookup_Pext` without copying code
