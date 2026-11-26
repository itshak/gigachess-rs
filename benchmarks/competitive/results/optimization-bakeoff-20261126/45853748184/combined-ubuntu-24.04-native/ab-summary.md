| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.015x | 0.993x | 1.500x |
| materialize | 1.004x | 0.993x | 1.484x |
| perft_bulk_d4 | 0.993x | 0.981x | 1.157x |
| perft_full_d3 | 1.000x | 0.996x | 1.362x |
| fen_parse | 1.015x | 1.008x | 1.093x |
| fen_write_native_ep | 1.005x | 0.985x | 2.112x |
| snapshot | 0.999x | 0.999x | 1.155x |
| san_parse_all | 1.003x | 0.996x | 1.673x |
| san_render_native | 1.016x | 1.011x | 2.132x |
| apply_children_full | 0.964x | 0.948x | 1.254x |
| make_unmake_full | 0.937x | 0.870x | 1.098x |
| startpos | 1.119x | 1.119x | 1.118x |
| chess960_materialize | 0.973x | 0.954x | 1.320x |
| chess960_perft_d3 | 1.000x | 0.996x | 1.503x |
| polyglot_key | 1.007x | 0.999x | 39.936x |
| polyglot_recompute | 0.995x | 0.991x | 0.986x |
| repetition_key | 0.783x | 0.763x | 24.434x |
| corpus_count | 0.996x | 0.993x | 0.942x |
| corpus_materialize | 0.994x | 0.992x | 1.047x |
| corpus_uci_sorted | 1.009x | 1.009x | 1.563x |
| replay_trusted_native | 0.996x | 0.980x | 0.863x |
| replay_checked_native | 0.991x | 0.990x | 1.067x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.01x).
