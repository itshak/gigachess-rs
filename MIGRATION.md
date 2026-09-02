# MIGRATION: turbochess-rs Chess960 / Hashing / Codec Changes

This release intentionally breaks three stored-data contracts (project
directive: no compatibility modes — switch outright; see
[ADR-003](openspec/adr/003-chess960-castling-hashing-and-breaking-encodings.md)).
Every affected consumer needs a **one-time migration**; all procedures are
position-aware and streaming-friendly.

| Change | Affects | Effort |
|---|---|---|
| 1. En-passant hash condition pinned to Pseudo (Polyglot) | stored Zobrist indexes | rare positions only |
| 2. Castling hash keys switched to per (color, rook file) | stored Zobrist indexes (Chess960 only) | Chess960 data only |
| 3. moves2 castling words re-encoded king-from → rook-square | stored moves2 blobs with castling moves | one pass over blobs |
| 4. `Board` is now `Copy`; `pseudo_legal_moves()` added | engine-style consumers | compile-time |

---

## 1. Zobrist re-keying (EP condition: Legal → Pseudo)

The hash of every position is unchanged **except** positions where an
en-passant capture is pseudo-legally available but illegal (the capturer is
pinned or the capture would expose the king). Under the old shakmaty-`Legal`
semantics those positions omitted the ep key; Polyglot (and turbochess-rs)
include it.

**Detection vector** (the only class of divergent positions):

```text
FEN:    8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1
shakmaty (Legal):  0x9f26fb3044738771
turbochess (Pseudo): 0x83bf25e378cb17d0
```

**Procedure:** re-hash. Do not try to patch stored hashes — the condition is
position-dependent and not recoverable from the stored value. Re-run your
indexer over the games (see §4 for the replay loop; `replay_moves2_hashes`
yields the new hashes incrementally).

Standard-chess positions hash **bit-identically to the Polyglot book
specification** (startpos `0x463b96181691fc9c`, Kiwipete
`0xc3ce103f01d15e1d` — differential-verified against python-chess), so
opening-book interop is unaffected.

## 2. Castling hash keys: per (color, rook file)

- Standard chess: the a/h-file keys **are** the Polyglot castling keys
  (768..771) — hashes of standard positions are unchanged by this switch.
- Chess960: keys for rook files b..g are compile-time constants derived from
  a documented splitmix64 PRNG (`zobrist::CASTLE_KEY_SEED = 0x00C0FFEEDABAD00D`).
  If you stored 960 hashes computed with a side-of-king fold (python-chess /
  JS turbochess style), those were lossy and cannot be mapped — re-hash the
  960 games.

## 3. moves2 castling words: king→final-square → king→rook-square

Old blobs encode standard castling as the king's final square
(`e1g1`/`e1c1`/`e8g8`/`e8c8`). New blobs encode castling as
**king-from → rook-square** (`e1h1`, `d1f1`, …) — the only unambiguous form
for Chess960 (UCI-960 / cozy-chess convention). Decoding is by
*destination-own-rook detection*: a king move whose destination holds the
mover's own rook is castling; no file heuristics are used.

**Position-aware re-encode procedure** (a castling word is only identifiable
once the position before it is known — replay sequentially):

```text
words_in  : little-endian u16 stream of the old blob
words_out : empty
board     : Board::from_fen(start_fen)
for word in words_in:
    (from, to, promo) = decode(word)
    piece = board.piece_at(from)
    if piece.role == KING
       and board.piece_at(to) is a rook of piece.color   # already new format
       and to is one of board.castling_rook_squares():
        emit word unchanged                       # new-format castling
    else if piece.role == KING
       and piece.color king file == e-file (or any) and to == king_final_square
       and is one of e1g1/e1c1/e8g8/e8c8 shapes
       and board.castling_rights() has the matching side
       and board.castling_rook_square(side) != to:
        rook_sq = board.castling_rook_square(matching side)
        emit encode(from, rook_sq, promo)         # RE-ENCODE
    else:
        emit word unchanged
    board.play(decoded move)
```

In practice the old-format detector reduces to: the mover is a king, `to` is
the king's final square (g/c-file) for a right the side still holds, and `to`
does **not** hold the mover's rook. Standard blobs only ever contain the four
shapes `e1g1 e1c1 e8g8 e8c8`.

