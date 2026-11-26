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
