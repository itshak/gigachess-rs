| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.995x | 1.484x |
| materialize | 0.993x | 0.979x | 1.562x |
| perft_bulk_d4 | 1.004x | 0.995x | 1.265x |
| perft_full_d3 | 1.027x | 1.010x | 1.549x |
| fen_parse | 1.041x | 1.011x | 1.129x |
| fen_write_native_ep | 0.955x | 0.914x | 1.824x |
| snapshot | 0.499x | 0.498x | 1.002x |
| san_parse_all | 0.947x | 0.926x | 1.790x |
| san_render_native | 1.018x | 1.013x | 2.201x |
| apply_children_full | 0.911x | 0.892x | 1.075x |
| make_unmake_full | 0.986x | 0.970x | 1.141x |
| startpos | 0.743x | 0.743x | 1.331x |
| chess960_materialize | 1.071x | 1.069x | 1.458x |
| chess960_perft_d3 | 0.996x | 0.994x | 1.388x |
| polyglot_key | 1.012x | 0.999x | 39.405x |
| polyglot_recompute | 1.007x | 1.002x | 1.014x |
| repetition_key | 1.139x | 0.977x | 21.572x |
| corpus_count | 1.002x | 0.999x | 1.414x |
| corpus_materialize | 0.991x | 0.986x | 1.100x |
| corpus_uci_sorted | 0.997x | 0.994x | 1.595x |
| replay_trusted_native | 1.002x | 1.000x | 0.698x |
| replay_checked_native | 0.986x | 0.976x | 1.094x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.11x).
