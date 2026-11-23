# Rust market, correctness and performance audit — 2026-11-23

Repo `itshak/gigachess-rs` main `4b941b8496d34558009642a875a0bed40d4e74a8`.
Baseline `cargo test --locked --all-targets` passed in an isolated temporary
Rust 1.100 toolchain on Debian aiserver. The crate already tests perft reference
positions, 100 Chess960 FENs, several 960 castle edge cases, 1,000 EPD FENs,
random make/unmake and incremental-vs-full Zobrist. Its benchmark harness
covers cozy-chess, shakmaty and ultrachess. Historical README numbers
(~540 Mnps startpos d5 on M1 Max) are not independently reproduced here.

## Confirmed issues and exclusions
- Rust compiles Polyglot and Chess960 castling keys into static tables, so the
  **JS lazy-load Zobrist bug is not present**. Separate FIDE repetition key
  remains needed for pinned EP (Polyglot intentionally pseudo-legal).
- Rust Board is `Copy` (no shared mutable Sets); JS castling mutation bug is
  structurally inapplicable to Rust. Its `castle_rook_sq` and packed moves2
  match JS king-to-rook canonical wire representation.
- Rust FEN parser accepted longer tokens beginning `w`/`b`, silently accepted
  seventh field, and did not reject an opposite-side king in check. Repairs
  added tests; two prior Chess960 test fixtures were intrinsically illegal,
  repaired to preserve castle geometry.
- Rust already makes *trusted* vs *validated* replay explicit, unlike former
  JS `Board.fromMoves2`; preserve distinct cost/safety contracts.
- Rust uses checked, fully hashed, hashed-only, checkers-only and cache-free
  make modes; do not globally require fresh checkers after `play_fast`.

## Competitor matrix / research opportunities
| Project | License/access | What to evaluate |
|---|---|---|
| `cozy-chess` https://github.com/analog-hors/cozy-chess | MIT | legal move generation, Chess960, bulk visitor / speed and safety; existing benchmark baseline |
| `shakmaty` https://github.com/niklasf/shakmaty | GPL-3.0 | independent rule/SAN/variant oracle in dev only, **never copy** into MIT source |
| `ultrachess` https://github.com/yahorbarkouski/ultrachess | MIT | precomputed tables, compressed undo, bulk count; existing competitor |
| `ferrum-movegen` https://github.com/TheChii/ferrum-movegen | Apache-2.0 | advertised fast bulk perft, leaf counting, PEXT; benchmark only, do not copy Apache source into MIT-only crate |
| `chessie` https://github.com/dannyhammer/chessie | inspect license before reuse | iterator and filtered moves, perft bulk and engine interface |
| `owlchess` https://github.com/alex65536/owlchess | MIT | safety, adversarial FEN and movegen tests |
| `chess_bench` https://github.com/alex65536/chess_bench | benchmark methods; per-engine licenses | neutral corpus/methodology to avoid optimising solely for perft |

## Recommendations
- Keep standard 16-bit packed canonical moves and established castling
  formulas; Rust and JS fixtures must be byte-identical.
- Treat `fastest Rust engine` and `fastest JS` as workload-bound hypotheses,
  compare on identical hardware and lib versions before publishing claims.
- Prioritize the 2 Rust FEN acceptance errors and legal EP repetition; avoid
  invasive changes to the mature perft/PEXT core until profiling shows a gap.
- Add cache-validity and bulk replay correctness tests in addition to perft.
