| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.042x | 1.002x | 1.629x |
| materialize | 1.010x | 0.988x | 1.676x |
| perft_bulk_d4 | 1.029x | 1.011x | 1.230x |
| perft_full_d3 | 1.233x | 1.056x | 1.654x |
| fen_parse | 0.950x | 0.814x | 1.110x |
| fen_write_native_ep | 1.020x | 0.994x | 2.077x |
| snapshot | 0.999x | 0.986x | 1.148x |
| san_parse_all | 1.116x | 1.067x | 1.870x |
| san_render_native | 0.997x | 0.990x | 2.118x |
| apply_children_full | 0.973x | 0.906x | 1.308x |
| make_unmake_full | 1.003x | 0.975x | 1.207x |
| startpos | 1.004x | 1.004x | 1.120x |
| chess960_materialize | 0.964x | 0.945x | 1.449x |
| chess960_perft_d3 | 1.010x | 0.980x | 1.493x |
| polyglot_key | 0.977x | 0.958x | 39.025x |
| polyglot_recompute | 1.686x | 1.384x | 1.632x |
| repetition_key | 0.962x | 0.913x | 18.670x |
| corpus_count | 1.170x | 1.130x | 1.235x |
| corpus_materialize | 1.001x | 0.995x | 1.093x |
| corpus_uci_sorted | 1.003x | 0.996x | 1.554x |
| replay_trusted_native | 1.118x | 1.087x | 0.951x |
| replay_checked_native | 0.985x | 0.983x | 1.108x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.79x).
