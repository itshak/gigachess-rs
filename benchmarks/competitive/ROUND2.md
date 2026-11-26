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

### Round 4 Undo and hashing follow-up
- undo-outside: tiny transition result (capture/castle/moved metadata)
  with prior hash/checker/clocks saved in an inline wrapper. The optimizer
  can eliminate the prior-state snapshot when Undo is discarded. The
  public Undo layout and all restoration semantics are unchanged.
- hash-pointer: select the exact 64-key Polyglot slice once per color/role.
- hash-pair: scan two occupied squares with independent XOR accumulators.
- fen-code-lut: direct byte-to-piece-code table, preserving error validation.
Passed the expanded debug regression suite with optional prepared moves,
ordinary/pawn fast paths, copy-make and explicit Undo role enabled.

### Cold start and blocked positions
The original 522 strict cases exclude cold first-use. E lost the first-use
median on all six platforms (ratio 0.649–0.929 vs fastest peer). Static tables
already eliminated runtime initialization; remaining cost includes code and
data demand paging. Two general optimizations, not startpos special cases:
blocked-sliders returns the adjacent attack mask when every adjacent ray
square is occupied; no-danger-needed skips enemy attack-map construction
when no king target or standard castle can consume it. Nonstandard castle
checks remain independent. Expanded debug regression passed.

### Round 4 evidence checkpoint
F (E + rank FEN writer, run 45783634997): four Linux jobs available at
this snapshot, 303/348 wins, 294 >=1.03x, 39 >=2x. Same-platform E 288/348,
D 276/348. FEN writer 1.489x vs paired E, 1.674x vs fastest peer, 24/24.
All remaining jobs pending at this evidence checkpoint. Local G combined
eight experimental flags passed complete release tests and all six ignored
deep perft tests; no timing claim until ABBA completes.
Strict gate now has --require-complete, --cold, --require-wins. It refuses
to treat partial 3/4-platform artifacts as a completed universal result;
cold medians are tracked separately. The release gate is not satisfied.

## Round 5: replay, board copy, and state update refinement
Completed F 446/522; E+align16 465/522; E+align32 460/522.
F and aligned models were independent, so test F+align16 and F+align32
without claiming combined results in advance. Board copying still loses
on ARM and macOS; trusted native replay loses all 12 cases to Shakmaty.
Its decoded moves avoid source role lookup and its native transition
does not update both caches. Cozy/Jordan use XOR updates and sparse
checker rays; Ultra uses specialized moves and mailbox lookup. No GPL
source copied, no wire changes or weaker native output.

New independent flags:
- sparse-role3: exact 72-byte board, pawn has role code zero (occupancy
  distinguishes empty), knight/bishop/rook have a single bit-plane, queen
  and king two. Direct diagonal/orthogonal masks. This is a new encoding,
  not redundant role storage. Compare branchy and branchless mutations.
- direct-role-xor: one role plane XOR in the existing quiet path.
- castle-compact: known standard rights hash lookup, skip irrelevant
  rights processing off back ranks. Chess960 detection moved before
  initial hash computation; all-960 regression caught the initial error.
- checker-pair: two branchless ray candidates before arbitrary fallback.
- compact-attack-desc: colocated attack table active in E/F compact path
  (previous attack-descriptor was masked by the earlier return).
All-960, real games, exhaustive sliders, FEN, SAN and rollback gates
passed for sparse and branchless configurations. Keep raw native rows.

### Round 5 role lookup and replay alternatives
- role-tree: grouped role lookup, adapted from Jordan Bray chess 3.2.0
  (MIT, board.rs piece_on). Use in source lookup, captures and FEN.
- role-simd: pawn fast branch, four remaining non-king planes tested
  together via AVX2 or NEON; scalar tree on portable x64. AVX2 compile
  checked locally; architecture-specific execution requires CI.
- capture-direct: exact ordinary capture with known role/color and two
  XOR mutations, no redundant piece decoding. Special cases unchanged.
- checked-cached: conservative off-ray nonking shortcut, explicitly
  confirm old king safety because cached checkers may be stale after
  documented hash-only operations; fallback for EP, king and ray moves.
- quiet-common: simple raw-word test for normal nonpromotion moves;
  fallback preserves invalid/reserved promotion-word behavior.
Combined expanded debug regression passed. Five F-paired CI experiments.

### Round 5 evidence and combined configurations
Six complete round4 runs archived. F 446/522; E+align16 465/522;
E+align32 460/522; E+ordinary-kernel 452/522; F+color-gen 463/522;
F+blocked-sliders 449/522 and all six cold first-use wins
(1.085–1.632x against fastest peer). No claim of universal win.
Three further combinations dispatched on role branch: F+align16+blocked,
F+align32+blocked, F+align16+blocked+color-gen. All three flags together
passed expanded local regression. 16 total new CI runs submitted in round 5.
Sparse layout is exactly 72 bytes unaligned, 80 aligned to 16, 96 aligned
to 32. Native SIMD needs actual cross-platform execution; scalar fallback
tested, AVX2 compile checked. Existing F baseline is unchanged apart from
initial hash computation order (after Chess960 detection).

