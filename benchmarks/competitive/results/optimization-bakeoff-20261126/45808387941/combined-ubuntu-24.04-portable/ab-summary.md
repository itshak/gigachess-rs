| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.989x | 0.920x | 1.142x |
| materialize | 1.025x | 0.970x | 1.280x |
| perft_bulk_d4 | 0.973x | 0.914x | 1.128x |
| perft_full_d3 | 0.992x | 0.950x | 1.354x |
| fen_parse | 1.012x | 0.990x | 1.221x |
| fen_write_native_ep | 0.996x | 0.980x | 1.526x |
| snapshot | 1.010x | 0.997x | 0.999x |
| san_parse_all | 0.978x | 0.949x | 1.703x |
| san_render_native | 1.013x | 0.989x | 2.412x |
| apply_children_full | 1.026x | 0.964x | 1.173x |
| make_unmake_full | 0.992x | 0.962x | 1.243x |
| startpos | 1.023x | 1.023x | 1.287x |
| chess960_materialize | 1.148x | 1.071x | 1.475x |
| chess960_perft_d3 | 1.048x | 1.006x | 1.360x |
| polyglot_key | 1.015x | 0.993x | 32.705x |
| polyglot_recompute | 1.074x | 0.991x | 1.060x |
| repetition_key | 0.785x | 0.773x | 20.061x |
| corpus_count | 0.910x | 0.822x | 1.026x |
| corpus_materialize | 0.896x | 0.853x | 1.111x |
| corpus_uci_sorted | 0.978x | 0.938x | 1.606x |
| replay_trusted_native | 0.970x | 0.960x | 0.615x |
| replay_checked_native | 0.955x | 0.933x | 0.983x |

Cold first legal move list: 0.011 ms -> 0.006 ms (1.92x).
