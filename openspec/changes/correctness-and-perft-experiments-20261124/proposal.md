# Correctness closeout and perft workspace experiments

Finish the previously committed independent JS/Rust correctness fixes
before performance experiments. Both draft PRs include the earlier FIDE
repetition, EP, Polyglot, FEN, Chess960 and moves2 interoperability fixes.
They remain unmerged.

JavaScript additionally fixes shared mutable castling/check context scratch:
earlier analyses could be overwritten by later analysis in Chess960.
Measure per-depth move buffer/context reuse and enemy-occupancy pin filtering
separately and together, plus the competitor's specialized perft hash-skip.

Rust already had split-pin generation, cached checkers, perft-only hash
skipping and MaybeUninit move-list storage. Investigate just the incremental
per-depth workspace change with the established Board::perft left intact.

Correctness gates SHALL pass prior to benchmarking. Run on aiserver only;
no Github CI, no WASM integration, no production external dependencies.
