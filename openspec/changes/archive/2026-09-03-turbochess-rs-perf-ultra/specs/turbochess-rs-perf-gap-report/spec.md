## MODIFIED Requirements

### Requirement: Parity-Gap Report SHALL Quantify Each Axis vs ultrachess

The system SHALL publish `BENCH.md` gap report 14 axes (8 micro + 6 perft) turbo median vs ultrachess target (M1 Max `criterion 10 LTO=fat` 103/208/3.67/0.43/0.34/1.47/42/736/~400, M4 836) delta % + technique + follow-up, now **8/8 win** (was 5 wins / 4 deliberate), and `C++ 2.1 Gnps` study MIT-compliant.

#### Scenario: Gap report completeness
- **WHEN** `BENCH.md` is viewed after this change
- **THEN** it contains table 14 axes all green (< ultrachess) and notes `8/8 win` + `perft 214 vs 179` + visitor/compact/template steps

### Requirement: C++ 2.1 Gnps Study SHALL Be MIT-Compliant

The system SHALL document Gigantua `CPOL` / Stockfish `GPL-3` / `Chess_Movegen` MIT-safe with `Lookup_Pext` etc, **no copy**, only MIT `ultrachess`/`cozy-chess` reused, plus real Stockfish `apple-silicon` bench alongside.

#### Scenario: Licensing note
- **WHEN** `BENCH.md` stretch-target mentions Gigantua/Stockfish
- **THEN** it labels `CPOL (not MIT, study only)` and `GPL-3 (study only)` with `CodeProject 5313417` `Chess_Base.hpp:Lookup_Pext`
