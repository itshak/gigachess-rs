## Purpose

Amends the core-engine capability: pins the en-passant hash condition, switches castling hashing to rook-file keys, redefines the moves2 castling encoding, and adds engine-facing API surface.

## MODIFIED Requirements

### Requirement: High-Throughput Batch Replay Engine

The system SHALL provide `replay_moves2_batch` replaying slices of 16-bit encoded games in parallel, achieving ≥500,000 games/second across CPU cores. The per-ply hash SHALL be maintained incrementally (O(1)) using Polyglot keys with the Pseudo en-passant condition (ep key included iff a pawn of the side to move geometrically attacks the ep square), and castling keys SHALL be per (color, rook file) per `turbochess-rs-chess960`. Castling moves SHALL be decoded as king-from → own-rook-square (per `turbochess-rs-chess960`); no legal-movegen scan SHALL be required to decode a word.

#### Scenario: Batch replay matches FEN positions
- **WHEN** 100,000 games are replayed from binary `moves2` slices
- **THEN** final board hashes and legal statuses are verified with 100% parity

#### Scenario: Pinned en-passant hash condition
- **WHEN** a position has an en-passant capturer that is pinned (e.g. `8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1`)
- **THEN** the ep key SHALL still be included (Pseudo condition), yielding `0x83bf25e378cb17d0` as verified against python-chess

## ADDED Requirements

### Requirement: Board SHALL Be Copy and Support Pseudo-Legal Generation

The system SHALL make `Board` a `Copy` type (bit-for-bit copy semantics for engine hot paths) and SHALL expose `pseudo_legal_moves()` generating moves without king-safety filtering, for engines that apply their own legality handling.

#### Scenario: Copy semantics
- **WHEN** a `Board` is copied before making a move
- **THEN** the copy is a bit-for-bit snapshot and the original is unchanged after the move is applied to the copy
