| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.980x | 1.279x |
| materialize | 0.986x | 0.973x | 1.722x |
| perft_bulk_d4 | 1.000x | 0.990x | 1.182x |
| perft_full_d3 | 1.206x | 1.055x | 1.598x |
| fen_parse | 1.114x | 1.061x | 1.181x |
| fen_write_native_ep | 1.018x | 0.986x | 1.788x |
| snapshot | 1.000x | 0.996x | 0.998x |
| san_parse_all | 1.009x | 0.981x | 1.903x |
| san_render_native | 1.014x | 0.998x | 2.213x |
| apply_children_full | 1.003x | 1.000x | 1.390x |
| make_unmake_full | 1.049x | 1.012x | 1.290x |
| startpos | 1.001x | 1.001x | 1.290x |
| chess960_materialize | 1.000x | 0.998x | 1.551x |
| chess960_perft_d3 | 1.005x | 0.992x | 1.505x |
| polyglot_key | 1.011x | 1.000x | 43.908x |
| polyglot_recompute | 1.029x | 1.027x | 1.048x |
| repetition_key | 0.978x | 0.967x | 21.236x |
| corpus_count | 1.004x | 1.001x | 1.139x |
| corpus_materialize | 0.995x | 0.980x | 1.133x |
| corpus_uci_sorted | 1.001x | 1.000x | 1.557x |
| replay_trusted_native | 0.998x | 0.994x | 0.886x |
| replay_checked_native | 1.006x | 0.997x | 1.148x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.96x).
