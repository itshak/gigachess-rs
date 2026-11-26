| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.032x | 0.978x | 1.212x |
| materialize | 1.014x | 0.976x | 1.591x |
| perft_bulk_d4 | 0.923x | 0.888x | 1.068x |
| perft_full_d3 | 0.962x | 0.913x | 1.448x |
| fen_parse | 1.034x | 1.016x | 1.215x |
| fen_write_native_ep | 0.992x | 0.976x | 1.934x |
| snapshot | 1.120x | 1.111x | 1.002x |
| san_parse_all | 0.919x | 0.863x | 1.748x |
| san_render_native | 0.970x | 0.933x | 2.093x |
| apply_children_full | 1.004x | 1.000x | 1.217x |
| make_unmake_full | 1.007x | 0.966x | 1.192x |
| startpos | 1.002x | 1.002x | 1.430x |
| chess960_materialize | 1.046x | 1.022x | 1.364x |
| chess960_perft_d3 | 1.046x | 0.980x | 1.392x |
| polyglot_key | 0.950x | 0.901x | 41.467x |
| polyglot_recompute | 1.026x | 1.010x | 1.092x |
| repetition_key | 0.716x | 0.711x | 17.374x |
| corpus_count | 0.782x | 0.739x | 0.965x |
| corpus_materialize | 0.928x | 0.909x | 1.082x |
| corpus_uci_sorted | 0.981x | 0.980x | 1.515x |
| replay_trusted_native | 1.019x | 1.018x | 0.833x |
| replay_checked_native | 0.994x | 0.969x | 1.132x |

Cold first legal move list: 0.009 ms -> 0.005 ms (1.65x).
