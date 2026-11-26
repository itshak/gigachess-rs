| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.918x | 0.687x | 1.261x |
| materialize | 0.966x | 0.829x | 1.013x |
| perft_bulk_d4 | 0.971x | 0.784x | 1.251x |
| perft_full_d3 | 0.976x | 0.834x | 0.828x |
| fen_parse | 0.869x | 0.773x | 1.154x |
| fen_write_native_ep | 0.968x | 0.849x | 1.268x |
| snapshot | 0.990x | 0.824x | 0.956x |
| san_parse_all | 0.959x | 0.885x | 1.618x |
| san_render_native | 0.967x | 0.754x | 2.575x |
| apply_children_full | 1.102x | 0.966x | 0.839x |
| make_unmake_full | 1.074x | 0.994x | 1.064x |
| startpos | 0.978x | 0.978x | 1.112x |
| chess960_materialize | 1.215x | 0.939x | 1.154x |
| chess960_perft_d3 | 1.220x | 1.109x | 1.490x |
| polyglot_key | 1.251x | 1.217x | 38.302x |
| polyglot_recompute | 1.192x | 1.173x | 1.005x |
| repetition_key | 1.196x | 1.114x | 18.997x |
| corpus_count | 1.319x | 1.197x | 1.265x |
| corpus_materialize | 1.220x | 1.186x | 1.180x |
| corpus_uci_sorted | 1.199x | 1.155x | 1.010x |
| replay_trusted_native | 1.291x | 1.039x | 0.495x |
| replay_checked_native | 0.962x | 0.755x | 0.802x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.78x).
