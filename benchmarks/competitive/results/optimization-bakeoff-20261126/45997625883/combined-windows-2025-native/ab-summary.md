| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.993x | 1.522x |
| materialize | 1.012x | 1.000x | 1.272x |
| perft_bulk_d4 | 1.018x | 0.999x | 1.377x |
| perft_full_d3 | 1.002x | 0.999x | 1.319x |
| fen_parse | 1.012x | 0.996x | 1.298x |
| fen_write_native_ep | 1.047x | 1.038x | 1.708x |
| snapshot | 0.993x | 0.981x | 0.994x |
| san_parse_all | 0.999x | 0.993x | 2.132x |
| san_render_native | 0.997x | 0.988x | 3.872x |
| apply_children_full | 1.000x | 0.984x | 1.409x |
| make_unmake_full | 1.000x | 0.977x | 1.375x |
| startpos | 0.997x | 0.997x | 1.245x |
| chess960_materialize | 1.031x | 1.015x | 1.337x |
| chess960_perft_d3 | 1.004x | 1.002x | 1.543x |
| polyglot_key | 0.953x | 0.931x | 26.239x |
| polyglot_recompute | 1.001x | 0.999x | 1.052x |
| repetition_key | 1.014x | 1.012x | 21.102x |
| corpus_count | 0.998x | 0.994x | 1.378x |
| corpus_materialize | 1.006x | 0.997x | 1.160x |
| corpus_uci_sorted | 1.001x | 0.994x | 1.469x |
| replay_trusted_native | 1.018x | 1.015x | 0.857x |
| replay_checked_native | 0.873x | 0.750x | 0.957x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.01x).
