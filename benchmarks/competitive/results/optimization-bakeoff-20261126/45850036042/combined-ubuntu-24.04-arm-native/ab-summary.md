| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.548x |
| materialize | 0.998x | 0.994x | 1.318x |
| perft_bulk_d4 | 1.002x | 0.997x | 1.398x |
| perft_full_d3 | 0.992x | 0.975x | 1.325x |
| fen_parse | 1.011x | 0.999x | 1.224x |
| fen_write_native_ep | 1.002x | 0.998x | 1.590x |
| snapshot | 1.008x | 1.004x | 0.936x |
| san_parse_all | 0.995x | 0.989x | 1.815x |
| san_render_native | 0.996x | 0.973x | 2.524x |
| apply_children_full | 1.001x | 0.993x | 1.160x |
| make_unmake_full | 1.014x | 0.998x | 1.527x |
| startpos | 1.001x | 1.001x | 1.277x |
| chess960_materialize | 0.999x | 0.998x | 1.489x |
| chess960_perft_d3 | 1.005x | 0.995x | 1.506x |
| polyglot_key | 1.004x | 1.002x | 25.305x |
| polyglot_recompute | 0.994x | 0.994x | 1.078x |
| repetition_key | 1.000x | 0.999x | 18.890x |
| corpus_count | 1.003x | 0.996x | 1.377x |
| corpus_materialize | 1.007x | 0.999x | 1.230x |
| corpus_uci_sorted | 1.004x | 1.000x | 1.462x |
| replay_trusted_native | 1.005x | 1.000x | 0.625x |
| replay_checked_native | 1.003x | 0.998x | 1.051x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.10x).
