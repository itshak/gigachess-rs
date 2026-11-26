| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.996x | 1.487x |
| materialize | 0.945x | 0.902x | 1.491x |
| perft_bulk_d4 | 1.001x | 0.998x | 1.257x |
| perft_full_d3 | 0.989x | 0.967x | 1.494x |
| fen_parse | 0.991x | 0.980x | 0.988x |
| fen_write_native_ep | 0.989x | 0.959x | 2.142x |
| snapshot | 1.092x | 1.086x | 0.994x |
| san_parse_all | 0.996x | 0.986x | 1.863x |
| san_render_native | 0.982x | 0.970x | 2.116x |
| apply_children_full | 0.977x | 0.956x | 1.079x |
| make_unmake_full | 1.002x | 0.992x | 1.144x |
| startpos | 0.891x | 0.891x | 1.077x |
| chess960_materialize | 0.855x | 0.854x | 1.279x |
| chess960_perft_d3 | 1.003x | 1.003x | 1.392x |
| polyglot_key | 1.024x | 0.998x | 39.306x |
| polyglot_recompute | 0.911x | 0.873x | 0.913x |
| repetition_key | 1.037x | 1.023x | 19.417x |
| corpus_count | 0.999x | 0.993x | 1.403x |
| corpus_materialize | 0.996x | 0.995x | 1.119x |
| corpus_uci_sorted | 1.002x | 0.991x | 1.588x |
| replay_trusted_native | 1.001x | 0.995x | 0.715x |
| replay_checked_native | 1.010x | 1.002x | 1.051x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.95x).
