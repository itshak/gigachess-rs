| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.984x | 1.537x |
| materialize | 1.020x | 1.007x | 1.597x |
| perft_bulk_d4 | 1.000x | 0.992x | 1.193x |
| perft_full_d3 | 1.002x | 0.999x | 1.294x |
| fen_parse | 0.995x | 0.984x | 1.034x |
| fen_write_native_ep | 1.000x | 0.989x | 2.160x |
| snapshot | 1.000x | 0.999x | 1.330x |
| san_parse_all | 0.996x | 0.988x | 1.894x |
| san_render_native | 1.015x | 1.010x | 2.213x |
| apply_children_full | 1.002x | 1.000x | 1.222x |
| make_unmake_full | 1.004x | 0.998x | 1.172x |
| startpos | 1.117x | 1.117x | 1.250x |
| chess960_materialize | 1.007x | 0.996x | 1.535x |
| chess960_perft_d3 | 1.011x | 1.008x | 1.482x |
| polyglot_key | 0.932x | 0.868x | 36.018x |
| polyglot_recompute | 1.010x | 1.009x | 0.971x |
| repetition_key | 0.961x | 0.917x | 18.239x |
| corpus_count | 0.994x | 0.972x | 1.053x |
| corpus_materialize | 1.015x | 1.009x | 1.100x |
| corpus_uci_sorted | 0.980x | 0.977x | 1.557x |
| replay_trusted_native | 0.999x | 0.998x | 0.917x |
| replay_checked_native | 1.002x | 0.995x | 1.042x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.00x).
