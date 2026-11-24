# Rust move-generation SOTA audit — 2026-11-24

## Release decision
GigaChess-RS v0.1.11 should ship the correctness and interoperability work once all-feature tests and cargo publish dry-run pass. Experimental performance optimizations are not a release blocker; claims of absolute fastest cannot be established across different CPUs.

## Independent upstream review
- Ferrum Movegen (https://github.com/TheChii/ferrum-movegen, inspected commit cd46b93): legal movegen, leaf bulk count, PEXT/magic and make/undo; published 343–434 Mnps are from different hardware. Note license discrepancy: README says Apache-2.0 while Cargo.toml says MIT. Avoid copying code until resolved.
- Gaia Chess (https://github.com/jromang/gaiachess, inspected 85f8c96): runtime dispatch among PEXT, AVX2, AVX-512/setwise kernels and magic fallback; avoid slow-microcoded PEXT on certain AMD CPUs. GPL-3.0-or-later, use only conceptual inspiration and clean-room implementation.
- Cozy Chess (https://github.com/analog-hors/cozy-chess): bulk destination masks, optional PEXT and robust Chess960/DFRC; remains relevant as a differential oracle.
- Shakmaty (https://github.com/niklasf/shakmaty): robust variant/FEN/SAN semantics and make/unmake, important for real-world application-operation comparisons.
- chess_bench (https://github.com/alex65536/chess_bench, inspected cd8b851): Hperft makes every last-ply move and hashes resulting board, exposing misleading leaf-popcount perft wins. GPL source; reproduce experiment independently.
- chessbot (https://github.com/pfyhr/chessbot): reports cozy bulk-generation speed but better Shakmaty full make-move in its MCTS workload; test the relevant workload.
- shahmat (https://docs.rs/shahmat/1.0.2/shahmat/movegen/index.html): analytic en-passant legality, packed undo, depth-two quiet-reply-count reuse. Promising for the next isolated experiment but risky in EP/check/Chess960 edge cases.

## Already in GigaChess-RS
- Compact u64 bitboards and magic/PEXT slider attacks.
- Bulk pawn shifts, MoveSink/MoveCounter, cached checker bitboard, split pinned directions.
- Dedicated perft make/unmake skipping hash and clocks; no change in public make and replay.
- 16-bit moves2, Chess960, Polyglot Zobrist, FIDE repetition semantics with legally relevant EP.
- New experimental per-depth MoveList reuse: +1.1 to +3.5 percent on five benchmarks (Xeon E5-1620 v2, nine alternating release runs, exact reference nodes).

## Follow-up research plan — not release blockers
1. Implement an independently specified Hperft-style full make/unmake checksum benchmark vs Cozy and Shakmaty, plus realistic indexed-game replay.
2. Compare runtime-selected magic/PEXT/AVX2 kernel choices on modern Intel, AMD, and Apple Silicon; no fake ranking from cross-machine results.
3. Prototype shahmat-style depth-two bulk response count under separate feature/branch, with 960/EP/CPW/random correctness gates.
4. Measure packed undo state and specialize quiet moves, checking full FEN/clocks/hash after every move.
5. Profile code footprint/cache behavior; consider PGO only if gains reproduce on real user hardware.

No third-party source code was imported. Keep experimental optimizations separate from the patch release.


## Same-server Ferrum control comparison

On aiserver (Intel Xeon E5-1620 v2), the upstream Ferrum commit cd46b93
was cloned and independently benchmarked using its public generate_moves,
generate_moves_count and make_move_new path. Exact canonical node counts
were asserted before any timing. Each position ran nine measured release
iterations after warmup, no PEXT (the old Xeon lacks BMI2).

| Position | GigaChess perft Mnodes/s | Ferrum perft Mnodes/s |
|---|---:|---:|
| Start d5 | 213.322 | 130.409 |
| Kiwipete d4 | 345.891 | 171.475 |
| Endgame d5 | 181.127 | 78.832 |
| Promotion d4 | 287.468 | 279.192 |

GigaChess uses the original reference perft (not perft_reuse), Rust
LTO fat/cgu1 release; Ferrum uses upstream LTO fat/cgu1 release.
These are same-server, similar workloads with correct node counts,
but Ferrum has extra original benchmark configurations and the CPU
is much older than CPUs reported upstream. No cross-machine SOTA
ranking follows from either table. The upstream README's Apache
statement contradicts its Cargo.toml MIT declaration, so do not
copy code without reconciling the licensing terms.
