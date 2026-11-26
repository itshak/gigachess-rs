| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.017x | 0.999x | 1.520x |
| materialize | 0.984x | 0.949x | 1.545x |
| perft_bulk_d4 | 1.005x | 0.988x | 1.172x |
| perft_full_d3 | 1.179x | 1.061x | 1.637x |
| fen_parse | 0.995x | 0.955x | 1.146x |
| fen_write_native_ep | 1.004x | 1.001x | 1.916x |
| snapshot | 1.004x | 0.998x | 1.150x |
| san_parse_all | 1.037x | 1.024x | 1.937x |
| san_render_native | 1.001x | 0.991x | 2.139x |
| apply_children_full | 1.015x | 1.004x | 1.351x |
| make_unmake_full | 1.018x | 0.977x | 1.215x |
| startpos | 1.010x | 1.010x | 1.119x |
| chess960_materialize | 1.011x | 0.994x | 1.547x |
| chess960_perft_d3 | 1.019x | 1.008x | 1.533x |
| polyglot_key | 1.026x | 1.000x | 39.088x |
| polyglot_recompute | 1.067x | 1.018x | 1.070x |
| repetition_key | 0.983x | 0.972x | 19.075x |
| corpus_count | 1.037x | 1.018x | 1.128x |
| corpus_materialize | 1.017x | 1.016x | 1.077x |
| corpus_uci_sorted | 1.015x | 1.013x | 1.600x |
| replay_trusted_native | 0.867x | 0.866x | 0.740x |
| replay_checked_native | 0.937x | 0.910x | 1.118x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.05x).
