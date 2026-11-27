| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.043x | 1.010x | 1.508x |
| materialize | 1.015x | 0.999x | 1.258x |
| perft_bulk_d4 | 1.038x | 1.023x | 1.460x |
| perft_full_d3 | 1.231x | 1.036x | 1.571x |
| fen_parse | 1.050x | 1.015x | 1.175x |
| fen_write_native_ep | 0.999x | 0.975x | 1.746x |
| snapshot | 0.723x | 0.721x | 0.902x |
| san_parse_all | 0.980x | 0.960x | 2.012x |
| san_render_native | 0.997x | 0.988x | 2.203x |
| apply_children_full | 1.009x | 0.997x | 1.335x |
| make_unmake_full | 1.008x | 0.971x | 1.283x |
| startpos | 1.001x | 1.001x | 1.253x |
| chess960_materialize | 0.997x | 0.978x | 1.408x |
| chess960_perft_d3 | 0.993x | 0.988x | 1.517x |
| polyglot_key | 1.000x | 0.999x | 26.074x |
| polyglot_recompute | 0.976x | 0.971x | 1.017x |
| repetition_key | 0.977x | 0.964x | 19.387x |
| corpus_count | 1.067x | 1.041x | 1.482x |
| corpus_materialize | 1.011x | 1.007x | 1.146x |
| corpus_uci_sorted | 0.996x | 0.994x | 1.697x |
| replay_trusted_native | 0.959x | 0.958x | 0.858x |
| replay_checked_native | 0.978x | 0.970x | 1.104x |

Cold first legal move list: 0.007 ms -> 0.009 ms (0.86x).
