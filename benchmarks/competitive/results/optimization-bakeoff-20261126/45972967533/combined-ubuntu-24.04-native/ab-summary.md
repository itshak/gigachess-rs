| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.078x | 1.023x | 1.581x |
| materialize | 1.004x | 0.928x | 1.483x |
| perft_bulk_d4 | 1.073x | 1.050x | 1.240x |
| perft_full_d3 | 1.260x | 1.052x | 1.705x |
| fen_parse | 0.893x | 0.875x | 0.991x |
| fen_write_native_ep | 0.994x | 0.975x | 2.138x |
| snapshot | 0.998x | 0.991x | 1.330x |
| san_parse_all | 1.015x | 0.957x | 1.802x |
| san_render_native | 0.975x | 0.955x | 2.107x |
| apply_children_full | 1.001x | 0.999x | 1.320x |
| make_unmake_full | 0.998x | 0.977x | 1.195x |
| startpos | 0.799x | 0.799x | 1.000x |
| chess960_materialize | 1.033x | 0.999x | 1.430x |
| chess960_perft_d3 | 1.009x | 0.995x | 1.518x |
| polyglot_key | 0.975x | 0.955x | 38.519x |
| polyglot_recompute | 0.964x | 0.945x | 0.939x |
| repetition_key | 1.240x | 1.198x | 31.636x |
| corpus_count | 1.398x | 1.137x | 1.303x |
| corpus_materialize | 1.007x | 1.004x | 1.053x |
| corpus_uci_sorted | 0.999x | 0.997x | 1.568x |
| replay_trusted_native | 1.061x | 0.996x | 0.910x |
| replay_checked_native | 1.014x | 1.012x | 1.256x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.19x).
