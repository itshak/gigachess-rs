# Optimization bakeoff — 2026-11-26

## Reproducibility and status

All 26 independent experiments (52 Linux x64/ARM64 jobs) completed. Features are independent switches, not release defaults. Each job compiles baseline and candidate on the same machine, executes ABBA, validates legal positions and runs release tests. Raw repeated TSVs, environment, validations and per-peer results are preserved under [results/optimization-bakeoff-20261126](results/optimization-bakeoff-20261126). Ratios below 1 mean slower; a ~3% difference is not decisive. Per-operation geometric means are not per-position wins.

| Run | Scope |
|---|---|
| [45765368559](https://github.com/itshak/gigachess-rs/actions/runs/45765368559) | 11 first experiments |
| [45765584819](https://github.com/itshak/gigachess-rs/actions/runs/45765584819) | 6 layout and legality experiments |
| [45765771008](https://github.com/itshak/gigachess-rs/actions/runs/45765771008) | 5 kernel experiments |
| [45766229761](https://github.com/itshak/gigachess-rs/actions/runs/45766229761) | 4 geometry/UCI experiments |
| [45766467103](https://github.com/itshak/gigachess-rs/actions/runs/45766467103) | initial seven-feature combination, six platforms |
| [45767240593](https://github.com/itshak/gigachess-rs/actions/runs/45767240593) | twelve-feature combination, six platforms |
| [45767325283](https://github.com/itshak/gigachess-rs/actions/runs/45767325283) | fifteen-feature combination, six platforms |

The initial seven-feature matrix generated all six performance summaries; five jobs succeeded and Windows reported a failure in the full ABBA step despite producing its summary artifacts. Treat Windows data as provisional. The latter two combined runs are in progress at this snapshot. No selection or release claim is made.

## Interpretation

The strongest independent improvements are static slider tables (cold initialization), direct hypothetical legality and SAN suffix shortcuts, direct UCI output, fused geometry, compact board (144 to 96 bytes), copy-perft and specialized quiet replay. Keep rejected experiments off: mailbox, branchless role lookup, king-targets and king-local. Direct hash, standard-movegen and standard-no-special offered negligible gains.

Initial seven-feature combination across six runners: bulk perft 1.07–1.26x own baseline and 1.03–1.23x fastest measured peer; SAN parse 1.35–1.59x own baseline; SAN render 1.29–1.58x own baseline. Full perft, child application, sorted UCI, snapshots and replay remain competitive deficits. Full perft harness now observes every leaf, so earlier unobserved-leaf timings are not comparable. Static table generic x86 runtime PEXT dispatch must be checked before release; native PEXT is separately covered.

Extra real-game correctness gate passed locally for a fourteen-feature combination: 18,954 positions, 554,971 legal move SAN/undo transitions, all 65,536 encoded moves, exhaustive relevant slider blockers, Chess960 special moves. The stronger fifteen-feature combination has its own CI gate and passed the local full release suite plus all six ignored deep perft cases (including startpos d6 and Kiwipete d5). An earlier local timing screen was invalidated because the source changed during the sequence; none of its timings is used.

## Remaining acceptance gates

Collect the two pending combination runs, compare each platform and case against every supported competitor, investigate regressions, and repeat the selected configuration. Measure allocation counts separately from inline Board size. Keep experiments and evidence on draft branches; do not merge or release until verified.
