| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.999x | 1.199x |
| materialize | 0.996x | 0.983x | 1.667x |
| perft_bulk_d4 | 0.997x | 0.989x | 1.166x |
| perft_full_d3 | 1.005x | 0.978x | 1.521x |
| fen_parse | 0.993x | 0.980x | 1.171x |
| fen_write_native_ep | 1.002x | 0.993x | 1.883x |
| snapshot | 0.895x | 0.889x | 1.000x |
| san_parse_all | 1.003x | 0.994x | 1.895x |
| san_render_native | 1.002x | 0.999x | 2.200x |
| apply_children_full | 0.998x | 0.989x | 1.100x |
| make_unmake_full | 1.001x | 0.989x | 1.192x |
| startpos | 1.003x | 1.003x | 1.371x |
| chess960_materialize | 1.066x | 1.063x | 1.436x |
| chess960_perft_d3 | 0.997x | 0.997x | 1.334x |
| polyglot_key | 1.000x | 0.997x | 41.070x |
| polyglot_recompute | 1.007x | 1.000x | 1.030x |
| repetition_key | 1.110x | 0.979x | 23.976x |
| corpus_count | 1.001x | 0.999x | 1.279x |
| corpus_materialize | 1.010x | 1.009x | 1.166x |
| corpus_uci_sorted | 0.986x | 0.978x | 1.545x |
| replay_trusted_native | 0.995x | 0.983x | 0.712x |
| replay_checked_native | 0.909x | 0.864x | 1.008x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.88x).
