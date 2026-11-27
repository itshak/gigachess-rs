| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.998x | 1.575x |
| materialize | 1.014x | 0.980x | 1.149x |
| perft_bulk_d4 | 1.057x | 1.038x | 1.505x |
| perft_full_d3 | 1.326x | 1.078x | 1.703x |
| fen_parse | 1.066x | 0.977x | 1.509x |
| fen_write_native_ep | 0.994x | 0.979x | 1.573x |
| snapshot | 0.999x | 0.994x | 1.130x |
| san_parse_all | 1.016x | 0.993x | 1.963x |
| san_render_native | 1.027x | 1.008x | 3.421x |
| apply_children_full | 1.041x | 0.981x | 1.326x |
| make_unmake_full | 1.023x | 0.980x | 1.313x |
| startpos | 1.000x | 1.000x | 1.000x |
| chess960_materialize | 1.001x | 0.988x | 1.360x |
| chess960_perft_d3 | 1.010x | 1.010x | 1.603x |
| polyglot_key | 1.006x | 0.990x | 20.676x |
| polyglot_recompute | 1.003x | 0.999x | 0.980x |
| repetition_key | 0.997x | 0.996x | 19.026x |
| corpus_count | 1.060x | 1.041x | 1.384x |
| corpus_materialize | 1.006x | 1.003x | 1.209x |
| corpus_uci_sorted | 1.010x | 1.008x | 1.506x |
| replay_trusted_native | 1.130x | 1.011x | 1.197x |
| replay_checked_native | 0.993x | 0.993x | 1.098x |

Cold first legal move list: 0.016 ms -> 0.013 ms (1.23x).
