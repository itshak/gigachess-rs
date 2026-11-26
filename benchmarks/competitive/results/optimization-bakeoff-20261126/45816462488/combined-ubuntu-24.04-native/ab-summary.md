| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.947x | 1.330x |
| materialize | 1.029x | 0.952x | 1.477x |
| perft_bulk_d4 | 0.993x | 0.954x | 1.000x |
| perft_full_d3 | 1.001x | 0.973x | 1.499x |
| fen_parse | 0.982x | 0.960x | 1.099x |
| fen_write_native_ep | 1.075x | 1.043x | 2.115x |
| snapshot | 0.977x | 0.959x | 0.959x |
| san_parse_all | 1.089x | 1.025x | 1.809x |
| san_render_native | 1.010x | 0.953x | 2.167x |
| apply_children_full | 0.997x | 0.940x | 1.174x |
| make_unmake_full | 0.977x | 0.943x | 1.104x |
| startpos | 1.067x | 1.067x | 1.215x |
| chess960_materialize | 1.017x | 0.997x | 1.295x |
| chess960_perft_d3 | 0.959x | 0.948x | 1.356x |
| polyglot_key | 1.035x | 1.010x | 41.052x |
| polyglot_recompute | 1.067x | 1.026x | 1.046x |
| repetition_key | 0.990x | 0.979x | 21.463x |
| corpus_count | 1.022x | 0.974x | 0.838x |
| corpus_materialize | 0.981x | 0.970x | 1.015x |
| corpus_uci_sorted | 0.921x | 0.883x | 1.427x |
| replay_trusted_native | 0.980x | 0.969x | 0.787x |
| replay_checked_native | 1.037x | 0.982x | 1.136x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.87x).
