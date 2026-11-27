| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.998x | 1.548x |
| materialize | 1.005x | 1.000x | 1.321x |
| perft_bulk_d4 | 0.999x | 0.996x | 1.393x |
| perft_full_d3 | 1.002x | 1.000x | 1.329x |
| fen_parse | 1.002x | 1.000x | 1.228x |
| fen_write_native_ep | 0.999x | 0.996x | 1.590x |
| snapshot | 0.991x | 0.986x | 0.945x |
| san_parse_all | 0.999x | 0.989x | 1.832x |
| san_render_native | 0.999x | 0.996x | 2.697x |
| apply_children_full | 0.970x | 0.951x | 1.127x |
| make_unmake_full | 1.030x | 1.010x | 1.602x |
| startpos | 0.999x | 0.999x | 1.303x |
| chess960_materialize | 1.010x | 1.006x | 1.502x |
| chess960_perft_d3 | 1.003x | 1.000x | 1.503x |
| polyglot_key | 1.024x | 1.008x | 25.348x |
| polyglot_recompute | 0.994x | 0.991x | 1.082x |
| repetition_key | 1.012x | 1.001x | 19.027x |
| corpus_count | 1.004x | 1.003x | 1.374x |
| corpus_materialize | 0.994x | 0.993x | 1.221x |
| corpus_uci_sorted | 1.004x | 1.003x | 1.460x |
| replay_trusted_native | 1.022x | 1.015x | 0.655x |
| replay_checked_native | 0.970x | 0.959x | 1.091x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.11x).