## Round 6: Undo-outside, cold-start sweep, and refined compact board
Round5 all 16 passed. F+undo-outside (45784348835) 477/522,
including 2/12 trusted replay, 30/36 children, 27/36 snapshots.
F+align32 470/522; F+capture-direct 469/522; castle-compact 12/12
recomputed hashes; compact-attack-desc 11/12 checked replay.
F+align32+blocked 461/522 plus 6/6 cold first-use wins.
Sparse 72-byte layout 34/36 snapshot but trusted replay 0.807x F;
branchless 35/36 snapshots but more regressions. SIMD and role-tree
not selected (387 and 440 total). No-danger-needed 464/522,
4/6 cold. Inline-make 34/36 children, 1.114x E trusted replay.
Two incomplete old runs cancelled on macOS, retried failed jobs.

New variants:
- pawnless4: 80-byte board; four role planes N, B|Q, R|Q, K, pawn
  is occupied complement. Preserves malformed FEN king counting.
  No pawn-plane writes, no redundant cache; exact hash and checkers.
- zone-sliders: reverse empty rays from king-neighborhood and both
  standard castling transit paths to prefilter enemy danger-map sliders.
  Pin/check geometry stays exact, other castle paths probe independently.
- ep-mask: direct zero-mask entry for absent EP, keep native EP square
  and Polyglot geometric capturer semantics.
- special-cold: compact normal move transition returning minimal
  capture/castle metadata, rare cases call noninlined general path.
  Independent of prepared input; unchanged moves2 and Undo contract.
All four passed combined debug regression (nine tests including size,
960 and stripped castle fixtures, real games, exhaustive blocker tests).
F+undo-outside is the new benchmark-only paired baseline.

## Round 7: exact checker and common transition refinements
Round6 all 10 passed. Best combined cold sweep I5 473/522 + 6/6 cold,
I4 462/522 + 6/6 cold and all 36/36 child creation. I3 12/12 hash
recomputation, I4 native trusted replay 0.787x fastest peer (1.127x I).
Special-cold 470/522, 1.077x trusted and 1.117x checked vs I; local
1.105x and 1.127x. Pawnless4 regresses mutations despite snapshot wins.
Both cancelled macOS jobs retried successfully; checker-first and
hash-pair not retained. Keep previous I 477/522 record separate.
New common combination J = I+special-cold+align32+blocked-sliders.
Compare J against I, plus variants with inline and danger filtering;
use paired J baseline on separate feature experiments for attribution.
New experiments:
- checker-sentinel: first candidate ray uses zero-safe sentinel at 64,
  exact lowbit mask; fallback scans arbitrary additional candidates.
- refresh-inline: inline exact full checker refresh.
- attack-mask: apply color/removal occupancy once across role masks
  in boolean legality checks, fallback on encoded layouts.
- castle-pair: compile-time from/to clear mask in one lookup.
- hash-turn-fold: pre-XOR turn key into first piece key in full delta.
- quiet-ep-gate: only query new EP relevance for actual double pawn push.
- promo-checkers: four promotion leaves have identical occupancy and
  non-promoted attackers; reuse exact base checkers and evaluate only
  promoted destination's bishop/rook rays. Knight child is unchanged.
Expanded regression passed including promotion leaf equality.

## Round 8: specialize rights-free transitions and improve board locality
All eleven round7 CI runs passed and all six cold medians were won
by each candidate. J-inline (45816428181) 467/522 and 5/12 trusted
replay, 11/12 checked replay, 35/36 children, 1.146x trusted vs I.
J-refresh-inline 472/522; J-quiet-ep-gate 470/522.
Best overall remains I 477/522, best native+cold I5 473/522 + 6/6.
Round7 local full release and six ignored deep perft passed; focused
local ABBA stopped during promotion variant, use complete CI for it.
New benchmark paired baseline K = J-inline. New experiments:
- no-rights-kernel: normal full transition when old castling rights and
  old en-passant are absent, retaining new double-push EP, clocks, all
  hashes/checkers, Undo and rare-move fallback.
- promotion-direct: direct known pawn promotion in slim perft, retaining
  captures, rights, occupancy, cached checkers and exact leaf observation.
- layout-hot and layout-roles: reorder fields only, no state removed.
- hash-role-order: independently implement role-major cache ordering
  (Shakmaty benchmark observation), two accumulators, same Polyglot keys.
  No GPL source copied.
- ray-combined: combine raw diagonal/orthogonal role masks before the
  color mask; no weakening of checker semantics.
- unalign: separate 8-byte natural alignment comparison.
- kingless-ray+unalign: 88-byte board, king mask derived exactly from
  occupied complement; avoid expensive complement when king attack
  candidates are empty; malformed FEN validation unchanged.
Combined debug regression passed for kernel/layout-hot/hash/rays, and
separately for layout-roles/kingless/unalign, plus explicit size test.
