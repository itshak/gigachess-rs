| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 1.001x | 1.197x |
| materialize | 0.939x | 0.888x | 1.543x |
| perft_bulk_d4 | 0.990x | 0.976x | 1.154x |
| perft_full_d3 | 1.000x | 0.992x | 1.511x |
| fen_parse | 0.942x | 0.888x | 1.135x |
| fen_write_native_ep | 1.027x | 1.017x | 1.934x |
| snapshot | 1.004x | 0.997x | 1.003x |
| san_parse_all | 0.920x | 0.907x | 1.760x |
| san_render_native | 1.003x | 0.995x | 2.143x |
| apply_children_full | 1.028x | 1.010x | 1.167x |
| make_unmake_full | 0.995x | 0.946x | 1.233x |
| startpos | 1.008x | 1.008x | 1.289x |
| chess960_materialize | 0.990x | 0.974x | 1.406x |
| chess960_perft_d3 | 1.007x | 1.005x | 1.334x |
| polyglot_key | 0.897x | 0.801x | 39.395x |
| polyglot_recompute | 0.922x | 0.895x | 0.987x |
| repetition_key | 0.998x | 0.990x | 21.454x |
| corpus_count | 0.998x | 0.997x | 1.251x |
| corpus_materialize | 1.017x | 1.013x | 1.166x |
| corpus_uci_sorted | 0.995x | 0.988x | 1.555x |
| replay_trusted_native | 1.014x | 1.004x | 0.731x |
| replay_checked_native | 1.005x | 0.997x | 1.099x |

Cold first legal move list: 0.009 ms -> 0.012 ms (0.71x).
