| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.027x | 0.992x | 1.372x |
| materialize | 0.994x | 0.911x | 1.432x |
| perft_bulk_d4 | 0.996x | 0.980x | 1.347x |
| perft_full_d3 | 1.019x | 0.984x | 1.303x |
| fen_parse | 0.984x | 0.956x | 1.265x |
| fen_write_native_ep | 1.001x | 0.992x | 1.731x |
| snapshot | 0.997x | 0.990x | 0.973x |
| san_parse_all | 1.033x | 1.002x | 2.231x |
| san_render_native | 1.041x | 0.989x | 2.341x |
| apply_children_full | 1.033x | 0.975x | 1.413x |
| make_unmake_full | 1.014x | 0.990x | 1.585x |
| startpos | 1.000x | 1.000x | 1.335x |
| chess960_materialize | 0.992x | 0.987x | 1.633x |
| chess960_perft_d3 | 1.032x | 1.004x | 1.614x |
| polyglot_key | 0.999x | 0.996x | 21.610x |
| polyglot_recompute | 1.022x | 1.013x | 0.990x |
| repetition_key | 1.003x | 1.002x | 20.178x |
| corpus_count | 1.000x | 0.999x | 1.229x |
| corpus_materialize | 1.039x | 0.988x | 1.320x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.589x |
| replay_trusted_native | 0.977x | 0.954x | 1.063x |
| replay_checked_native | 1.008x | 1.006x | 1.158x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.91x).
