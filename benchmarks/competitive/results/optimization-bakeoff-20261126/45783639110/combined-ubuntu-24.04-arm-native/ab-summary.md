| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.997x | 1.357x |
| materialize | 1.004x | 0.994x | 1.233x |
| perft_bulk_d4 | 1.004x | 1.000x | 1.350x |
| perft_full_d3 | 0.995x | 0.978x | 1.077x |
| fen_parse | 0.999x | 0.993x | 1.236x |
| fen_write_native_ep | 1.002x | 0.998x | 1.041x |
| snapshot | 0.989x | 0.984x | 0.864x |
| san_parse_all | 0.999x | 0.995x | 1.812x |
| san_render_native | 1.001x | 0.990x | 2.542x |
| apply_children_full | 0.991x | 0.970x | 1.034x |
| make_unmake_full | 1.007x | 0.999x | 1.435x |
| startpos | 1.073x | 1.073x | 1.184x |
| chess960_materialize | 0.993x | 0.986x | 1.314x |
| chess960_perft_d3 | 1.011x | 1.007x | 1.324x |
| polyglot_key | 1.012x | 0.999x | 25.520x |
| polyglot_recompute | 0.995x | 0.991x | 1.071x |
| repetition_key | 0.992x | 0.969x | 18.750x |
| corpus_count | 1.003x | 0.999x | 1.455x |
| corpus_materialize | 1.006x | 1.003x | 1.191x |
| corpus_uci_sorted | 0.996x | 0.993x | 1.449x |
| replay_trusted_native | 1.000x | 0.996x | 0.505x |
| replay_checked_native | 0.990x | 0.983x | 0.984x |

Cold first legal move list: 0.012 ms -> 0.012 ms (1.00x).
