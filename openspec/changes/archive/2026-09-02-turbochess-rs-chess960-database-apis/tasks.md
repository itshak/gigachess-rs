## 1. Chess960 Board Support

- [x] 1.1 Castling rights as rook squares (per color/side); standard rights map to files a/h; FEN parse (KQkq + Shredder fallback) and output per dialect; verify with round-trip unit tests
- [x] 1.2 Path-based castling legality + generation (king path safety, between-king-and-rook emptiness, rook-not-pinned; final squares g/c + f/d); verify with 960 perft on known Chess960 reference positions
- [x] 1.3 moves2 castling encoding switch: king-from → rook-square, both variants; decode by destination-own-rook detection; update all existing castling tests/decoders
- [x] 1.4 Differential validation vs python-chess (script-generated 960 positions and move sequences)

## 2. Hashing Changes

- [x] 2.1 Castling hash switch to per (color, rook file) keys; file-a/h keys = Polyglot 768..771; derive the 12 remaining keys at compile time from a documented PRNG; incremental make/unmake updates
- [x] 2.2 Pin EP condition to Pseudo (already implemented) with regression tests: startpos/Kiwipete Polyglot vectors, pinned-ep vector (`8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1` → `0x83bf25e378cb17d0`), incremental-vs-full parity re-run
- [x] 2.3 Update zobrist docs (ADR-003 references); confirm standard-chess Polyglot parity via python-chess differential test

## 3. Database Batch APIs

- [x] 3.1 `parse_movetext_to_moves2`: byte-level tokenizer (move numbers, comments, NAGs, variation skipping, result tokens), in-place play, `Vec<u8>` output; error on illegal SAN
- [x] 3.2 `moves2_to_san_movetext`: O(1) word decode, shakmaty-byte-identical SAN rendering (differential tests vs shakmaty `SanPlus`), move numbers, result token
- [x] 3.3 `replay_moves2_hashes` (+ movetext variant): incremental (hash, ply) stream; parity test vs from-scratch recompute at every ply
- [x] 3.4 Position-statistics builder: batch aggregation of per-position move counts + game samples, rayon-parallel; parity test vs sequential reference
- [x] 3.5 Codec benches (import path, render path, hash-replay path) recording vs blind-base's current shakmaty implementation timings

## 4. Engine API & Cleanup

- [x] 4.1 `Board: Copy` (all fields plain data) + test
- [x] 4.2 `pseudo_legal_moves()` + king-safety filter test proving `legal_moves == filter(pseudo_legal_moves)` across reference positions
- [x] 4.3 Remove dead `shakmaty-compat` feature from Cargo.toml

## 5. Docs, Migration & Validation

- [x] 5.1 `MIGRATION.md`: hash re-keying (pinned-ep detection vector + values), moves2 castling re-encode procedure (pseudocode, position-aware), shakmaty→turbochess API mapping table, worked masters_pack before/after example
- [x] 5.2 ADR-003: Chess960, rook-file castling hashing, breaking encodings — decisions, benchmark/reference table, cross-references (cozy-chess, JS turbochess, python-chess, shakmaty, Polyglot spec, UCI-960)
- [x] 5.3 Full validation: `cargo check`, `cargo test --release -- --include-ignored`, `cargo clippy --all-targets`, differential python-chess run; README performance table refresh

## 6. Comparative Benchmark Suite (head-to-head vs best-in-class)

- [x] 6.1 Add cozy-chess to dev-dependencies; build the head-to-head Criterion suite (`benches/vs_libraries/`): legal movegen, perft (bulk + non-bulk), board copy, make-move, FEN parse/format, SAN parse/render, Zobrist scratch + incremental — each axis measuring turbochess-rs vs shakmaty vs cozy-chess on identical inputs (startpos, Kiwipete, 960 position)
- [x] 6.2 Import/replay benches: movetext→moves2, moves2→SAN movetext, and hash replay — turbochess-rs vs a shakmaty baseline mirroring blind-base's `gigabase_moves.rs` loops (O(n²) gigabase decode included) and vs shakmaty primitives
- [x] 6.3 Publish the README results table (machine context: M1 Max/10 cores); turbochess-rs ≥ both references per axis or gap documented with a follow-up issue; include best-in-class non-Rust context (Stockfish perft rates) as stretch targets
- [x] 6.4 Optimize any axis where a reference library wins until parity or better; record the delta and technique in the bench module docs
