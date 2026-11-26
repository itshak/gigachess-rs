| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.958x | 1.291x |
| materialize | 0.999x | 0.977x | 1.464x |
| perft_bulk_d4 | 0.987x | 0.949x | 1.039x |
| perft_full_d3 | 0.998x | 0.995x | 1.494x |
| fen_parse | 1.125x | 1.093x | 1.191x |
| fen_write_native_ep | 1.060x | 1.030x | 2.047x |
| snapshot | 1.003x | 0.986x | 1.154x |
| san_parse_all | 0.951x | 0.939x | 1.713x |
| san_render_native | 1.000x | 0.997x | 2.133x |
| apply_children_full | 1.006x | 1.001x | 1.315x |
| make_unmake_full | 0.959x | 0.928x | 1.133x |
| startpos | 1.006x | 1.006x | 1.126x |
| chess960_materialize | 1.004x | 1.003x | 1.291x |
| chess960_perft_d3 | 0.966x | 0.932x | 1.330x |
| polyglot_key | 1.061x | 1.003x | 39.648x |
| polyglot_recompute | 1.000x | 0.994x | 0.996x |
| repetition_key | 1.146x | 0.994x | 24.086x |
| corpus_count | 1.003x | 1.001x | 0.833x |
| corpus_materialize | 0.997x | 0.997x | 1.072x |
| corpus_uci_sorted | 1.006x | 1.002x | 1.551x |
| replay_trusted_native | 0.980x | 0.950x | 0.860x |
| replay_checked_native | 1.059x | 1.028x | 1.177x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.00x).
