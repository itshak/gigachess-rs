# Experiment design

**JS:** Each check/pin context owns its castling plans; no global mutable
plans. Perft uses one Uint16 buffer and one reusable analysis context per
ply. Enemy occupancy limits potential sniper rays beyond opposing blockers.
Public makeMove always maintains Zobrist; the perft-only private core may
skip it during recursion but restores the original cached key through undo.
The benchmark explicitly loads Zobrist for BOTH main and candidate packages
with --hash-on; uninitialized tables give a different baseline.

**Rust:** Existing source already uses cached checker bitboards, split
pinned-diagonal/orthogonal masks, fast Make/Undo and hash-free perft.
Experimental perft_reuse uses one MoveList per ply, disjoint child
slices and the existing make_move_perft. It remains separate from the
stable Board::perft API pending more independent benchmarks.

**Measurement:** fixed published reference counts, fresh samples, nine
alternating timed rounds for Rust release (lto=fat); five to nine
alternating JS rounds in Node, with and without loaded hash tables.
All perft nodes, Polyglot, FEN and Chess960 results must remain identical.
