| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.993x | 0.940x | 1.172x |
| materialize | 1.010x | 0.996x | 1.247x |
| perft_bulk_d4 | 0.969x | 0.912x | 1.095x |
| perft_full_d3 | 0.932x | 0.913x | 0.672x |
| fen_parse | 0.909x | 0.886x | 1.182x |
| fen_write_native_ep | 1.002x | 0.992x | 1.202x |
| snapshot | 0.750x | 0.749x | 0.746x |
| san_parse_all | 0.897x | 0.861x | 1.774x |
| san_render_native | 0.888x | 0.838x | 2.032x |
| apply_children_full | 0.840x | 0.793x | 0.721x |
| make_unmake_full | 0.763x | 0.706x | 0.785x |
| startpos | 0.738x | 0.738x | 0.969x |
| chess960_materialize | 1.016x | 1.014x | 1.401x |
| chess960_perft_d3 | 0.932x | 0.923x | 1.273x |
| polyglot_key | 1.038x | 0.999x | 22.589x |
| polyglot_recompute | 0.995x | 0.994x | 0.971x |
| repetition_key | 0.833x | 0.730x | 16.822x |
| corpus_count | 0.995x | 0.993x | 1.188x |
| corpus_materialize | 0.983x | 0.974x | 1.279x |
| corpus_uci_sorted | 0.985x | 0.985x | 0.970x |
| replay_trusted_native | 0.938x | 0.780x | 0.516x |
| replay_checked_native | 0.869x | 0.721x | 0.835x |

Cold first legal move list: 0.012 ms -> 0.012 ms (1.00x).
