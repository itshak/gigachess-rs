| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.976x | 1.546x |
| materialize | 0.998x | 0.990x | 1.420x |
| perft_bulk_d4 | 0.998x | 0.982x | 1.230x |
| perft_full_d3 | 1.005x | 1.004x | 1.359x |
| fen_parse | 1.011x | 1.000x | 1.272x |
| fen_write_native_ep | 1.015x | 0.970x | 1.920x |
| snapshot | 1.000x | 0.998x | 1.133x |
| san_parse_all | 1.001x | 0.998x | 1.934x |
| san_render_native | 0.993x | 0.978x | 2.430x |
| apply_children_full | 0.998x | 0.986x | 1.252x |
| make_unmake_full | 0.984x | 0.942x | 1.338x |
| startpos | 1.001x | 1.001x | 1.000x |
| chess960_materialize | 0.997x | 0.989x | 1.448x |
| chess960_perft_d3 | 1.004x | 1.002x | 1.401x |
| polyglot_key | 1.010x | 1.000x | 31.301x |
| polyglot_recompute | 0.968x | 0.964x | 1.058x |
| repetition_key | 1.000x | 0.999x | 19.515x |
| corpus_count | 1.000x | 0.997x | 1.073x |
| corpus_materialize | 1.027x | 1.020x | 1.148x |
| corpus_uci_sorted | 0.997x | 0.994x | 1.487x |
| replay_trusted_native | 1.001x | 0.994x | 0.871x |
| replay_checked_native | 1.003x | 0.994x | 1.098x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.92x).
