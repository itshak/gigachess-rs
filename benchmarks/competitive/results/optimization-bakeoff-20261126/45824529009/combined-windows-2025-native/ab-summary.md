| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.998x | 1.315x |
| materialize | 1.014x | 0.984x | 1.178x |
| perft_bulk_d4 | 1.006x | 0.989x | 1.203x |
| perft_full_d3 | 0.966x | 0.912x | 1.210x |
| fen_parse | 1.022x | 1.000x | 1.280x |
| fen_write_native_ep | 1.010x | 0.973x | 1.556x |
| snapshot | 1.009x | 0.999x | 1.002x |
| san_parse_all | 1.002x | 0.994x | 2.060x |
| san_render_native | 0.999x | 0.994x | 3.846x |
| apply_children_full | 0.979x | 0.946x | 1.325x |
| make_unmake_full | 1.007x | 0.966x | 1.349x |
| startpos | 1.001x | 1.001x | 1.247x |
| chess960_materialize | 1.011x | 1.005x | 1.283x |
| chess960_perft_d3 | 0.998x | 0.991x | 1.412x |
| polyglot_key | 0.991x | 0.982x | 24.866x |
| polyglot_recompute | 1.003x | 1.000x | 1.016x |
| repetition_key | 1.201x | 1.011x | 18.592x |
| corpus_count | 1.076x | 1.000x | 1.229x |
| corpus_materialize | 1.070x | 0.984x | 1.101x |
| corpus_uci_sorted | 1.012x | 1.009x | 1.411x |
| replay_trusted_native | 0.998x | 0.990x | 0.833x |
| replay_checked_native | 0.980x | 0.977x | 0.976x |

Cold first legal move list: 0.017 ms -> 0.018 ms (0.95x).
