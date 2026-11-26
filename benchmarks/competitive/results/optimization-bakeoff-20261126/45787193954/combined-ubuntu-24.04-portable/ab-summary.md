| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.180x |
| materialize | 1.002x | 0.992x | 1.656x |
| perft_bulk_d4 | 1.000x | 0.985x | 1.166x |
| perft_full_d3 | 0.977x | 0.955x | 1.464x |
| fen_parse | 1.013x | 0.983x | 1.176x |
| fen_write_native_ep | 0.985x | 0.962x | 1.806x |
| snapshot | 1.118x | 1.116x | 1.001x |
| san_parse_all | 0.979x | 0.936x | 1.881x |
| san_render_native | 1.022x | 1.005x | 2.177x |
| apply_children_full | 1.007x | 0.993x | 1.133x |
| make_unmake_full | 0.996x | 0.987x | 1.197x |
| startpos | 0.999x | 0.999x | 1.286x |
| chess960_materialize | 1.039x | 1.035x | 1.325x |
| chess960_perft_d3 | 0.999x | 0.997x | 1.335x |
| polyglot_key | 0.999x | 0.999x | 43.084x |
| polyglot_recompute | 0.935x | 0.912x | 1.028x |
| repetition_key | 0.871x | 0.664x | 18.763x |
| corpus_count | 1.002x | 1.000x | 1.265x |
| corpus_materialize | 0.992x | 0.991x | 1.127x |
| corpus_uci_sorted | 1.011x | 1.010x | 1.568x |
| replay_trusted_native | 1.008x | 1.004x | 0.703x |
| replay_checked_native | 0.997x | 0.990x | 1.112x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.97x).
