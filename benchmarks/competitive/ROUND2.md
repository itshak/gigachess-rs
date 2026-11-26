# Round 2: state mutation and remaining deficits
Baseline: configuration C, 15 features (CI 45767325283).
All candidates are independent feature flags, disabled by default. Existing
public moves2 words, full Polyglot hash, cached checkers and Chess960 state
are preserved. No production code copied from Shakmaty (GPL) or Ferrum
(ambiguous repository license). MIT Cozy-Chess 0.3.4 board/mod.rs and
UltraChess ddc2e95 rust/core/src/position.rs were consulted for the
normal-move specialization and ray-candidate approach; implementations
here are independently written against our existing Board interface.
- normal-kernel: one normal path for captures and quiets; one role/color
  occupancy XOR, direct piece keys, no repeated color decode.
- checker-rays: empty-board slider candidates and between occupancy tests
  instead of magic tables for attackers_to.
- checked-direct: pre-mutation hypothetical safety, castle fallback.
- uci-bytes: one allocation and stack ASCII encoding, all words tested.
- hash-unroll: compile-time role/color loops for full hash.
- full-visitor: visit every slim perft leaf, observable same exact Board,
  skip checker refresh only at internal nodes when fused geometry is active.
- direct-list not pursued: legal_moves already uses direct ArrayVec.
Windows summary output changed to ASCII, fixing cp1252 encoding error.

## Layout follow-up (paired baseline C)
- kingless-board: derive king bitboards as occupied minus the five other
  role planes. Keep king-square cache; preserves malformed FEN rejection
  and empty-board behavior. Expected Board size 88 bytes.
- known-role: skip source color occupancy test only on trusted moves,
  avoid role modulo in normal kernel.
- static-unchecked: remove data-array bounds check (descriptor square check
  remains; generated index bounds exhaustively verified).
- castle-hash-table: precompute 16 standard castling-mask hash deltas,
  fall back for nonstandard rook files.
- Generic x86 feature pext: build static PEXT and magic tables, choose
  runtime BMI2 when available. Native BMI2 builds still use static PEXT.
Benchmark-only default features select C; root production defaults unchanged.

## Mutation follow-up
- color-make: dispatch once by side, specialize all color indices and
  pawn direction in the existing full-state transition.
- checked-known: reuse the source piece code for pseudo geometry,
  hypothetical king safety and the eventual make; retain castle fallback.
- unmake-xor: restore normal-move piece and occupancy via XOR, restore
  captured piece without recomputing a discarded hash; exact Undo retained.

## Role lookup and output
- nibble-role: 32-byte packed role cache, occupancy provides color/presence.
- role-planes: three redundant bit planes (24 bytes) for branchless role
  decoding, independent of nibble cache.
- uci-sort-key: sort compact 15-bit lexical keys before allocating the
  same vector of UCI strings; assert identical normalized output on all
  21,002 benchmark positions and dedicated promotion/Chess960 fixtures.
- fen-rank: write occupied rank squares and gaps from bit scans, retain
  exact native FEN semantics; direct final byte copy.

## Prepared replay and residual ARM full-perft cases
- prepared-move: optional 4-byte position-specific token carries moved
  and captured codes for normal moves. Original 16-bit moves2 stays intact.
  Special moves dispatch to existing exact path; full cached state remains.
  Separate --prepared-replay benchmark includes preparation cost and
  matched per-ply Polyglot/checker observations, never replacing native
  16-bit replay. Shakmaty 0.30.0 native moves carry role/capture metadata
  and its native mutation does not maintain these cached values; no GPL
  source copied. This is an independently designed compact token.
- compact-attacks: combine raw role planes then mask color once.
- bool-attacks: early-return boolean king-safety scan; exclude captures.
- perft-xor: one role-mask update for ordinary slim moves.
- promotion-batch: construct first promotion child once and vary role,
  refresh exact checker cache for each observed leaf.
- leaf-reuse: test make/unmake versus copy-per-child in grouped visitor.

## Round 3: smaller hot-path and output experiments
C remains the direct paired benchmark baseline, E and D stay available as
immutable comparison configurations.
- inline-make: force-inline generic full transition to test elimination of
  call/Undo return overhead and better context specialization.
- split-quiet: inline a small quiet-only path with local hash accumulator,
  XOR occupancy and exact cached checker refresh; fallback to normal
  full-state transition on captures/specials.
- role-select: common pawn branch, branchless role encoding thereafter.
- attack-descriptor: co-locate each square's leaper and empty-ray masks.
- fast-castle-keys: standard 16-entry hash delta table, exact nonstandard
  fallback even during FEN construction.
- hash-role-scan: six role scans with color determined per set bit, not
  twelve color-role loops.
- fen-byte-copy: isolate final String construction from rank writer changes.
No legal-state assumptions are added; native replay, hash and checker
contracts remain unchanged.

