| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.993x | 1.479x |
| materialize | 1.010x | 0.998x | 1.485x |
| perft_bulk_d4 | 1.006x | 0.997x | 1.259x |
| perft_full_d3 | 0.989x | 0.954x | 1.508x |
| fen_parse | 1.038x | 1.015x | 1.173x |
| fen_write_native_ep | 0.990x | 0.982x | 1.890x |
| snapshot | 0.912x | 0.910x | 0.999x |
| san_parse_all | 1.029x | 1.018x | 2.123x |
| san_render_native | 1.023x | 1.019x | 2.169x |
| apply_children_full | 0.993x | 0.942x | 1.276x |
| make_unmake_full | 1.002x | 0.984x | 1.108x |
| startpos | 1.000x | 1.000x | 1.332x |
| chess960_materialize | 0.994x | 0.991x | 1.467x |
| chess960_perft_d3 | 1.002x | 1.000x | 1.377x |
| polyglot_key | 1.013x | 1.000x | 39.978x |
| polyglot_recompute | 1.007x | 1.006x | 1.024x |
| repetition_key | 0.996x | 0.989x | 19.411x |
| corpus_count | 1.006x | 1.002x | 1.403x |
| corpus_materialize | 1.009x | 1.006x | 1.124x |
| corpus_uci_sorted | 1.003x | 1.002x | 1.602x |
| replay_trusted_native | 1.003x | 0.964x | 0.797x |
| replay_checked_native | 0.994x | 0.990x | 1.098x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.89x).
