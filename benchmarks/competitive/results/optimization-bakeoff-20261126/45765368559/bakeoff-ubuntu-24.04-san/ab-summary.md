| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.999x | 1.192x |
| materialize | 0.952x | 0.846x | 1.290x |
| perft_bulk_d4 | 1.013x | 0.999x | 0.880x |
| perft_full_d3 | 0.999x | 0.988x | 0.602x |
| fen_parse | 1.027x | 0.997x | 1.227x |
| fen_write_native_ep | 1.032x | 1.010x | 1.370x |
| snapshot | 1.053x | 1.048x | 0.789x |
| san_parse_all | 1.531x | 1.470x | 1.403x |
| san_render_native | 1.680x | 1.618x | 1.781x |
| apply_children_full | 0.996x | 0.991x | 0.668x |
| make_unmake_full | 0.997x | 0.987x | 0.751x |
| startpos | 0.960x | 0.960x | 0.590x |
| chess960_materialize | 1.060x | 1.045x | 1.208x |
| chess960_perft_d3 | 1.004x | 1.003x | 0.959x |
| polyglot_key | 0.962x | 0.928x | 36.585x |
| polyglot_recompute | 1.031x | 1.021x | 1.015x |
| repetition_key | 1.027x | 0.996x | 15.262x |
| corpus_count | 0.997x | 0.997x | 1.147x |
| corpus_materialize | 1.006x | 1.006x | 1.089x |
| corpus_uci_sorted | 0.950x | 0.948x | 0.596x |
| replay_trusted_native | 1.000x | 0.993x | 0.526x |
| replay_checked_native | 1.047x | 1.006x | 0.788x |

Cold first legal move list: 142.726 ms → 138.521 ms (1.03x).
