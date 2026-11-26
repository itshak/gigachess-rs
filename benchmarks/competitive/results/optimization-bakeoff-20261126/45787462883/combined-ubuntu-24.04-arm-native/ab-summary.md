| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.357x |
| materialize | 1.000x | 0.994x | 1.232x |
| perft_bulk_d4 | 0.845x | 0.807x | 1.150x |
| perft_full_d3 | 0.949x | 0.930x | 1.007x |
| fen_parse | 0.997x | 0.991x | 1.237x |
| fen_write_native_ep | 0.992x | 0.982x | 1.589x |
| snapshot | 0.998x | 0.995x | 0.925x |
| san_parse_all | 0.985x | 0.970x | 1.798x |
| san_render_native | 0.839x | 0.806x | 2.130x |
| apply_children_full | 0.872x | 0.818x | 0.903x |
| make_unmake_full | 0.727x | 0.638x | 1.050x |
| startpos | 0.968x | 0.968x | 1.147x |
| chess960_materialize | 0.988x | 0.985x | 1.314x |
| chess960_perft_d3 | 0.877x | 0.815x | 1.176x |
| polyglot_key | 0.995x | 0.975x | 25.421x |
| polyglot_recompute | 1.001x | 1.001x | 1.073x |
| repetition_key | 0.989x | 0.982x | 19.118x |
| corpus_count | 0.997x | 0.991x | 1.445x |
| corpus_materialize | 1.009x | 1.008x | 1.203x |
| corpus_uci_sorted | 1.008x | 1.000x | 1.458x |
| replay_trusted_native | 0.922x | 0.864x | 0.477x |
| replay_checked_native | 0.897x | 0.891x | 0.883x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.03x).
