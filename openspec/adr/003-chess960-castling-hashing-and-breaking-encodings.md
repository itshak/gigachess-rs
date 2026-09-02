# ADR-003: Chess960 Support, Rook-File Castling Hashing, and Breaking Codec Changes

## Status

Accepted (2026-09-01)

## Context

Three consumer-driven requirements converged:

1. **Chess960 support** (standard + Chess960 only). The Polyglot castling
   hashing scheme (4 keys, W-K/W-Q/B-K/B-Q) is lossy for 960: two positions
   differing only in *which* rook file holds a kingside right hash
   identically. Both ecosystem references fold this way — python-chess
   (`hash_castling` via `has_kingside_castling_rights`, "h-side in Chess960")
   and the JS turbochess library (`castlingKeyIdx`, fold by side-of-king) —
   so there is no stricter standard to adopt.
2. **blind-base's hot paths** need batch codecs (`movetext ↔ moves2`, hash
   replay) and an incremental hash. Its current shakmaty-based replay is
   O(n²) per game (FEN round-trip + re-replay from ply 0 per decoded move)
   and decodes each 2-byte move via a full legal-movegen + linear scan.
3. **Project directive**: no compatibility modes or legacy paths — where a
   better scheme exists, switch outright and document migration.

Additionally, a hash-strategy investigation established:
- shakmaty 0.30's `Zobrist64` key table **is the Polyglot key set** (the low
  64 bits of its 128-bit table; its doctest asserts the Polyglot startpos
  hash `0x463b96181691fc9c`). No separate "Zobrist64 standard" exists to
  adopt — we already have the keys, cleanroom, MIT.
- The only semantic divergence from the Polyglot spec is the en-passant key
  condition: shakmaty offers `Always`/`Legal` (no `Pseudo`), while the
  Polyglot spec, python-chess, and the JS turbochess library use the
  pseudo-legal (geometric adjacency) condition — which is O(1) and
  incremental-safe, unlike `Legal` (requires king-safety simulation per
  double-push position and defeats incremental updates; shakmaty's own
  `update_zobrist_hash` bails out in those cases).

## Decision

1. **En-passant hash condition: Pseudo, permanently.** Single condition, no
   toggle. Matches the Polyglot spec, python-chess, and the JS turbochess
   library. Cost: one bitboard AND + table lookup (~1 ns); incremental-safe.
   Apps relying on shakmaty-`Legal` hashes re-hash (see `MIGRATION.md`; the
   only affected positions are those with a pseudo-legal-but-illegal ep
   capture, detection vector `8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1`).
2. **Castling hash: per (color, rook file) — 16 keys.** Rights are
   represented as rook squares. The file-a/h keys are defined as Polyglot
   `768..771`, so **standard-chess positions hash bit-identically to the
   Polyglot spec** (opening books keep working); files b–g are compile-time
   constants derived from a documented deterministic PRNG (cozy-chess
   precedent: compile-time key generation from a published seed). This makes
   Chess960 rights distinguishable (cozy-chess scheme) at zero runtime cost
   over the previous 4-key scheme.
3. **moves2 castling encoding: king-from → rook-square**, for both standard
   and Chess960. This is the only unambiguous encoding (in 960, a normal king
   move can coincide with the castling king-move; the rook destination never
   collides with a normal king move). Precedents: cozy-chess movegen emits
   exactly this; the UCI-960 convention is king→rook-square. Standard-chess
   stored blobs (king→final, `e1g1`/`e1c1`) require a one-time re-encode —
   procedure in `MIGRATION.md`. UCI/SAN rendering follows per-variant
   convention (`e1g1` standard, king→rook in 960, `O-O` in SAN).
4. **Chess960 castling legality: path-based.** Emptiness between king and
   rook (plus the rook destination), king-path and destination safety, rook
   not pinned. Final squares: king g/c, rook f/d — identical files in both
   variants.
5. **FEN dialects: standard KQkq + Shredder file letters** with parse
   auto-fallback (cozy-chess style); standard positions render KQkq, 960
   positions render file letters.
