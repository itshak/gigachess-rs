# turbochess-rs-chess960 Specification

## Purpose
Defines Chess960 (Fischer Random) support: board representation with rook-square castling rights, path-based castling legality, FEN dialect handling, moves2 castling encoding, and castling hashing.

## Requirements

### Requirement: Castling Rights SHALL Be Represented as Rook Squares

The system SHALL represent castling rights as the squares of the castling rooks (per color, kingside/queenside), supporting arbitrary back-rank rook placements. Standard-chess rights SHALL coincide with files a/h.

#### Scenario: Standard and 960 rights round-trip
- **WHEN** a standard position (`KQkq`) and a Chess960 position with non-standard rook files are parsed from FEN
- **THEN** the castling rights round-trip through FEN output exactly (standard KQkq and Shredder file letters respectively)

### Requirement: Castling Legality SHALL Use Path-Based Rules

The system SHALL determine castling legality by: rights present, all squares between king and rook empty (rook excluded from its own path check), all squares between king and its destination plus the destination safe from enemy attack, and the rook not pinned. Final squares SHALL be king on the g-file (kingside) / c-file (queenside) and rook on the f-file / d-file respectively, for both standard and Chess960.

#### Scenario: 960 castling generation
- **WHEN** legal moves are generated for a Chess960 position with castling rights
- **THEN** castling moves are generated as king-from → rook-square whenever the path and safety rules hold, and never when the king is in check or passes through an attacked square

### Requirement: FEN Parsing SHALL Support Standard and Shredder Dialects

The system SHALL parse standard FEN castling notation (KQkq) and Shredder FEN notation (file letters), automatically falling back to the Shredder dialect when standard parsing fails, and SHALL emit standard notation for standard positions and file-letter notation for Chess960 positions.

#### Scenario: Dialect auto-fallback
- **WHEN** a FEN with Shredder castling letters (e.g. `bqkb1ppr/... w HAha`) is parsed
- **THEN** parsing succeeds and yields the same castling rights as the equivalent Shredder-encoded position

### Requirement: moves2 Castling Moves SHALL Encode King-from to Rook-square

The system SHALL encode castling moves in the moves2 format as king-from square → rook square (for both standard and Chess960), and SHALL decode them by detecting that the destination square holds the mover's own rook. No file-count heuristics SHALL be used.

#### Scenario: Castling word round-trip
- **WHEN** a castling move (standard `e1h1`/`e1a1`, Chess960 e.g. `f1h1`) is encoded into moves2 and decoded against the resulting position sequence
- **THEN** the decoded move is the same castling move and replay matches

### Requirement: Castling Zobrist Keys SHALL Be Per Rook File

The system SHALL hash castling rights with 16 keys indexed by (color, rook file), where the file-a/h keys SHALL equal the Polyglot castling keys (`768..771`) so that standard-chess positions hash identically to the Polyglot specification, and the remaining keys SHALL be compile-time constants derived from a documented deterministic PRNG.

#### Scenario: Standard-chess Polyglot parity
- **WHEN** any standard-chess position is hashed
- **THEN** the hash equals the Polyglot specification hash (verified against python-chess and the canonical startpos/Kiwipete vectors)

#### Scenario: 960 rights distinguishability
- **WHEN** two Chess960 positions differ only in which rook file holds a kingside right
- **THEN** their hashes differ
