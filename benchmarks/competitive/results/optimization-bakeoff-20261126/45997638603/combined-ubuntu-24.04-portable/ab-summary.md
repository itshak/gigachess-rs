| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.057x | 0.998x | 1.370x |
| materialize | 0.993x | 0.980x | 1.755x |
| perft_bulk_d4 | 1.062x | 1.033x | 1.266x |
| perft_full_d3 | 1.175x | 1.064x | 1.562x |
| fen_parse | 1.024x | 1.002x | 1.221x |
| fen_write_native_ep | 1.004x | 0.997x | 1.814x |
| snapshot | 1.000x | 0.999x | 1.001x |
| san_parse_all | 0.997x | 0.989x | 1.893x |
| san_render_native | 0.997x | 0.990x | 2.132x |
| apply_children_full | 1.003x | 1.001x | 1.347x |
| make_unmake_full | 1.057x | 1.000x | 1.262x |
| startpos | 1.001x | 1.001x | 1.429x |
| chess960_materialize | 0.914x | 0.904x | 1.480x |
| chess960_perft_d3 | 1.011x | 0.989x | 1.522x |
| polyglot_key | 1.117x | 0.996x | 43.928x |
| polyglot_recompute | 1.048x | 1.031x | 1.069x |
| repetition_key | 1.138x | 0.982x | 23.553x |
| corpus_count | 1.187x | 1.109x | 1.341x |
| corpus_materialize | 1.004x | 0.998x | 1.139x |
| corpus_uci_sorted | 0.986x | 0.978x | 1.516x |
| replay_trusted_native | 0.995x | 0.979x | 0.910x |
| replay_checked_native | 1.004x | 0.990x | 1.170x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.88x).
