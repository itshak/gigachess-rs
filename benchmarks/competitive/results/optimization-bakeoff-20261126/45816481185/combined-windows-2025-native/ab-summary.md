| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.023x | 1.003x | 1.329x |
| materialize | 1.010x | 0.998x | 1.197x |
| perft_bulk_d4 | 0.990x | 0.979x | 1.242x |
| perft_full_d3 | 0.981x | 0.851x | 1.253x |
| fen_parse | 1.067x | 1.029x | 1.343x |
| fen_write_native_ep | 0.999x | 0.982x | 1.614x |
| snapshot | 1.001x | 0.984x | 0.997x |
| san_parse_all | 1.036x | 1.018x | 2.137x |
| san_render_native | 1.007x | 0.985x | 3.848x |
| apply_children_full | 0.992x | 0.981x | 1.267x |
| make_unmake_full | 1.007x | 1.003x | 1.264x |
| startpos | 1.003x | 1.003x | 1.253x |
| chess960_materialize | 1.009x | 0.995x | 1.278x |
| chess960_perft_d3 | 1.004x | 0.997x | 1.423x |
| polyglot_key | 0.997x | 0.991x | 25.562x |
| polyglot_recompute | 0.974x | 0.970x | 0.994x |
| repetition_key | 0.984x | 0.972x | 18.545x |
| corpus_count | 1.003x | 1.000x | 1.251x |
| corpus_materialize | 0.987x | 0.986x | 1.066x |
| corpus_uci_sorted | 0.981x | 0.973x | 1.386x |
| replay_trusted_native | 1.011x | 1.004x | 0.743x |
| replay_checked_native | 0.996x | 0.996x | 1.005x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.01x).
