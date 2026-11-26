| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.291x |
| materialize | 1.000x | 0.996x | 1.200x |
| perft_bulk_d4 | 1.007x | 0.999x | 1.290x |
| perft_full_d3 | 0.990x | 0.980x | 0.755x |
| fen_parse | 0.993x | 0.988x | 1.222x |
| fen_write_native_ep | 1.004x | 1.000x | 1.049x |
| snapshot | 1.001x | 1.000x | 0.932x |
| san_parse_all | 0.999x | 0.997x | 1.733x |
| san_render_native | 1.002x | 0.989x | 2.397x |
| apply_children_full | 1.015x | 0.999x | 0.838x |
| make_unmake_full | 1.074x | 1.062x | 1.242x |
| startpos | 1.035x | 1.035x | 1.140x |
| chess960_materialize | 1.000x | 0.994x | 1.225x |
| chess960_perft_d3 | 1.003x | 0.998x | 1.224x |
| polyglot_key | 0.987x | 0.974x | 24.979x |
| polyglot_recompute | 0.999x | 0.998x | 1.075x |
| repetition_key | 0.980x | 0.969x | 18.505x |
| corpus_count | 1.007x | 1.004x | 1.415x |
| corpus_materialize | 1.008x | 1.006x | 1.192x |
| corpus_uci_sorted | 0.999x | 0.995x | 0.973x |
| replay_trusted_native | 0.991x | 0.984x | 0.420x |
| replay_checked_native | 0.995x | 0.992x | 0.896x |

Cold first legal move list: 0.012 ms -> 0.013 ms (0.95x).
