| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.225x |
| materialize | 1.000x | 0.986x | 1.614x |
| perft_bulk_d4 | 1.000x | 0.993x | 1.074x |
| perft_full_d3 | 0.994x | 0.984x | 1.463x |
| fen_parse | 0.982x | 0.950x | 1.205x |
| fen_write_native_ep | 0.985x | 0.966x | 1.822x |
| snapshot | 0.996x | 0.993x | 1.001x |
| san_parse_all | 1.082x | 1.035x | 1.869x |
| san_render_native | 0.982x | 0.967x | 2.159x |
| apply_children_full | 1.002x | 0.994x | 1.216x |
| make_unmake_full | 1.011x | 0.989x | 1.178x |
| startpos | 0.996x | 0.996x | 1.352x |
| chess960_materialize | 1.006x | 1.003x | 1.398x |
| chess960_perft_d3 | 1.001x | 0.998x | 1.395x |
| polyglot_key | 1.017x | 0.999x | 43.347x |
| polyglot_recompute | 1.055x | 1.040x | 1.075x |
| repetition_key | 0.981x | 0.962x | 17.459x |
| corpus_count | 0.999x | 0.997x | 1.042x |
| corpus_materialize | 0.995x | 0.995x | 1.083x |
| corpus_uci_sorted | 1.001x | 0.997x | 1.520x |
| replay_trusted_native | 1.000x | 0.997x | 0.848x |
| replay_checked_native | 0.991x | 0.986x | 1.138x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.09x).
