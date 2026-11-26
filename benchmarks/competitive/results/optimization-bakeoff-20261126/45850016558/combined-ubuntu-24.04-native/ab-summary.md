| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.990x | 1.466x |
| materialize | 0.992x | 0.958x | 1.294x |
| perft_bulk_d4 | 1.001x | 0.994x | 1.343x |
| perft_full_d3 | 0.992x | 0.967x | 1.292x |
| fen_parse | 1.038x | 1.018x | 1.240x |
| fen_write_native_ep | 0.977x | 0.920x | 1.708x |
| snapshot | 0.464x | 0.462x | 0.698x |
| san_parse_all | 0.988x | 0.981x | 1.972x |
| san_render_native | 0.959x | 0.899x | 2.109x |
| apply_children_full | 0.993x | 0.933x | 1.353x |
| make_unmake_full | 1.045x | 1.006x | 1.289x |
| startpos | 1.011x | 1.011x | 1.333x |
| chess960_materialize | 0.977x | 0.961x | 1.473x |
| chess960_perft_d3 | 1.002x | 0.995x | 1.519x |
| polyglot_key | 0.986x | 0.971x | 25.331x |
| polyglot_recompute | 1.002x | 0.996x | 1.034x |
| repetition_key | 0.941x | 0.894x | 18.222x |
| corpus_count | 0.995x | 0.992x | 1.357x |
| corpus_materialize | 1.004x | 0.998x | 1.195x |
| corpus_uci_sorted | 1.007x | 1.001x | 1.744x |
| replay_trusted_native | 1.013x | 1.003x | 0.867x |
| replay_checked_native | 1.006x | 1.001x | 1.030x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.96x).
