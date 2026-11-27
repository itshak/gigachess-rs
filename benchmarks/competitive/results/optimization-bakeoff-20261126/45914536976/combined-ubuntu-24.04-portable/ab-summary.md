| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.992x | 1.310x |
| materialize | 0.995x | 0.983x | 1.337x |
| perft_bulk_d4 | 1.002x | 0.985x | 1.301x |
| perft_full_d3 | 1.001x | 0.992x | 1.280x |
| fen_parse | 0.955x | 0.938x | 1.197x |
| fen_write_native_ep | 1.051x | 0.991x | 1.744x |
| snapshot | 0.998x | 0.984x | 1.010x |
| san_parse_all | 0.997x | 0.983x | 1.957x |
| san_render_native | 0.985x | 0.975x | 2.177x |
| apply_children_full | 1.017x | 0.989x | 1.432x |
| make_unmake_full | 0.947x | 0.903x | 1.345x |
| startpos | 0.999x | 0.999x | 1.329x |
| chess960_materialize | 0.979x | 0.962x | 1.438x |
| chess960_perft_d3 | 0.994x | 0.993x | 1.540x |
| polyglot_key | 0.947x | 0.900x | 22.572x |
| polyglot_recompute | 0.998x | 0.986x | 0.993x |
| repetition_key | 1.038x | 0.997x | 18.492x |
| corpus_count | 1.019x | 1.007x | 1.305x |
| corpus_materialize | 0.999x | 0.998x | 1.222x |
| corpus_uci_sorted | 1.016x | 1.009x | 1.741x |
| replay_trusted_native | 0.998x | 0.993x | 0.920x |
| replay_checked_native | 0.951x | 0.945x | 1.098x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.91x).
