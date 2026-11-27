| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.982x | 1.564x |
| materialize | 1.014x | 0.997x | 1.344x |
| perft_bulk_d4 | 1.042x | 1.023x | 1.450x |
| perft_full_d3 | 1.130x | 1.006x | 1.507x |
| fen_parse | 0.997x | 0.993x | 1.219x |
| fen_write_native_ep | 1.001x | 0.999x | 1.570x |
| snapshot | 0.999x | 0.991x | 0.944x |
| san_parse_all | 0.999x | 0.994x | 1.826x |
| san_render_native | 1.001x | 0.993x | 2.699x |
| apply_children_full | 1.000x | 0.998x | 1.155x |
| make_unmake_full | 1.002x | 0.988x | 1.556x |
| startpos | 1.001x | 1.001x | 1.308x |
| chess960_materialize | 1.005x | 1.004x | 1.502x |
| chess960_perft_d3 | 1.017x | 1.001x | 1.524x |
| polyglot_key | 1.014x | 0.997x | 25.700x |
| polyglot_recompute | 0.996x | 0.995x | 1.084x |
| repetition_key | 1.009x | 1.000x | 19.204x |
| corpus_count | 1.091x | 1.061x | 1.489x |
| corpus_materialize | 1.008x | 1.007x | 1.245x |
| corpus_uci_sorted | 1.004x | 1.002x | 1.458x |
| replay_trusted_native | 0.998x | 0.996x | 0.626x |
| replay_checked_native | 1.007x | 1.005x | 1.136x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.85x).
