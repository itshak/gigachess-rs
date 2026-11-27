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

## Round 9: 88-byte board with six direct role bitboards
Round8 eight complete runs retrieved so far. New best K+layout-roles
(45824543156) 479/522, 457 >=1.03, 71 >=2, cold 6/6. K+zone-sliders
+no-danger-needed 477/522, cold 6/6 and 12/12 checked replay; K+castle
compact+promo-checkers+attack-mask also 12/12 checked replay.
K+layout-hot 471/522, trusted replay 5/12 and 0.841x fastest peer.
Rights-free kernel 464/522; local 1.126x trusted replay but CI 0.982x
against paired K, so not selected solely on local measurements.
K+role-order 459/522, recompute 0.954x K despite local 1.259x;
defer. Remaining three round8 runs not yet complete at this checkpoint.
Round8 full local release, six deep-perft and all four focused ABBA
screens completed. New paired baseline L=K+layout-roles.
- packed-meta: six ordinary role planes, derive king square from actual
  king bitboard, retain four exact castle backing squares including
  absent black-square sentinel as four nibbles; rights/turn/variant
  share one byte, preserve EP and two u16 clocks. Natural alignment gives
  verified 88 bytes. Malformed FEN still rejected; duplicate king
  diagnostics may differ before final validation. No new unsafe code.
- checker-hybrid: switch to attack tables when >2 ray candidates;
  fallback exact between-ray scan for sparse candidates.
- checker-color: exact full checker refresh specialized by mover color,
  independent of prior reachability and stale caches.
- direct-key: safe 12x64 Polyglot key rows, same numerical values.
- castle-file-gate: early return if no king move or standard corner/home
  square touched; Chess960 retains exact existing path.
Combined local regression passed, 88-byte size/startpos/empty verified,
default library 55 tests passed. Additional local full release tests
and six-platform comparisons required before selecting.

## Round 10: direct role layout and exact leaf transitions
Round9 nine complete runs retrieved. M=L+no-danger-needed+zone-sliders
(45831167420) new best 482/522, 468 >=1.03, 77 >=2, cold 6/6,
all 36/36 child creation, count, FEN parse and make/unmake.
Direct-key 467/522, 12/12 checked replay, 1.085x trusted vs L.
Color-specific refresh 476/522, 1.074x checked vs L.
Packed meta rejected: unaligned 434/522, aligned 462/522, local
trusted 0.673x L despite 1.161x snapshot. Hybrid checker rejected.
Round9 full local release + six deep perft + four focused ABBA passed.
Round8 cancelled macOS jobs now both passed; kingless-ray 441/522,
unaligned 475/522; K1 468/522. Local-only improvements do not justify
promotion without cross-platform confirmation.
New benchmark baseline M. New five experimental techniques:
- implicit-pawn: five direct K,N,B,R,Q planes, pawn from occupied
  complement; no pawn role writes, cached king squares retained.
  Natural alignment 88 bytes, separate alignment alternative.
- role-weight: pawn fast path, weighted role mask lookup for other
  roles instead of multiple conditional branches.
- leaf-direct: exact ordinary slim child transition without Undo
  construction, retains hash/clocks unchanged, EP/castle fallback.
- color-pawn: specialize pawn-side index at compile time.
- hash-square-order: independently generated square-major Polyglot
  key layout; exact same keys for public and internal accessors.
Expanded local regression passed for four combined features and
separate role-weight variant; full release validation pending.

## Round 11: reuse source removal and checker geometry across leaf groups
All ten round10 runs passed, cold 6/6 each. M+leaf-direct (45839690097)
480/522, 470 >=1.03, all 36/36 full perft (1.220x vs M), all 12/12
checked replay. M remains 482/522 record; N=M+leaf-direct is selected
as paired baseline because the improvement is targeted and repeatable
(local full perft 1.115x, min 1.039x, CI 1.220x). Implicit pawn
421/522 unaligned, 435 aligned, role-weight 461, square-order hash
451 with recompute 0.684x M: rejected. Round10 local full release,
six deep perft, four focused screens complete.
New techniques:
- leaf-base: share nonpawn source removal, EP clearing, turn change,
  copy that base for each destination. Castling retains existing path.
- leaf-check-delta: compute exact checker mask after source removal
  once, clear old ray checkers blocked by the destination, add direct
  destination checker using role-specific reverse geometry. Capture
  leaves destination occupancy unchanged. No assumption of reachable
  FEN or preexisting opposite check. No skipped observable state.
- implicit-queen: five direct P,N,B,R,K planes, queen from occupied
  complement. 88-byte natural alignment with cached king square.
- align16-override: compare 16 vs 32-byte alignment on N, same size.
- role-key-fold: interleaved role/color rows with and without turn
  key, identical Polyglot delta semantics, no new unsafe.
