# Proposal: Chess960 Support, Rook-File Castling Hashing, and Database/Engine APIs

## Why

`turbochess-rs` must serve two consumers with one lean API:

1. **blind-base** (primary consumer) — a high-throughput chess database whose
   hot paths are PGN import (`movetext → moves2`), movetext rendering
   (`moves2 → SAN`), position indexing (per-ply hashes), and opening-tree
   statistics. Its current shakmaty-based loops cost O(n²) per game in one
   path (FEN round-trip + re-replay per ply), do a full legal-movegen + linear
   scan per ply just to decode a 2-byte move, and recompute Zobrist hashes
   from scratch every ply. blind-base's `moves2` blob format is already
   byte-identical to ours (`u16 LE = from | to<<6 | promo<<12`), so the
   library can consume its stored data directly once the missing batch codecs
   exist.
2. **Rust chess engines** — which need a `Copy` board, pseudo-legal movegen,
   public attacks, and an incremental hash, with zero API-style overhead.

Additionally, Chess960 must be supported (standard + Chess960 only — no other
variants), and several architecture decisions taken during review must be
recorded and implemented as breaking changes (per project directive: no
compatibility modes — switch outright):

- **EP hash mode**: exactly one — Pseudo (Polyglot spec). Matches python-chess
  and the JS turbochess library (verified in `src/zobrist.ts::epIsHashable`);
  O(1) and incremental-safe. shakmaty's `Legal` mode (used by blind-base's
  legacy hashes) is not implemented; migrators re-hash.
- **Castling hash**: per (color, rook file) — 16 keys — replacing the 4-key
  Polyglot fold. Collision-proof for Chess960 (the fold is lossy there);
  file-a/h keys are defined as Polyglot `768..771`, so every standard-chess
  position still hashes bit-identically to Polyglot (opening books keep
  working; the 12 other keys are compile-time constants derived from a
  documented PRNG, cozy-chess style).
- **moves2 castling encoding**: king-from → **rook square** for both standard
  and Chess960 (the only unambiguous encoding; UCI-960 and cozy-chess
  precedent). Breaking for stored blobs → migration guide required.

## What Changes

- **Chess960 board support**: castling rights as rook squares, path-based
  castling legality (between-king-and-rook emptiness + king-path safety +
  rook-not-pinned), fixed final squares (king g/c, rook f/d), Shredder/X-FEN
  castling notation parsing with standard-KQkq auto-fallback (cozy-chess
  style), moves2 king→rook castling encoding.
- **Zobrist**: EP key condition fixed to Pseudo (Polyglot spec — no toggle);
  castling keys switched to per-rook-file (16 keys, 4 of them = Polyglot
  768..771, 12 derived at compile time from a documented PRNG).
- **Database batch APIs** (new `database`-oriented module surface, modeled on
  blind-base's `gigabase_moves.rs` semantics):
  - `parse_movetext_to_moves2(start_fen, movetext) -> Vec<u8>` — streaming
    byte-level tokenizer, mainline-only, tolerant of comments/NAGs/variations.
  - `moves2_to_san_movetext(start_fen, moves2, result) -> String`.
  - `replay_moves2_hashes(start_fen, moves2) -> iterator of (hash, ply)` with
    incremental hashes.
  - `position_stats(games) -> HashMap<u64, MoveStats>` building helper for
    opening-tree/index workloads.
- **Engine API**: `Board: Copy`, `pseudo_legal_moves()`, public `make_move`
  (unchecked fast path is already public).
- **Cleanup**: remove the dead `shakmaty-compat` feature from Cargo.toml.
- **Docs**: `MIGRATION.md` for blind-base and similar apps (hash re-keying,
  moves2 castling word re-encode procedure, shakmaty→turbochess API mapping,
  worked before/after example of the masters_pack replay loop).
- **ADR-003** recording the decisions above with cross-references to
  cozy-chess, JS turbochess, python-chess, shakmaty, the Polyglot spec, and
  the UCI-960 convention.

## Capabilities

### New Capabilities
- `turbochess-rs-chess960`: Chess960 board representation, castling legality,
  FEN dialects, and castling hashing.
- `turbochess-rs-database-codecs`: movetext/moves2 batch codecs, hash
  replay streams, and position-statistics building for database workloads.

### Modified Capabilities
- `turbochess-rs-core-engine`: EP hash condition pinned to Pseudo; castling
  hash switched to rook-file keys; moves2 castling words redefined
  (king→rook); `Board: Copy`; `pseudo_legal_moves()` added.

## Impact

- **Performance**: blind-base's masters-pack replay path drops from O(n²) to
  O(n) per game; moves2 decode goes from full movegen + linear scan to O(1)
  word decode; per-ply hashing becomes incremental. Target ≥10× on replay and
  indexing paths (to be measured with the existing benches plus new
  codec benches).
- **Breaking changes**: moves2 castling words (stored blobs need one-time
  re-encode — migration guide provided); hash values differ from
  shakmaty-`Legal`-mode hashes in rare pinned-ep positions (documented with a
  detection test vector); `Board` becomes `Copy`.
- **License**: unchanged — MIT; no GPL-derived code or constants beyond the
  public Polyglot spec data already in use.
