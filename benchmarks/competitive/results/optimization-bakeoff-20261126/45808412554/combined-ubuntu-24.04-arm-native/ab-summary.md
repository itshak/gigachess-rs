| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.117x | 0.978x | 1.511x |
| materialize | 1.047x | 0.984x | 1.300x |
| perft_bulk_d4 | 1.078x | 0.992x | 1.447x |
| perft_full_d3 | 0.993x | 0.965x | 1.052x |
| fen_parse | 1.007x | 1.000x | 1.249x |
| fen_write_native_ep | 0.992x | 0.980x | 1.574x |
| snapshot | 0.925x | 0.921x | 0.960x |
| san_parse_all | 1.000x | 0.993x | 1.803x |
| san_render_native | 0.991x | 0.987x | 2.551x |
| apply_children_full | 0.951x | 0.929x | 1.084x |
| make_unmake_full | 1.003x | 0.998x | 1.447x |
| startpos | 0.899x | 0.899x | 1.166x |
| chess960_materialize | 1.067x | 1.041x | 1.408x |
| chess960_perft_d3 | 1.089x | 1.062x | 1.432x |
| polyglot_key | 0.982x | 0.968x | 25.341x |
| polyglot_recompute | 0.995x | 0.990x | 1.071x |
| repetition_key | 0.996x | 0.990x | 19.136x |
| corpus_count | 1.061x | 1.028x | 1.556x |
| corpus_materialize | 1.018x | 1.010x | 1.207x |
| corpus_uci_sorted | 0.996x | 0.995x | 1.457x |
| replay_trusted_native | 0.992x | 0.989x | 0.527x |
| replay_checked_native | 0.999x | 0.996x | 0.981x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.94x).
