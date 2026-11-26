| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.202x |
| materialize | 0.998x | 0.987x | 1.626x |
| perft_bulk_d4 | 0.996x | 0.990x | 1.064x |
| perft_full_d3 | 0.999x | 0.989x | 1.442x |
| fen_parse | 1.010x | 1.004x | 1.039x |
| fen_write_native_ep | 0.978x | 0.957x | 2.090x |
| snapshot | 0.997x | 0.990x | 1.001x |
| san_parse_all | 0.999x | 0.990x | 1.927x |
| san_render_native | 1.001x | 0.998x | 2.126x |
| apply_children_full | 1.000x | 0.997x | 1.329x |
| make_unmake_full | 0.981x | 0.956x | 1.223x |
| startpos | 1.000x | 1.000x | 1.354x |
| chess960_materialize | 1.006x | 1.001x | 1.393x |
| chess960_perft_d3 | 1.005x | 1.001x | 1.393x |
| polyglot_key | 0.987x | 0.974x | 40.515x |
| polyglot_recompute | 1.060x | 1.056x | 1.013x |
| repetition_key | 1.006x | 0.999x | 19.386x |
| corpus_count | 0.996x | 0.993x | 0.979x |
| corpus_materialize | 0.999x | 0.998x | 1.081x |
| corpus_uci_sorted | 0.993x | 0.989x | 1.526x |
| replay_trusted_native | 1.006x | 1.006x | 0.872x |
| replay_checked_native | 1.020x | 1.015x | 1.090x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.07x).