Combined debug gate passed including real corpus, all 960 and full
leaf parity. Extra checker-delta differential test covers 2708
sampled positions from 18,954 real-game plies, plus permissive FEN,
castling, promotion and EP. Explicit 88-byte assertion passed.

## Round 12: targeted trusted replay
Round11 local full release + all six deep perft + four ABBA screens
passed. First complete CI variant N+leaf-base 45849996242:
477/522, 463 >=1.03, 71 >=2, cold 6/6, full perft 36/36 and
1.076x paired N (local 1.183x). Nine other round11 runs were still
pending at checkpoint. Keep N as paired baseline until their results
are complete. No selection based solely on local gains.
Four new isolated techniques:
- pawn-transition: specialize ordinary pawn state transition with
  known role, exact capture, hash, rights, EP, clocks and checkers;
  promotions and EP captures retain existing fallback.
- pawn-key-delta: 4 KiB table of valid pawn displacement key XORs;
  retains fallback for other moves and original Polyglot constants.
- single-color-transition: compare dynamic mover-side transition
  against two color-specialized instantiations; no semantic change.
- equal-capture: compact six-plane storage only, same-role capture
  removes source from shared role plane and leaves destination set,
  while both color occupancies and capture/hash remain exact.
Combined targeted regression (real games, SAN/undo, all 960, full
leaf and slider parity) passed. Full release and deep perft pending.
Limit CI to four isolated runs while round11 queue is outstanding.

## Round 13: combine validated perft and replay gains
Round11 9/10 complete successfully; implicit-queen unaligned portable
runner failed at paired build after correctness, restarted failed job.
N+refresh-inline+quiet-ep-gate (45850040672) new overall best 484/522,
477 >=1.03, 75 >=2, all cold, 12/12 checked replay, 1.053x trusted vs N.
N+align16-override also 484 but 0.840x snapshot vs N; not selected.
N+leaf-check-delta (45850001441) 482/522, full perft 1.194x paired N,
36/36, 1.225x worst vs fastest peer. N+direct-key+checker-color+
quiet-ep-gate 479/522, 6/12 trusted, 12/12 checked, 36/36 full perft.
Round12 all four CI and full release/deep perft/ABBA passed:
equal-capture 482/522, pawn-transition 480 (1.051x trusted vs N,
5/12 wins), pawn-key-delta 475, single-color 464. Neither latter
is selected. New paired baseline O=N+refresh-inline+quiet-ep-gate.
New isolated techniques: hash-two-lanes (paired independent black and
white accumulators with constant role indexing); checker-near (gate
nearby attacks by occupancy before role tests); castle-standard-clear
(gate rights updates by affected standard corner/home squares);
role-test-bit (branchless bit-test role lookup after pawn fast path).
Combined exact regression passed including 2708-position differential.
Additional combinations O+leaf-check-delta, O+pawn-transition,
O+leaf-check-delta+direct-key+castle-compact, and combined perft/replay
variant; compare actual measurements rather than count alone.

## Round 14: pre-mutation checkers and filtered geometry
Round13 all 8 CI, full local release, six deep perft and local
ABBA screens passed. Round11 portable retry passed. New score record
492/522 (480 >=1.03, 67 >=2, cold 6/6) O+checker-near 45872840899.
However its paired trusted ratio is 0.966 and local 0.822, so keep O
as paired baseline rather than selecting by peer score alone.
O+castle-standard-clear 489/522, trusted 0.932; O+leaf-check-delta
480/522, full perft 1.220x O; O+leaf-check-delta+direct-key+
castle-compact 484/522, full 36/36, recompute 1.108x O. Two-lane
hash rejected (recompute 0.850 CI / 0.732 local).
New techniques: checker-predict calculates exact future checkers
from unchanged role planes, known source/destination and future
occupancy before mutation. Does not assume legal reachability.
geometry-union scans merged sniper candidates, assigns pinned
pieces to orthogonal/diagonal classes. zone-knights filters enemy
knights by the reverse attacks of every relevant king/castle step.
Combined regression and permissive-position checker-cache comparison
passed. Repeat O+checker-near as control; combine the new geometry
flags and the successful full leaf delta. O remains paired baseline.

