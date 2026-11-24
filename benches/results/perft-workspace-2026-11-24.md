# Rust perft depth workspace — 2026-11-24

Server: aiserver, Rust 1.100.0, release LTO=fat codegen-units=1.
Original gigachess-rs Board::perft versus experimental Board::perft_reuse.
Nine alternating timed runs after warmup, identical source and compiler.
All canonical perft counts match; complete locked test suite passed;
Chess960 reference count equality passed.

| Position | Original Mnodes/s | Reused workspace Mnodes/s | Relative |
|---|---:|---:|---:|
| Start d5 | 213.322 | 215.992 | +1.25% |
| Kiwipete d4 | 345.891 | 349.709 | +1.10% |
| Endgame d5 | 181.127 | 187.533 | +3.54% |
| Promotion d4 | 287.468 | 291.563 | +1.42% |
| Position 4 d4 | 351.416 | 355.827 | +1.26% |

The existing code ALREADY uses cached checkers, split pin computation,
bulk pawn move generation, uninitialized MoveList buffers, and a specialized
perft move/unmove path skipping Zobrist. These experiments did NOT invent
those techniques. Extra depth-scoped buffers are an **additive API** for
further study; default Board::perft and normal make/undo are unchanged.
Small gains need confirmation on independent machines.

Raw measured timings: perft-workspace-2026-11-24.json.
