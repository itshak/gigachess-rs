| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.979x | 1.565x |
| materialize | 1.012x | 1.001x | 1.350x |
| perft_bulk_d4 | 1.042x | 1.024x | 1.450x |
| perft_full_d3 | 1.136x | 1.011x | 1.516x |
| fen_parse | 0.986x | 0.978x | 1.206x |
| fen_write_native_ep | 1.001x | 0.999x | 1.577x |
| snapshot | 1.009x | 1.007x | 0.936x |
| san_parse_all | 0.999x | 0.993x | 1.812x |
| san_render_native | 1.003x | 0.994x | 2.692x |
| apply_children_full | 1.005x | 0.998x | 1.164x |
| make_unmake_full | 0.991x | 0.985x | 1.538x |
| startpos | 0.999x | 0.999x | 1.274x |
| chess960_materialize | 1.007x | 1.006x | 1.503x |
| chess960_perft_d3 | 1.017x | 1.003x | 1.522x |
| polyglot_key | 0.998x | 0.978x | 25.470x |
| polyglot_recompute | 0.992x | 0.989x | 1.084x |
| repetition_key | 1.004x | 0.999x | 19.258x |
| corpus_count | 1.092x | 1.064x | 1.519x |
| corpus_materialize | 1.006x | 1.006x | 1.253x |
| corpus_uci_sorted | 1.002x | 0.998x | 1.460x |
| replay_trusted_native | 1.001x | 0.998x | 0.634x |
| replay_checked_native | 1.013x | 1.009x | 1.141x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.93x).