## Round 15: compact 80-byte board and selective hash/attack work
Round14 all six CI, full local release and six deep perft passed.
Repeat P 45884473516 scored 484/522, paired native replay 0.911x O;
492/522 record remains a single observed maximum, not a stable gain.
O+geometry-union+zone-knights+leaf-check-delta 486/522,
full perft 1.213x O, checked 1.050x, trusted 1.007x.
O+checker-near+leaf-check-delta+direct-key+castle-compact 479/522,
12/12 hash recompute, but trusted 0.878x and checked 0.924x.
Zone-knights 484/522, corpus count 1.052x, materialize 1.053x,
trusted 1.044x. Predict-checkers 473, trusted 0.893x; reject.
Three new techniques: compact80 combines packed metadata with five
direct role planes, retaining all logical fields and deriving king
square from occupancy (80 bytes, verified); hash-small-role folds
two sentinel lookups per nonpawn role/color, handles extra promoted
pieces with exact loop; targeted-danger uses reverse attack queries
for <=2 relevant standard king/castling destinations.
Combined targeted tests pass, including 80-byte layout, clock extremes,
all 960 starts, exact perft leaf state and real-game replay.
Keep O paired baseline; test separate flags and selected combinations.

## Round 16: 80-byte snapshot win, preserve execution performance
Round15 all five CI and full local release/six deep perft/ABBA passed.
Compact80 45889555294: 474/522, snapshots 36/36, min1.051x fastest
peer, but child creation 0.885x O and full perft 0.850x O.
Hash-small-role 482/522, recompute 1.013x O, local 1.141x;
combined zone-knights/leaf-check-delta/hash-small-role 484/522,
full perft 1.201x O, 36/36, recompute 1.095x O. Targeted danger
477/522, corpus count 0.884x O; reject. All cold 6/6.
New 80-byte layout compact80-sliders: direct P,N,K plus overlapping
diagonal/orthogonal planes, packed metadata, no stored king-square
redundancy. King-square lookup scans a single direct role plane,
rather than complement of five. compact-slider-xor uses constant
plane indices and extra XOR for queens. king-pair tests split
complement with CTZ sentinel mapped to zero. hash-king-single
avoids second sentinel load for king, retains extra pieces fallback.
Both compact layouts pass targeted regression incl exact sizes,
960 and full leaf/undo state. O remains paired baseline.

## Round 17: cached 88-byte compromise and pawn count batching
Round16 all five CI, full local release, six deep perft and local
ABBA screens passed. Compact80-sliders 449/522, slider XOR 430,
combined slider/leaf 427. Compact80+king-pair 478; full perft
0.865x O, snapshots 32/36. Reject these as overall candidates.
Hash-small-role+king-single 483/522, 478 >=1.03, cold 6/6,
recompute 1.085x O, checked 12/12. Local recompute 0.954x O;
needs further replication. Best observed record remains 492, repeat 484.
New cached-packed-king keeps both king coordinates with packed
metadata and five direct role planes, exact 88-byte board. An 80-byte
layout preserving this extra state would require additional compression;
do not sacrifice clock/rook-square semantics. Pawn-double-reuse
reuses the unrestricted single push mask for intermediate squares,
never the check-filtered mask. Pawn-batch-count preserves default
callback ordering and specializes MoveCounter to sum ordinary
targets plus three extra moves per promotion. Combined 11 exact
regressions passed including all 960 starts, real-game legal/SAN/undo,
exact leaf differential and the 88-byte assertion.

## Round 18: count-mask union and compact checker/pin table
Round17 all five CI, full release, six deep perft and local ABBA passed.
O+pawn-batch-count 486/522, corpus count 1.154x O, 12/12 wins;
pawn-double-reuse 483/522, count 1.025x O, corpus count 1.030x.
Combined pawn/zone/leaf variant 482/522, full perft 1.208x O and
trusted replay 6/12. Cached 88-byte layouts 461 and 463; reject.
Retain O and its 96-byte board as the paired baseline.
New pawn-count-union exploits disjoint legal push and occupied capture
destinations, retaining distinct left/right capture multiplicities.
Pawn-count-promo-gate skips the extra promotion counts if none exist.
Between-compact replaces selected dense 32 KiB table reads with
per-source byte indices and 28-entry rows (18 KiB combined); exact
all 4096 entries verified. Extra regression checks promotion/capture
overlap and count/materialize equivalence. Combined targeted nine
regression tests plus exhaustive table equality passed.
Submit isolated count changes, compact table, combined leaf/count,
and O-versus-O identical-code ABBA control to quantify runner noise.
No production changes; 492/522 remains an observed maximum only.

## Round 19: reuse transition occupancy and hash rows
Round18 all six CI, full release and six deep perft/local ABBA passed.
Identical O/O control 45909213539 scored 465/522 native, cold 6/6;
129/522 paired Giga rows outside reciprocal 3% bounds and 62/522
outside reciprocal 10% bounds. Mac 79/87 outside 3% (0.602–1.811x);
Windows 28, PEXT 10, portable 8, native 3, ARM 1. Do not interpret
a single win count or 3% margin as statistical significance.
O+pawn-count-promo-gate 487/522, corpus count 1.118x O, 12/12.
Union+gate 486/522, corpus 1.117x O, count 1.043x, 36/36.
Combined pawn/zone/leaf 481, corpus count 1.100x O, full perft
1.186x, both all wins. Compact between 474, no meaningful paired
improvement. Local promo-gate speedups in unrelated operations
are confounded by machine load/code layout; not causal evidence.
New transition-known-occ computes exact future own/union occupancy
once and supplies it to an inline checker calculation after mutation.
Checker-row tests a per-color descriptor for this exact kernel.
Move-row-key reuses one bounds-checked Polyglot role/color row for
both square keys; no unchecked indexing or changed move encoding.
Nine combined targeted regression tests passed.

