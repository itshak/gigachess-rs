| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.356x |
| materialize | 1.000x | 0.996x | 1.238x |
| perft_bulk_d4 | 0.990x | 0.983x | 1.331x |
| perft_full_d3 | 0.996x | 0.979x | 1.068x |
| fen_parse | 1.003x | 0.992x | 1.242x |
| fen_write_native_ep | 1.002x | 0.999x | 1.587x |
| snapshot | 1.002x | 0.994x | 0.907x |
| san_parse_all | 0.997x | 0.990x | 1.821x |
| san_render_native | 1.003x | 0.983x | 2.568x |
| apply_children_full | 0.982x | 0.967x | 1.035x |
| make_unmake_full | 1.007x | 0.996x | 1.440x |
| startpos | 0.999x | 0.999x | 1.184x |
| chess960_materialize | 1.001x | 1.000x | 1.322x |
| chess960_perft_d3 | 0.995x | 0.989x | 1.323x |
| polyglot_key | 1.006x | 0.996x | 25.657x |
| polyglot_recompute | 1.048x | 1.032x | 1.125x |
| repetition_key | 1.003x | 1.001x | 19.129x |
| corpus_count | 1.000x | 0.999x | 1.461x |
| corpus_materialize | 1.015x | 1.013x | 1.210x |
| corpus_uci_sorted | 1.008x | 1.008x | 1.460x |
| replay_trusted_native | 0.980x | 0.978x | 0.485x |
| replay_checked_native | 0.993x | 0.989x | 0.978x |

Cold first legal move list: 0.012 ms -> 0.013 ms (0.93x).
