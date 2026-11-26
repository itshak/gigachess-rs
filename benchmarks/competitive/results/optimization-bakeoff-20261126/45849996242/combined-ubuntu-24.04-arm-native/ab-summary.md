| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.548x |
| materialize | 1.001x | 0.996x | 1.329x |
| perft_bulk_d4 | 1.000x | 0.997x | 1.389x |
| perft_full_d3 | 1.068x | 1.010x | 1.428x |
| fen_parse | 1.001x | 0.994x | 1.205x |
| fen_write_native_ep | 1.001x | 0.999x | 1.574x |
| snapshot | 1.001x | 0.999x | 0.936x |
| san_parse_all | 0.999x | 0.996x | 1.810x |
| san_render_native | 0.997x | 0.989x | 2.526x |
| apply_children_full | 0.996x | 0.991x | 1.160x |
| make_unmake_full | 1.028x | 1.001x | 1.549x |
| startpos | 1.000x | 1.000x | 1.277x |
| chess960_materialize | 0.998x | 0.998x | 1.490x |
| chess960_perft_d3 | 1.008x | 1.007x | 1.502x |
| polyglot_key | 0.976x | 0.949x | 25.625x |
| polyglot_recompute | 0.997x | 0.995x | 1.086x |
| repetition_key | 0.982x | 0.964x | 19.378x |
| corpus_count | 0.979x | 0.960x | 1.345x |
| corpus_materialize | 0.987x | 0.979x | 1.218x |
| corpus_uci_sorted | 1.002x | 1.002x | 1.454x |
| replay_trusted_native | 0.995x | 0.988x | 0.620x |
| replay_checked_native | 0.989x | 0.983x | 1.037x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.85x).
