| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.550x |
| materialize | 1.005x | 1.001x | 1.327x |
| perft_bulk_d4 | 1.002x | 0.994x | 1.400x |
| perft_full_d3 | 1.002x | 0.998x | 1.330x |
| fen_parse | 0.992x | 0.985x | 1.216x |
| fen_write_native_ep | 0.992x | 0.982x | 1.558x |
| snapshot | 0.981x | 0.978x | 0.956x |
| san_parse_all | 0.993x | 0.988x | 1.830x |
| san_render_native | 1.000x | 0.990x | 2.665x |
| apply_children_full | 0.992x | 0.972x | 1.152x |
| make_unmake_full | 1.018x | 0.995x | 1.585x |
| startpos | 1.002x | 1.002x | 1.337x |
| chess960_materialize | 1.005x | 1.001x | 1.509x |
| chess960_perft_d3 | 0.998x | 0.992x | 1.495x |
| polyglot_key | 0.988x | 0.978x | 25.328x |
| polyglot_recompute | 0.997x | 0.994x | 1.086x |
| repetition_key | 0.995x | 0.991x | 19.249x |
| corpus_count | 0.998x | 0.995x | 1.350x |
| corpus_materialize | 0.993x | 0.993x | 1.224x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.461x |
| replay_trusted_native | 0.994x | 0.981x | 0.620x |
| replay_checked_native | 0.997x | 0.995x | 1.133x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.95x).
