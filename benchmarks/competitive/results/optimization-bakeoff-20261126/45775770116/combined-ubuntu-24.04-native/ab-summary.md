| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.948x | 1.241x |
| materialize | 1.027x | 1.000x | 1.192x |
| perft_bulk_d4 | 0.996x | 0.978x | 1.229x |
| perft_full_d3 | 0.995x | 0.986x | 0.781x |
| fen_parse | 0.984x | 0.959x | 1.163x |
| fen_write_native_ep | 0.994x | 0.986x | 0.944x |
| snapshot | 1.001x | 1.000x | 0.870x |
| san_parse_all | 0.979x | 0.974x | 1.883x |
| san_render_native | 1.011x | 0.990x | 2.180x |
| apply_children_full | 1.007x | 0.986x | 0.956x |
| make_unmake_full | 1.009x | 0.980x | 1.020x |
| startpos | 1.000x | 1.000x | 1.145x |
| chess960_materialize | 0.997x | 0.976x | 1.292x |
| chess960_perft_d3 | 0.994x | 0.984x | 1.325x |
| polyglot_key | 0.875x | 0.798x | 24.000x |
| polyglot_recompute | 1.003x | 0.998x | 1.038x |
| repetition_key | 0.911x | 0.853x | 18.969x |
| corpus_count | 0.999x | 0.984x | 1.322x |
| corpus_materialize | 0.994x | 0.991x | 1.181x |
| corpus_uci_sorted | 1.021x | 1.013x | 1.032x |
| replay_trusted_native | 1.010x | 1.000x | 0.615x |
| replay_checked_native | 0.952x | 0.947x | 0.903x |

Cold first legal move list: 0.013 ms -> 0.013 ms (0.98x).