6. **Engine API**: `Board: Copy` (cozy-chess: "`Copy` for performance
   reasons"), `pseudo_legal_moves()` exposed.
7. **`shakmaty-compat` feature removed** — a compat facade inside the MIT
   crate is impossible (GPL dependency) and was already rejected by ADR-001.

## Alternatives considered

- **Polyglot fold for 960 castling hashing** (python-chess/JS behavior):
  rejected — knowingly lossy for 960; no interop benefit because no 960 book
  standard exists, and our rook-file scheme keeps standard-chess Polyglot
  parity anyway.
- **`ep_mode` toggle (Pseudo/Legal)**: rejected — two code paths for a rare
  edge case; consumers needing Legal-mode hashes can XOR the ep key out
  themselves (`hash_without_ep`-style, cozy-chess precedent) or re-hash.
- **Keep 4-key castling hashing + fold, migrate nothing**: rejected — leaves
  960 hashing ambiguous and contradicts the directive.
- **king→final-square castling encoding with ambiguity resolution at decode**:
  rejected — the ambiguity is irresolvable in general (a normal king move and
  a castling move can share from/to), losing information.

## Consequences

- Standard-chess Polyglot interop (books, python-chess, JS turbochess) is
  fully preserved for positions without 960-specific castling rights.
- Chess960 positions hash correctly and distinctly; their hashes are a
  turbochess extension not shared with other tools (documented here and to be
  mirrored in the JS library).
- Stored moves2 blobs with castling moves and shakmaty-`Legal`-mode hash
  indexes require one-time migration (`MIGRATION.md`).
- `Board: Copy` requires the board to remain plain-data (no heap fields in
  hot state); batch APIs already satisfy this.

## Implementation record (2026-09-02)

The decisions above are implemented as follows:

- **Rights representation.** Four right bits (WK, WQ, BK, BQ), each backed by
  a rook square (`Board::castling_rook_square`). Standard chess maps to
  h1/a1/h8/a8. Legal play can never produce two same-side rights for one
  color (rights require king and rooks still on their original squares, and
  960 starts place the king between the rooks), so the per-color/side
  encoding is lossless for reachable positions.
- **FEN dialects.** Parse accepts standard letters (outermost rook of the
  color on its back rank on that side of the king) and Shredder file letters;
  output follows the X-FEN ambiguity rule — byte-identical to python-chess
  `castling_xfen` (differential-verified). The two spellings of the same
  rights (`KQkq` vs `DAda` for 960 position 654) parse identically.
- **Derived castling keys.** The 12 non-Polyglot keys come from splitmix64
  with the published seed `zobrist::CASTLE_KEY_SEED = 0x00C0FFEEDABAD00D`,
  generated in slot order (Black files b..g, then White b..g); the a/h keys
  are pinned to Polyglot 768..771. All 16 keys are pairwise distinct.
- **Castling legality.** Path-based per ADR decision 4, implemented with a
  standard-geometry fast path (king-removed danger map; provably equivalent
  because h1/a1/d1/f1 rook placement cannot uncover or block a relevant ray)
  and a general Chess960 path that evaluates king-path safety in the
  post-castling occupancy (king removed for x-rays, rook displaced to its
  final square — which subsumes the rook-not-pinned rule).
- **make/unmake.** Castling words are king-from → rook-square; make and
  unmake remove both pieces then place both (adjacent king+rook castling
  swaps their squares). `Undo` carries a `castled` flag: a 960 king move and
  a castling move can share from/to squares, so post-hoc detection is not
  reliable.
- **pseudo_legal_moves / Copy.** `Board` is plain data and `Copy`;
  `pseudo_legal_moves()` carries full castling path legality (a king-safety
  post-filter cannot restore transit-square safety), so
  `legal_moves() == king-safety-filter(pseudo_legal_moves())` holds exactly
  (tested across reference and 960 positions).
- **Validation.** Differential harness `scripts/diff_python_chess.py`
  (python-chess oracle, `tc_probe` binary): 23,907 positions from random
  Chess960 and standard games — legal move sets, perft(2), byte-identical
  X-FEN round-trips, and Polyglot hash parity for standard chess all match.
  960 perft(3) reference counts for start positions 0/284/518/959 are
  hard-coded in `tests/chess960.rs`. SAN rendering is byte-identical to
  shakmaty `SanPlus` over 3,000+ random positions (`tests/san_parity.rs`).

## References

- ADR-001: Maximum Performance Primacy and Pure Native API Architecture
- ADR-002: Rayon as the sole batch-replay backend
- cozy-chess (MIT): rook-file castling keys, compile-time PCG64 key
  generation, Shredder-FEN auto-fallback, king→rook castling encoding,
  `Copy` board (`analog-hors/cozy-chess`, `src/board/zobrist.rs`,
  `src/board/parse.rs`, `src/board/movegen/mod.rs`)
- JS turbochess (MIT): Polyglot keys, Pseudo ep condition
  (`src/zobrist.ts::epIsHashable`), incremental `{lo, hi}` maintenance
- python-chess (BSD): Polyglot hash semantics (`chess/polyglot.py`), 960
  side-of-king fold (`has_kingside_castling_rights`)
- shakmaty 0.30 (GPL — reference only): `Zobrist64` = Polyglot low-half keys,
  `EnPassantMode::Always/Legal` semantics, incremental-update bail-out
- Polyglot opening-book specification: 781-key array, ep pseudo-legal
  condition, castling key order (W-K, W-Q, B-K, B-Q)
- UCI Chess960 convention: castling encoded king→rook-square

