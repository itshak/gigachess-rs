## Purpose

Defines zero-allocation, compiler-optimized, and lookup-accelerated performance standards across move generation, SAN parsing, board cache queries, and Chess960 castling paths.

## ADDED Requirements

### Requirement: Zero-allocation SAN Parsing
The engine SHALL parse standard Algebraic Chess Notation (SAN) tokens into legal move representations without allocating heap memory, and SHALL locate candidate moving pieces via reverse attacker queries against the target square rather than generating all legal moves across the board.

#### Scenario: Parse standard pawn move without heap allocation
- **WHEN** `san_to_move` is called with a simple move like `"e4"` or `"d5"`
- **THEN** the token is parsed into a move using stack storage without heap allocations in under 500 nanoseconds

#### Scenario: Disambiguate piece move using target attacker query
- **WHEN** `san_to_move` is called with an ambiguous piece move like `"Nbd7"`
- **THEN** only knights attacking the square `d7` are inspected and the correct move is returned with byte-for-byte correctness

### Requirement: Loop-Invariant Movegen Optimization
The legal move generator SHALL hoist square coordinates and bit shifts outside per-target bitboard iteration loops and SHALL construct quiet and capture moves directly without optional promotion pattern matching.

#### Scenario: Generate startpos moves under 42 nanoseconds
- **WHEN** `board.legal_moves()` is invoked on the standard starting position
- **THEN** all 20 legal moves are produced in an `ArrayVec<Move, 256>` in under 42 nanoseconds on Apple Silicon

### Requirement: Direct-Register Board Cache Access
The position query methods `Board::in_check()`, `Board::zobrist()`, and `Board::checkers_bb()` SHALL load cached state fields directly from the board struct without pointer casting or memory barrier indirection.

#### Scenario: in_check compiles to direct register test
- **WHEN** `board.in_check()` is invoked
- **THEN** the call evaluates in a single CPU cycle (under 0.45 nanoseconds) by testing `self.checkers != 0`

#### Scenario: zobrist compiles to single register load
- **WHEN** `board.zobrist()` is invoked
- **THEN** the call returns the incrementally maintained 64-bit hash in a single load instruction

### Requirement: Precomputed Chess960 Castling Paths
Path clearance and transit check safety for king and rook castling moves in standard and Chess960 positions SHALL be validated against static lookup bitmasks rather than dynamic per-file iteration loops.

#### Scenario: Fast castling validation on Chess960 position
- **WHEN** legal moves are generated on an arbitrary Chess960 position with castling rights
- **THEN** square clearance between king and rook is evaluated via bitwise AND with precomputed bitmasks in under 70 nanoseconds
