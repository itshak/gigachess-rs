| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.999x | 1.185x |
| materialize | 0.925x | 0.884x | 1.559x |
| perft_bulk_d4 | 1.001x | 0.996x | 1.163x |
| perft_full_d3 | 1.000x | 0.997x | 1.486x |
| fen_parse | 1.092x | 1.076x | 1.163x |
| fen_write_native_ep | 1.006x | 0.985x | 1.826x |
| snapshot | 1.254x | 1.249x | 1.002x |
| san_parse_all | 1.067x | 1.034x | 1.891x |
| san_render_native | 0.982x | 0.977x | 2.142x |
| apply_children_full | 0.966x | 0.904x | 1.077x |
| make_unmake_full | 1.004x | 0.997x | 1.197x |
| startpos | 0.995x | 0.995x | 1.353x |
| chess960_materialize | 0.997x | 0.995x | 1.432x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.328x |
| polyglot_key | 1.005x | 0.972x | 41.407x |
| polyglot_recompute | 1.042x | 1.022x | 1.052x |
| repetition_key | 0.965x | 0.937x | 20.936x |
| corpus_count | 1.003x | 1.002x | 1.270x |
| corpus_materialize | 0.999x | 0.997x | 1.155x |
| corpus_uci_sorted | 0.996x | 0.993x | 1.573x |
| replay_trusted_native | 1.005x | 0.996x | 0.685x |
| replay_checked_native | 0.999x | 0.997x | 1.087x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.91x).