## Round 3: alternate slider-plane storage and expanded correctness

Two independent layout candidates are based on E. In the first, diagonal
(bishop or queen) and orthogonal (rook or queen) planes overlap, so individual
roles are recovered by intersection/difference. Pawn, knight and king
retain direct planes; expected Board size 88 bytes. The second derives
king occupancy from the other planes and color occupancy (80 bytes).
Movegen and checker paths can access combined sliders directly. Hash,
clocks, castling rights, checker masks and the public move word stay intact.
Both variants passed the debug regression suite, including 960 starting
arrangements and their stripped castling fixtures, exact legal move sets,
depth-2 perft, checker parity, and reversible state.
Benchmark-only defaults now select E to isolate layout effects; root
production defaults remain unchanged. Supplemental per-ply replay hash
and hash-plus-checkers rows are excluded from the original strict score.

## Round 3 progress (2026-11-26)
E: run 45776626381. Three available jobs (native ARM64, Linux x64,
Windows x64) succeeded: 216/261 strict wins, 207 >=1.03x, 30 >=2x.
Remaining jobs are pending at this snapshot. E vs D on aiserver paired
ABBA targeted screen (same f6e0115 source): sorted UCI 1.590x,
checked replay 1.389x, child creation 1.364x, trusted replay 1.345x,
make/unmake 1.329x, full perft 1.313x; local host shared/noisy.
Prepared replay local first measurement: 36.8ns wire vs 19.0ns
prepared (preparation included 27.8ns); Shakmaty native 16.0ns.
Matched per-ply hash+checkers Giga 2.15x Shakmaty. Treat as preliminary
because timing variation is evident; not a substitute for native replay.
The full expanded E plus optional layout/prepared feature release suite
passed locally. All 960 start and 960 castling fixtures passed in debug
on E and both new slider layouts. Seven more independent hot-path
experiments are being screened locally. The first split-quiet and
branchless role-select results are not compelling; do not adopt yet.
The direct pre-mutation check alone (45775463989) finished with 368/522
wins; 0.882x checked replay vs C. The reused-role variant (45775774803)
finished with 377/522 wins and 1.034x checked replay vs C.
New E-paired layout runs 45778344181 and 45778347701 were dispatched;
no result claimed. Neither production defaults nor main changed.

## Round 4: E-selected baseline, exact remaining losses

All six E jobs finished successfully (45776626381): 432/522 strict wins,
412 >=1.03x, 63 >=2x. Sorted UCI 12/12; make/unmake 35/36; full
perft 32/36; children 22/36; snapshot 9/36; trusted replay 0/12.
The 88/80-byte overlapping slider layouts lost to E on mutation and
were rejected (394/522 and 407/522). Redundant role cache and branchless
role encoding also rejected. Independent FEN rank writer (45776160472)
won 36/36 with 1.474x vs C; numeric UCI sort (45776157007) won 12/12.
Retain 96-byte layout for F = E + exp-fen-rank.
Chess 3.2.0 copy-make uses a simple source/destination XOR plus ray-based
checkers and pins. Shakmaty 0.30.0 uses predecoded move roles and
direct plane mutation without cached Polyglot/checker upkeep. Ferrum
uses flagged moves and 88-byte boards. No code copied from GPL or
ambiguous-license sources. Existing 16-bit full-state replay retained.

New independent experiments:
- align16/align32: 96-byte layout aligned for copy operations, no field changes.
- ordinary-kernel: one occupancy classification, known capture role, single
  local hash accumulator, XOR piece and occupancy, special moves fallback.
- hash-pawn-bytes: rank-pattern XOR table for pawns (32 KiB), exact Polyglot.
- hash-delta: precomputed two-square key XOR (384 KiB), tests cache tradeoff.
- no-promotion-batch / no-perft-xor: independent ablations of E, target
  Windows and ARM residual full-perft losses.
- Existing leaf-reuse and inline-make evaluated separately.
Expanded E+all new compatible feature debug regression suite passed,
including all 960 starts and 960 stripped castling fixtures. CI required.

### Round 4 copy and move generation follow-up
- color-gen: use the const WHITE parameter for actual color indices.
- bulk-write: one checked capacity reservation and length update for each
  target group; promotion groups written together, exact ordering.
- pawn-kernel: specialize quiet nonpromoting pawn transitions before
  general role decoding, preserve full hash/clock/checker state.
- child-copy: public experimental full-state copy-make method, no Undo;
  same move inputs and exact output Board as existing child workload.
- checker-first: process first ray candidate separately with branchless
  mask, retain remaining candidates and exact checker semantics.
All five combined passed debug regression including real legal move
transitions, exact child equality and all Chess960 fixtures. F is the
benchmark-only default for the follow-up branch. Root defaults unchanged.
