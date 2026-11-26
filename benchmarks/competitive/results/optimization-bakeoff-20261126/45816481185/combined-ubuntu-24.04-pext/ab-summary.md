| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.990x | 1.385x |
| materialize | 0.999x | 0.989x | 1.524x |
| perft_bulk_d4 | 1.001x | 0.995x | 1.064x |
| perft_full_d3 | 1.005x | 0.994x | 1.463x |
| fen_parse | 1.000x | 0.980x | 1.046x |
| fen_write_native_ep | 0.963x | 0.933x | 2.063x |
| snapshot | 0.999x | 0.989x | 1.148x |
| san_parse_all | 1.000x | 0.988x | 1.901x |
| san_render_native | 1.011x | 1.002x | 2.179x |
| apply_children_full | 0.998x | 0.991x | 1.182x |
| make_unmake_full | 1.006x | 1.003x | 1.121x |
| startpos | 1.111x | 1.111x | 1.120x |
| chess960_materialize | 1.085x | 1.053x | 1.458x |
| chess960_perft_d3 | 1.005x | 1.005x | 1.380x |
| polyglot_key | 1.014x | 1.001x | 39.538x |
| polyglot_recompute | 1.006x | 1.004x | 1.036x |
| repetition_key | 1.036x | 1.018x | 16.891x |
| corpus_count | 1.001x | 0.999x | 0.925x |
| corpus_materialize | 1.010x | 1.004x | 1.075x |
| corpus_uci_sorted | 1.000x | 0.994x | 1.559x |
| replay_trusted_native | 0.986x | 0.984x | 0.834x |
| replay_checked_native | 1.004x | 1.003x | 1.119x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.03x).