After re-encoding, verify: replay the new blob and compare the final FEN and
per-ply hashes against a reference implementation.

## 4. shakmaty → turbochess-rs API mapping

| shakmaty (0.30) | turbochess-rs | Notes |
|---|---|---|
| `Chess::default()` / `Chess::from_setup` | `Board::startpos()` / `fen::parse_fen` | FEN parser validates + rejects illegal placements |
| `pos.legal_moves()` | `board.legal_moves()` | `ArrayVec<Move, 256>`, zero-alloc |
| — | `board.pseudo_legal_moves()` | engines with their own legality filters |
| `pos.san_candidates` + `San::disambiguate` | `san::move_to_san(&board, mv)` | byte-identical to `SanPlus` (differential-tested) |
| `San::to_move` | `san::san_to_move(&board, tok)` | |
| `pos.play(m)` | `board.play(m)` / `make_move_unchecked` + `unmake_move` | unchecked pair is the engine hot path |
| `pos.castling_rights()` | `board.castling_rights()` + `castling_rook_square(bit)` | rights carry their rook squares |
| `zobrist::Zobrist64` + `EnPassantMode::Legal` | `board.zobrist()` (incremental) | Pseudo ep; see §1 |
| `zobrist::update_zobrist_hash` | `board.zobrist()` after `make_move_unchecked` | always O(1) (shakmaty bails out in pinned-ep positions) |
| FEN strings as replay carrier | `replay_moves2_batch` / `database::replay_moves2_hashes` | O(n) per game, no string round-trips |
| legal-movegen scan to decode a stored move word | direct `Move::from_word` + `board.play` | O(1) decode; castling via own-rook detection |
| UCI: `e1g1` | castling words are `e1h1`-style; `Board`-aware UCI rendering keeps `e1g1` for standard positions | per-variant convention (ADR-003) |

### Worked example: the masters_pack replay loop

**Before (shakmaty, O(n²) per game):**

```rust
let mut pos = Chess::default();
for san_token in game {
    let mv = San::from_ascii(token)?.to_move(&pos)?;
    pos = pos.play(mv)?;
    // per decoded move: rebuild FEN, re-replay from ply 0 to index positions
    let fen = fen::ToFen::fen(&pos);
    let hash = zobrist_hash::<Zobrist64>(
        Chess::from_setup(Position::from_fen(fen)?)?, EnPassantMode::Legal,
    );
    index(hash);
}
```

**After (turbochess-rs, O(n) per game):**

```rust
use turbochess_rs::database;

// Import: PGN movetext -> binary moves2 (streaming, no intermediate Strings).
let blob: Vec<u8> = database::parse_movetext_to_moves2(&start_fen, movetext)?;

// Index: incremental Polyglot hashes for every ply, in one pass.
let words: Vec<u16> = blob.chunks_exact(2)
    .map(|p| u16::from_le_bytes([p[0], p[1]]))
    .collect();
for (hash, ply) in database::replay_moves2_hashes(&start_fen, &words)? {
    index(hash, ply);
}

// Render: moves2 -> canonical SAN movetext (shakmaty-identical bytes).
let pgn = database::moves2_to_san_movetext(&start_fen, &blob, "1-0")?;
```

The `database::position_stats` builder aggregates per-position move counts
and game samples across whole batches on Rayon's pool with sequential-order
parity.

## 5. Chess960 notes

- FEN: standard `KQkq` letters select the outermost rook on that side of the
  king (X-FEN, python-chess-compatible); Shredder file letters (`HAha`) are
  accepted for any rook file; output follows the X-FEN ambiguity rule.
- Castling legality is path-based: emptiness between king and rook, king-path
  safety in the post-castling occupancy (handles the 960 rank-pin / rook
  removal cases), king and rook final squares g/f + c/d.
- Adjacent king+rook castling swaps the pieces; the moves2 word is still
  king→rook-square and decodes unambiguously.
- Validated against python-chess: 23,907 positions (legal move sets, perft,
  FEN round-trips, standard-chess Polyglot hashes) via
  `scripts/diff_python_chess.py`.
