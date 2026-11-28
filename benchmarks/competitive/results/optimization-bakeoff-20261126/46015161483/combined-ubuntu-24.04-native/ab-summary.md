| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.038x | 1.021x | 1.538x |
| materialize | 1.013x | 0.993x | 1.189x |
| perft_bulk_d4 | 1.020x | 0.986x | 1.402x |
| perft_full_d3 | 1.320x | 1.095x | 1.602x |
| fen_parse | 1.024x | 0.850x | 1.190x |
| fen_write_native_ep | 1.013x | 1.004x | 1.848x |
| snapshot | 1.007x | 0.998x | 1.250x |
| san_parse_all | 1.003x | 0.985x | 1.955x |
| san_render_native | 0.994x | 0.971x | 2.131x |
| apply_children_full | 0.991x | 0.969x | 1.323x |
| make_unmake_full | 1.018x | 0.976x | 1.224x |
| startpos | 1.001x | 1.001x | 1.200x |
| chess960_materialize | 0.995x | 0.977x | 1.486x |
| chess960_perft_d3 | 1.005x | 1.003x | 1.608x |
| polyglot_key | 1.001x | 0.999x | 19.737x |
| polyglot_recompute | 1.182x | 1.034x | 1.179x |
| repetition_key | 1.000x | 0.996x | 18.291x |
| corpus_count | 1.073x | 1.051x | 1.398x |
| corpus_materialize | 1.021x | 1.011x | 1.216x |
| corpus_uci_sorted | 0.998x | 0.996x | 1.821x |
| replay_trusted_native | 1.018x | 1.009x | 1.127x |
| replay_checked_native | 1.017x | 1.006x | 1.098x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.00x).
