| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.991x | 1.283x |
| materialize | 0.964x | 0.933x | 1.702x |
| perft_bulk_d4 | 0.994x | 0.981x | 1.183x |
| perft_full_d3 | 1.005x | 1.000x | 1.329x |
| fen_parse | 1.004x | 0.991x | 1.072x |
| fen_write_native_ep | 1.014x | 0.997x | 1.863x |
| snapshot | 1.002x | 0.992x | 1.005x |
| san_parse_all | 0.994x | 0.987x | 1.885x |
| san_render_native | 1.017x | 1.008x | 2.143x |
| apply_children_full | 0.998x | 0.950x | 1.362x |
| make_unmake_full | 1.041x | 0.983x | 1.227x |
| startpos | 0.991x | 0.991x | 1.340x |
| chess960_materialize | 1.003x | 0.991x | 1.562x |
| chess960_perft_d3 | 1.005x | 1.003x | 1.513x |
| polyglot_key | 1.002x | 0.994x | 41.242x |
| polyglot_recompute | 1.038x | 1.017x | 0.986x |
| repetition_key | 1.054x | 0.850x | 21.872x |
| corpus_count | 1.036x | 1.000x | 1.170x |
| corpus_materialize | 0.999x | 0.996x | 1.133x |
| corpus_uci_sorted | 0.997x | 0.991x | 1.566x |
| replay_trusted_native | 0.929x | 0.901x | 0.817x |
| replay_checked_native | 1.011x | 0.993x | 1.209x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.99x).
