# Mirrored GigaChess Core Contract — normative for JS/TS and Rust

## ADDED Requirements

### Requirement: The two engines SHALL expose identical chess-state semantics
Both engines use a1=0, h8=63; White=0, Black=1; the same square orientation,
standard and Chess960 rules, castling rights associated with actual rook
squares, null-move representation, promotion codes, and Polyglot key vectors.
Implementation data structures, APIs and runtime initialization mechanisms MAY
differ. Changes to a wire format or key algorithm require an explicit joint
ADR, golden fixtures, and a coordinated release; one engine SHALL NOT change it
alone.

#### Scenario: Shared packed move encoding
- **GIVEN** any ordinary legal move, including promotions and Chess960 castling
- **WHEN** encoded as a 16-bit moves2 word
- **THEN** both libraries produce `from | (to << 6) | (promo << 12)`, with
  promotion codes 0=none, 1=N, 2=B, 3=R, 4=Q, square a1=0 and h8=63.
- **AND** the wire move for castling is king-from to the **initial rook square**
  (`e1h1`/`e1a1` standard, and the corresponding rook square for Chess960),
  regardless of the final king square. UCI standard-castling conversion, if
  desired by an external engine, occurs explicitly at that protocol boundary.
- **AND** reserved null word `0xffff` is distinct from all legal board moves,
  rendering `0000` as UCI and `--` as canonical SAN (`Z0` accepted on input).

#### Scenario: Zobrist and Chess960 castling parity
- **GIVEN** any standard or Chess960 position, or the same position reached by
  a different legal move order
- **WHEN** the Polyglot-compatible position key is requested after valid cache
  initialization/maintenance
- **THEN** both return identical 64-bit keys, including startpos
  `463b96181691fc9c` and after 1.e4 `823c9b50fd114196`.
- **AND** Chess960 castling rights use a key per color and rook file; standard
  a/h rights reuse Polyglot 768..771; files b..g derive from splitmix64 seed
  `00c0ffeedabad00d` in the documented Black-then-White order.
- **AND** Polyglot's en-passant contribution uses geometric/pseudo-legal pawn
  reachability, even if that capture would illegally expose the king.

### Requirement: FIDE repetition equivalence SHALL NOT redefine Polyglot keys

#### Scenario: Phantom or pinned en-passant target
- **GIVEN** identical boards, turns and castling rights where one FEN has an
  en-passant target but the side to move has no **legal** en-passant capture
- **WHEN** comparing FIDE repetition equivalence or obtaining a repetition key
- **THEN** the positions are equivalent and repetition keys match.
- **BUT** Polyglot keys remain unchanged and MAY differ for pseudo-legal,
  king-pinned en-passant positions.

#### Scenario: Actually legal en-passant capture
- **GIVEN** a position with a legal en-passant capture
- **WHEN** the en-passant right is removed while pieces, turn, and castling
  remain identical
- **THEN** the repetition keys differ.

### Requirement: Public state and replay SHALL not corrupt unrelated boards

#### Scenario: Castling ownership
- **WHEN** a consumer makes, unmakes or clones a board, or inspects a second
  board's castling rights
- **THEN** modifications to a different board cannot silently alter its
  castling state or cached hash.

#### Scenario: Trusted versus checked moves2 replay
- **GIVEN** untrusted 16-bit moves2 input
- **WHEN** using the checked-replay API (JS `Board.fromMoves2Checked`, Rust
  `replay_moves2_stream`)
- **THEN** the first illegal move is rejected/reported with its ply, without
  exposing a corrupted board to the caller.
- **AND** a separate explicitly documented trusted fast replay path remains
  available for already validated stored games; the historical JS
  `Board.fromMoves2` signature is maintained for compatibility.
- **AND** Uint8Array little-endian transport supports odd byte *offsets*;
  the checked parser rejects odd byte *lengths*.

### Requirement: Cross-language verification SHALL be reproducible

#### Scenario: Any future rule-engine release
- **WHEN** a change modifies move generation, castle-right clearing, Chess960,
  SAN, FEN, Zobrist, repetition or packed moves
- **THEN** both engines run shared exact FEN+UCI/moves2/hash fixtures, reference
  perft, undo reversibility and independent differential checks before release.
- **AND** performance benchmarks distinguish cold first-use latency, warm
  UI-facing operations and bulk perft/PGN throughput. Rust-only shortcuts and
  JS-only wrappers are allowed, but never wire/rules disagreements.
- **AND** the JavaScript library SHALL NOT require WebAssembly for its core API.
