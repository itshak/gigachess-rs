| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.972x | 1.533x |
| materialize | 1.023x | 0.953x | 1.506x |
| perft_bulk_d4 | 1.003x | 0.984x | 1.180x |
| perft_full_d3 | 0.999x | 0.981x | 1.373x |
| fen_parse | 1.000x | 0.991x | 1.166x |
| fen_write_native_ep | 0.949x | 0.907x | 1.844x |
| snapshot | 1.000x | 0.999x | 1.332x |
| san_parse_all | 0.995x | 0.991x | 1.742x |
| san_render_native | 0.999x | 0.994x | 2.155x |
| apply_children_full | 0.998x | 0.993x | 1.250x |
| make_unmake_full | 0.993x | 0.977x | 1.186x |
| startpos | 0.998x | 0.998x | 1.247x |
| chess960_materialize | 1.004x | 1.003x | 1.403x |
| chess960_perft_d3 | 1.016x | 1.007x | 1.496x |
| polyglot_key | 0.998x | 0.996x | 40.043x |
| polyglot_recompute | 1.004x | 1.003x | 0.990x |
| repetition_key | 1.097x | 1.001x | 32.445x |
| corpus_count | 1.002x | 0.996x | 0.959x |
| corpus_materialize | 0.995x | 0.993x | 1.064x |
| corpus_uci_sorted | 1.002x | 0.999x | 1.550x |
| replay_trusted_native | 0.940x | 0.890x | 0.829x |
| replay_checked_native | 0.995x | 0.990x | 1.150x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.81x).