## Round 20: active rights, outlined capture, low-bit checker loop
Round19 all five CI, full local release, six deep perft and local
ABBA passed. Known occupancy 485/522, child creation 36/36;
checker row 480, trusted 1.031x O; shared move hash row 484,
trusted 1.028x O; paired occupancy/hash 483, trusted 1.004x O.
Combined pawn/leaf/replay 486, full perft 1.208x O (36/36),
corpus count 1.103x O. No convincing trusted replay breakthrough.
Local combined replay 1.269x O but unrelated hash/corpus changes
were similarly large: do not attribute these to replay code.
New active-castle-touch tests only squares affecting existing rights,
retaining permissive-position clearing semantics and Chess960 fallback.
Capture-outlined moves the ordinary capture mutation/key into an
out-of-line helper, preserving the inline quiet path. Checker-lowbit
uses the already isolated candidate bit and a branchless clear mask.
Nine combined exact regression tests passed.
Compare all three isolates, combined replay, combined replay/pawn/leaf,
and repeat the pawn/leaf configuration without new replay features.
O remains paired baseline, production unchanged.

## Round 21: replicate pawn/leaf gains, exact final-board bulk replay
Round20 all six CI, full release, six deep perft and local ABBA passed.
Repeated O+pawn-count-union+promo-gate+double-reuse+zone-knights+
leaf-check-delta (Q) 475/522, full perft 1.214x O (36/36),
corpus count 1.112x (12/12), count 1.067x (36/36). Prior round18
same flags 481, full 1.186x, corpus count 1.100x, count 1.056x.
These operation-specific improvements replicated despite noisy total
wins. Keep Q as leading targeted candidate, O as paired reference.
Active rights 486, trusted 0.953x O. Capture outline 480,
trusted 0.985x. Checker lowbit 484, trusted 1.027x. Combined
replay 485, trusted 0.944x. Reject outline/rights for overall promotion.
New checker-magic replaces the candidate BETWEEN loop with exact
conditional bishop/rook attack lookups. Raw-promotion-gate tests
the nibble with a range test, preserving all existing decode semantics.
New experimental bulk APIs return a full final board with either
incremental hash or final hash recomputation, defer checker refresh,
and never expose intermediate boards. Input unchanged, exact clocks,
null passes and special moves preserved. All 18954 real-game prefixes
and additional promotion, en-passant, null and Chess960 cases checked
against repeated full makes. Combined ten targeted tests passed.
Separate --batch-replay benchmark observes final board, Polyglot and
checkers for all competitors; never substitutes for strict native
single-move replay. Add conditional CI evidence lane.

## Round 22: 493-win record and direct batch mutation
Round21 all five CI, full local release, six deep perft and local
ABBA passed. Q+raw-promotion-gate (R) 493/522, 478 >=1.03,
79 >=2x, cold 5/6, materialize/count/full perft 36/36 each,
corpus count/checked replay 12/12 each. Full perft 1.199x O,
corpus count 1.109x, trusted 1.006x. Record is observational;
repeat R. Q+batch-replay 484/522, full perft 1.206x O,
corpus count 1.132x. Checker magic 480, combined replay 485,
raw promotion alone 481. No strict native replay breakthrough.
Separate final-output batch lane 45997638603: recompute vs own
full loop Mac 1.13x, ARM 1.34x, native 1.37x, PEXT 1.36x,
portable 1.29x, Win 1.59x. Shakmaty still faster on five lanes;
Windows noisy, local batch 1.295x own, 0.987x Shakmaty.
New batch-direct bypasses all piece key loads and undo snapshots
on ordinary moves, preserving full clocks/rights/EP/king squares.
Specials reuse canonical transitions. Batch-pair specializes
alternating colors in pairs (null also flips side). Final hash
and exact checkers rebuilt, empty sequence returns original.
Castle-delta clears only changed rights and hashes the removed
mask; no change to castling rules. Ten combined targeted regressions
passed including every real-game prefix and special cases.
Repeat R, isolate castle delta, combine R+delta, and separately
measure direct and paired batch kernels using identical final-output
lane. Keep original strict 522 native comparison unchanged.
