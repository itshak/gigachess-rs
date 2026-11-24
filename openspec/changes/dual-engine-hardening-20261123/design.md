# Rust architecture and compatibility decision

## No JS/WASM linkage
Retain the pure-Rust crate for native throughput and the pure-TypeScript
package for low-startup-latency app interactions. Never make JS init depend on
WASM. Same rules and encodings, independent implementation structures.

## Hash semantics
The `Board` caches incremental Polyglot `u64` keys; `zobrist_full()` can
recompute. They include EP whenever a side-to-move pawn geometrically attacks
its EP target, even if king safety prevents capture. `repetition_key()` first
checks geometric EP; only when present does it check legal EP moves with the
same `Board::is_legal` implementation used for game rules. If none is legal,
XOR the canonical Polyglot EP file key out. If no EP, return cached hash with
one branch. This is mirrored by `gigachess` JS `repetitionKey()`; neither
changes the opening-book/position-index hash. `play_fast`/perft-only pathways
can leave the incremental key stale and must not be passed to key consumers.

## Validated FEN
Require exactly `w` or `b`, no trailing fields beyond six; enforce the
opponent king not in check after last move, already enforced by JS parseFen.
`parse_fen_allow_illegal()` is a named escape hatch for contrived, unreachable
analysis positions that need opposite-king check; the checked default remains
safe and other FEN validations remain in force.
Four-field EPD and six-field FEN remain supported; Rust intentionally requires
at least four fields, while `chessops`-compatible TS accepts abbreviated FENs.
This is a parser input convenience difference, not a difference in valid board
representation. Invalid, contrived Chess960 test positions must be replaced
with legal ones preserving castle king/rook geometry.

## Performance study
Do not replace proven perft `MoveCounter`, visitor and slim perft make paths
without evidence. Profile standard+960, BMI2 feature and fallback; separate
`Board::perft` from actual database replay, SAN and FEN. Existing bench
`benches/vs_libraries` covers Shakmaty and cozy-chess; add open-source
ferrum-movegen and independent benchmarks as *test opponents*, not copied
implementations. Keep the crate MIT and document dependent license nuances.

## Release safety
Compile/test debug + release, feature `pext` if supported, run property tests
and shared JS/Rust golden vectors. Public addition is backward-compatible
except stricter rejection of malformed or unreachable FENs. Any change to
castling, hashing, promotion or packed moves needs two-repo review.
