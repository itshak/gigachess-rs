| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.993x | 1.405x |
| materialize | 0.999x | 0.993x | 1.496x |
| perft_bulk_d4 | 0.993x | 0.984x | 1.205x |
| perft_full_d3 | 1.011x | 0.999x | 1.507x |
| fen_parse | 1.102x | 1.067x | 1.121x |
| fen_write_native_ep | 0.964x | 0.945x | 1.286x |
| snapshot | 0.713x | 0.702x | 1.157x |
| san_parse_all | 1.021x | 0.999x | 1.939x |
| san_render_native | 1.009x | 1.001x | 2.141x |
| apply_children_full | 0.953x | 0.933x | 1.106x |
| make_unmake_full | 0.951x | 0.925x | 1.090x |
| startpos | 0.820x | 0.820x | 1.296x |
| chess960_materialize | 0.994x | 0.988x | 1.468x |
| chess960_perft_d3 | 0.998x | 0.998x | 1.388x |
| polyglot_key | 0.961x | 0.924x | 38.229x |
| polyglot_recompute | 1.047x | 1.039x | 1.006x |
| repetition_key | 0.972x | 0.963x | 19.060x |
| corpus_count | 0.999x | 0.993x | 1.330x |
| corpus_materialize | 1.012x | 1.010x | 1.097x |
| corpus_uci_sorted | 0.998x | 0.996x | 1.582x |
| replay_trusted_native | 1.002x | 0.995x | 0.708x |
| replay_checked_native | 1.015x | 1.015x | 1.139x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.84x).
