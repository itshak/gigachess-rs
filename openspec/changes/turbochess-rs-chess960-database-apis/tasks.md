## 1. Chess960 Board Support

- [ ] 1.1 Castling rights as rook squares (per color/side); standard rights map to files a/h; FEN parse (KQkq + Shredder fallback) and output per dialect; verify with round-trip unit tests
- [ ] 1.2 Path-based castling legality + generation (king path safety, between-king-and-rook emptiness, rook-not-pinned; final squares g/c + f/d); verify with 960 perft on known Chess960 reference positions
- [ ] 1.3 moves2 castling encoding switch: king-from → rook-square, both variants; decode by destination-own-rook detection; update all existing castling tests/decoders
- [ ] 1.4 Differential validation vs python-chess (script-generated 960 positions and move sequences)

## 2. Hashing Changes

- [ ] 2.1 Castling hash switch to per (color, rook file) keys; file-a/h keys = Polyglot 768..771; derive the 12 remaining keys at compile time from a documented PRNG; incremental make/unmake updates
- [ ] 2.2 Pin EP condition to Pseudo (already implemented) with regression tests: startpos/Kiwipete Polyglot vectors, pinned-ep vector (`8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1` → `0x83bf25e378cb17d0`), incremental-vs-full parity re-run
- [ ] 2.3 Update zobrist docs (ADR-003 references); confirm standard-chess Polyglot parity via python-chess differential test

## 3. Database Batch APIs

- [ ] 3.1 `parse_movetext_to_moves2`: byte-level tokenizer (move numbers, comments, NAGs, variation skipping, result tokens), in-place play, `Vec<u8>` output; error on illegal SAN
- [ ] 3.2 `moves2_to_san_movetext`: O(1) word decode, shakmaty-byte-identical SAN rendering (differential tests vs shakmaty `SanPlus`), move numbers, result token
- [ ] 3.3 `replay_moves2_hashes` (+ movetext variant): incremental (hash, ply) stream; parity test vs from-scratch recompute at every ply
- [ ] 3.4 Position-statistics builder: batch aggregation of per-position move counts + game samples, rayon-parallel; parity test vs sequential reference
- [ ] 3.5 Codec benches (import path, render path, hash-replay path) recording vs blind-base's current shakmaty implementation timings

## 4. Engine API & Cleanup

- [ ] 4.1 `Board: Copy` (all fields plain data) + test
- [ ] 4.2 `pseudo_legal_moves()` + king-safety filter test proving `legal_moves == filter(pseudo_legal_moves)` across reference positions
- [ ] 4.3 Remove dead `shakmaty-compat` feature from Cargo.toml

## 5. Docs, Migration & Validation

- [ ] 5.1 `MIGRATION.md`: hash re-keying (pinned-ep detection vector + values), moves2 castling re-encode procedure (pseudocode, position-aware), shakmaty→turbochess API mapping table, worked masters_pack before/after example
- [ ] 5.2 ADR-003: Chess960, rook-file castling hashing, breaking encodings — decisions, benchmark/reference table, cross-references (cozy-chess, JS turbochess, python-chess, shakmaty, Polyglot spec, UCI-960)
- [ ] 5.3 Full validation: `cargo check`, `cargo test --release -- --include-ignored`, `cargo clippy --all-targets`, differential python-chess run; README performance table refresh
