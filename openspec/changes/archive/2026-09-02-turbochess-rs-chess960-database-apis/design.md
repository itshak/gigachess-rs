## Context

Follow-up to the archived `turbochess-rs-core-engine` change. Consumer research
(blind-base `src-tauri/src/gigabase_moves.rs`, `masters_pack.rs`,
`position_index/replay.rs`) showed: blind-base's moves2 blob format is
byte-identical to ours; its hot paths decode each 2-byte move via a full
legal-movegen + linear scan, re-replay games from ply 0 per decoded move
(O(n²)) via FEN string round-trips, and recompute Zobrist hashes from scratch
every ply (`zobrist_hash::<Zobrist64>(EnPassantMode::Legal)`).

Key research facts (verified against primary sources):
- shakmaty 0.30's `Zobrist64` key table is the **Polyglot key set** (low 64
  bits of its 128-bit table; doctest asserts startpos `0x463b96181691fc9c`).
- shakmaty's only strict difference from Polyglot is the ep condition: it has
  no `Pseudo` mode (`Always`/`Legal` only). blind-base's stored hashes use
  `Legal`.
- JS turbochess (`src/zobrist.ts::epIsHashable`) and python-chess use the
  Pseudo (geometric adjacency) condition — identical to ours. The JS library
  hashes castling by side-of-king fold (4 keys) — lossy in 960.
- cozy-chess (MIT, cited in ADR-001) officially supports Chess960: Shredder
  FEN parsing with auto-fallback, castling rights as rook files, castling
  generated as king→rook-square, zobrist keys **per (color, rook file)**,
  `Copy` board "for performance reasons".

## Goals / Non-Goals

**Goals:**
- Chess960 board support (representation, castling legality, FEN dialects,
  moves2 encoding, hashing).
- Single, lean hash semantics: Polyglot keys, Pseudo ep — no toggles.
- Batch codecs for blind-base-shaped workloads with zero intermediate Strings.
- Engine-facing API: `Board: Copy`, `pseudo_legal_moves()`.
- Migration documentation for apps with different hashes/encodings.
- A head-to-head comparative benchmark suite vs shakmaty and cozy-chess
  (dev-dependencies) on every hot-path axis, with published results and a
  best-in-class (non-Rust, e.g. Stockfish) stretch target — the library must
  win its own benchmark suite.

**Non-Goals:**
- Other chess variants (Atomic, Crazyhouse, …) — ADR-001.
- A shakmaty facade inside turbochess-rs (would make the MIT crate depend on
  GPL shakmaty) — rejected; belongs to the AGPL consumer if needed.
- Backward compatibility modes for the old castling hash/encoding (project
  directive: switch outright; migration guide covers the data move).

## Decisions

### D1: EP hash condition — Pseudo, permanently
`ep_contribution` stays as implemented (geometric adjacency test, O(1),
incremental-safe). No `ep_mode` toggle. Consequence: hashes differ from
shakmaty-`Legal` only in pinned-ep positions (detection vector and hash values
documented in `MIGRATION.md`).

### D2: Castling hashing — per (color, rook file), 16 keys
Keys: file-a/h per color **are** Polyglot `768..771` (standard-chess positions
hash bit-identically to Polyglot — books keep working); files b–g are
compile-time constants derived from a documented PRNG (cozy-chess precedent:
compile-time key generation from a published seed). Runtime cost identical to
the previous 4-key scheme (one XOR per right). Incremental make/unmake XORs
out/in affected rights' keys.

### D3: Castling representation and moves2 encoding — rook squares
Rights: rook squares per color/side. moves2: castling = king-from →
rook-square (both variants; cozy-chess and UCI-960 precedent). Decode detects
castling by destination holding the mover's own rook. Standard-chess stored
blobs (king→final, e.g. `e1g1`) require one-time re-encode — migration guide
with position-aware procedure. UCI/SAN rendering follows per-variant
convention (`e1g1` standard, king→rook 960, `O-O` SAN).

### D4: Chess960 castling legality — path-based
Emptiness: squares strictly between king and rook (rook's own destination
included). Safety: king's path to destination + destination. Plus rook-not-
pinned (cozy-chess rule). Final squares: king g/c, rook f/d — same files in
standard and 960.

### D5: Engine API — `Board: Copy` + `pseudo_legal_moves()`
All fields are plain data; `Copy` gives bit-for-bit snapshots for engine
search (cozy-chess: "`Copy` for performance reasons"). Pseudo-legal
generation serves engines with their own legality filters.

### D6: Batch APIs — shaped exactly on blind-base's call sites
`parse_movetext_to_moves2`, `moves2_to_san_movetext`, `replay_moves2_hashes`,
position-stats builder — same names/semantics as blind-base's
`gigabase_moves.rs` so that file becomes a thin shim. SAN rendering must be
byte-identical to shakmaty `SanPlus` (stored DB strings rely on it).

## Risks / Trade-offs

- **Breaking**: moves2 castling words and (rare) hash values change; migration
  guide (`MIGRATION.md`) is a required deliverable, not an afterthought.
- **SAN byte-parity risk**: check/mate suffixes, disambiguation edge cases and
  castling notation must match shakmaty exactly; validated by
  differential tests against shakmaty (dev-dependency, test-only).
- **960 hashing has no interop target**: our rook-file scheme diverges from
  the JS/python-chess fold for 960 positions only; standard chess is
  unaffected (documented in ADR-003).
- shakmaty's `from_setup` rejects some positions python-chess accepts
  (observed: pinned-ep FEN above) — differential tests must generate
  positions by play, not by FEN strings alone.

## Migration Plan

`MIGRATION.md` (repo root) covers: (1) Zobrist re-keying from
shakmaty-`Legal`-mode hashes with the pinned-ep detection vector
(`8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1`: Legal `0x9f26fb3044738771` vs Pseudo
`0x83bf25e378cb17d0`); (2) position-aware moves2 castling re-encode
procedure with pseudocode; (3) shakmaty→turbochess-rs API mapping table with
per-call-site performance notes; (4) worked before/after example of the
masters_pack replay loop.

## Open Questions

- None blocking; Chess960 book hashing interop is explicitly out of scope
  (no standard exists; rook-file scheme documented in ADR-003).
