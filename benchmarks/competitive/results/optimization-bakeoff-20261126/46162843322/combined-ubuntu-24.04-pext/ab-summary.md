| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.043x | 1.006x | 1.625x |
| materialize | 0.902x | 0.867x | 1.485x |
| perft_bulk_d4 | 1.048x | 1.026x | 1.255x |
| perft_full_d3 | 1.174x | 1.045x | 1.576x |
| fen_parse | 0.945x | 0.812x | 1.098x |
| fen_write_native_ep | 0.992x | 0.951x | 1.890x |
| snapshot | 1.001x | 0.999x | 1.001x |
| san_parse_all | 1.125x | 1.092x | 2.054x |
| san_render_native | 0.962x | 0.937x | 2.051x |
| apply_children_full | 0.994x | 0.920x | 1.346x |
| make_unmake_full | 0.960x | 0.888x | 1.139x |
| startpos | 0.890x | 0.890x | 0.997x |
| chess960_materialize | 0.928x | 0.928x | 1.400x |
| chess960_perft_d3 | 1.022x | 1.005x | 1.509x |
| polyglot_key | 1.000x | 1.000x | 39.683x |
| polyglot_recompute | 1.691x | 1.383x | 1.639x |
| repetition_key | 1.211x | 1.029x | 23.234x |
| corpus_count | 1.225x | 1.127x | 1.310x |
| corpus_materialize | 0.999x | 0.998x | 1.089x |
| corpus_uci_sorted | 1.015x | 1.009x | 1.579x |
| replay_trusted_native | 0.974x | 0.964x | 0.833x |
| replay_checked_native | 0.935x | 0.917x | 1.119x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.82x).
