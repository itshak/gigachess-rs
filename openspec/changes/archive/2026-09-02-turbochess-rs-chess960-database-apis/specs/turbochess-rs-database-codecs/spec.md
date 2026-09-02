## Purpose

Defines the database-oriented batch codecs and hash-replay streams that
high-throughput chess databases (blind-base and similar) use on top of the
core engine: movetext ↔ moves2 conversion, incremental hash replay, and
position-statistics building.

## ADDED Requirements

### Requirement: Movetext SHALL Parse to moves2 Without Heap Strings in the Loop

The system SHALL provide `parse_movetext_to_moves2(start_fen, movetext) -> Vec<u8>` that tokenizes movetext at the byte level (handling move numbers, comments `{...}`, NAGs `$n`, variations `(...)` by skipping, and result tokens), resolves each SAN token against the current position, and emits little-endian moves2 words. Invalid SAN SHALL be reported as an error unless the token is ignorable per the tolerance rules.

#### Scenario: Import parity with stored blobs
- **WHEN** movetext from blind-base's database is parsed
- **THEN** the produced moves2 bytes are byte-identical to the stored blobs (modulo the castling re-encoding defined in `turbochess-rs-chess960`)

### Requirement: moves2 SHALL Render to SAN Movetext

The system SHALL provide `moves2_to_san_movetext(start_fen, moves2, result) -> String` decoding each word in O(1) (no legal-movegen scan), rendering conventional SAN with move numbers and check/mate suffixes, matching shakmaty's `SanPlus` rendering byte-for-byte.

#### Scenario: Rendering parity
- **WHEN** a moves2 blob is rendered and re-parsed
- **THEN** the round-trip reproduces the same moves2 blob, and the rendered SAN matches shakmaty's rendering for the same game

### Requirement: Hash Replay SHALL Use Incremental Hashes

The system SHALL provide a replay stream yielding (hash, ply) pairs for a moves2 blob (and a movetext variant), where each hash SHALL be maintained incrementally (O(1) per ply, Polyglot keys, Pseudo ep condition) rather than recomputed.

#### Scenario: Incremental parity
- **WHEN** replayed hashes are compared against from-scratch recomputation at every ply
- **THEN** they agree with 100% parity, including positions with pinned en-passant capturers (Pseudo condition includes the ep key)

### Requirement: Position Statistics SHALL Be Buildable from Game Batches

The system SHALL provide a position-statistics builder that consumes many games (moves2 slices) and aggregates per-position move counts and game samples keyed by position hash, parallelized across CPU cores.

#### Scenario: Batch statistics parity
- **WHEN** a batch of games is processed
- **THEN** per-position move counts match a sequential reference implementation and the final hashes match replay parity
