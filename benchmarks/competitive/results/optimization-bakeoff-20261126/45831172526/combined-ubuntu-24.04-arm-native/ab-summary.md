| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.386x |
| materialize | 1.000x | 0.986x | 1.247x |
| perft_bulk_d4 | 0.996x | 0.992x | 1.234x |
| perft_full_d3 | 1.000x | 0.990x | 1.048x |
| fen_parse | 1.000x | 0.997x | 1.208x |
| fen_write_native_ep | 1.000x | 0.998x | 1.572x |
| snapshot | 0.980x | 0.973x | 0.954x |
| san_parse_all | 0.982x | 0.977x | 1.791x |
| san_render_native | 1.015x | 1.003x | 2.556x |
| apply_children_full | 0.990x | 0.967x | 1.143x |
| make_unmake_full | 1.017x | 0.986x | 1.575x |
| startpos | 1.000x | 1.000x | 1.334x |
| chess960_materialize | 1.005x | 1.002x | 1.393x |
| chess960_perft_d3 | 1.001x | 0.993x | 1.347x |
| polyglot_key | 1.008x | 1.001x | 25.172x |
| polyglot_recompute | 1.039x | 1.027x | 1.126x |
| repetition_key | 1.010x | 1.004x | 19.202x |
| corpus_count | 1.000x | 0.993x | 1.169x |
| corpus_materialize | 0.994x | 0.987x | 1.147x |
| corpus_uci_sorted | 1.007x | 1.005x | 1.448x |
| replay_trusted_native | 1.023x | 0.996x | 0.637x |
| replay_checked_native | 1.009x | 1.007x | 1.052x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.97x).
