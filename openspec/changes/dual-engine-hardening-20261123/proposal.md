# Proposal: Rust independent correctness & dual-engine parity hardening

## Why
The Rust core powers high-throughput databases, and its contracts are
interoperable with the pure-TypeScript GigaChess library. The JS audit found
late Zobrist-loading, repetition and castling-state defects; the Rust code is
immune to its lazy-load/shared-Set bugs because `Board: Copy` and Zobrist keys
are compile-time constants, but its FEN parser accepted `white` and ignored
extra fields, and it lacked a FIDE repetition key separate from Polyglot.

The source crate already has a comprehensive perft, Chess960, incremental hash,
SAN differential and concurrency suite. Keep all its existing performance and
encoding assumptions. The companion JS proposal holds the same normative
contract (`specs/mirrored-core-contract/spec.md`).

## What changes
- Add `Board::repetition_key`, which differs from `zobrist()` only when EP is
  pseudo-legal yet ILLEGAL due to king safety. Do not change Polyglot keys.
- Validate exact side-to-move token, FEN field count and opposite-side check,
  matching JS semantics on well-formed four/six-field FEN; repair illegal test
  fixtures without changing Chess960 castling geometry.
- Preserve the original Rust fast variants (`play_fast`/`play_hashed`, etc.),
  zero-alloc movegen, `Board: Copy`, packed moves2 and rook-square castling.
- Extend differential and benchmark coverage with newer Rust competitors,
  stratified into perft, make/undo, SAN, replay, cache validity, FEN and warmup.

## Non-goals
No WASM bridge to GigaChess JS, no change in 16-bit wire layout, custom 960
keys or all-purpose web engine. No GPL code copied into MIT source. Changes in
FEN rejection behavior may affect consumers loading contrived illegal states;
communicate stricter default rejection and offer an explicitly permissive
`parse_fen_allow_illegal` for synthetic test positions before release.
