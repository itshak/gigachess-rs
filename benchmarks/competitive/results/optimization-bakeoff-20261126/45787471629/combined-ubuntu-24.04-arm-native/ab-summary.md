| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.356x |
| materialize | 1.000x | 0.997x | 1.236x |
| perft_bulk_d4 | 0.999x | 0.998x | 1.353x |
| perft_full_d3 | 1.002x | 0.994x | 1.070x |
| fen_parse | 1.001x | 0.994x | 1.247x |
| fen_write_native_ep | 1.001x | 0.998x | 1.587x |
| snapshot | 1.021x | 1.015x | 0.910x |
| san_parse_all | 0.999x | 0.995x | 1.835x |
| san_render_native | 1.000x | 0.998x | 2.538x |
| apply_children_full | 1.010x | 1.007x | 1.050x |
| make_unmake_full | 0.999x | 0.993x | 1.433x |
| startpos | 1.073x | 1.073x | 1.185x |
| chess960_materialize | 0.995x | 0.993x | 1.325x |
| chess960_perft_d3 | 0.995x | 0.994x | 1.320x |
| polyglot_key | 1.006x | 1.003x | 25.814x |
| polyglot_recompute | 0.991x | 0.987x | 1.064x |
| repetition_key | 1.000x | 0.998x | 19.043x |
| corpus_count | 1.007x | 1.002x | 1.488x |
| corpus_materialize | 1.001x | 0.995x | 1.202x |
| corpus_uci_sorted | 0.994x | 0.994x | 1.444x |
| replay_trusted_native | 0.982x | 0.966x | 0.496x |
| replay_checked_native | 0.926x | 0.915x | 0.919x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.08x).
