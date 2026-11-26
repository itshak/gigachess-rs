| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.968x | 0.898x | 1.501x |
| materialize | 0.984x | 0.952x | 1.309x |
| perft_bulk_d4 | 0.979x | 0.939x | 1.367x |
| perft_full_d3 | 0.994x | 0.979x | 1.332x |
| fen_parse | 0.996x | 0.991x | 1.215x |
| fen_write_native_ep | 1.002x | 0.999x | 1.583x |
| snapshot | 1.020x | 1.015x | 0.937x |
| san_parse_all | 1.000x | 0.994x | 1.817x |
| san_render_native | 1.003x | 0.996x | 2.681x |
| apply_children_full | 1.005x | 0.984x | 1.158x |
| make_unmake_full | 1.010x | 0.998x | 1.569x |
| startpos | 1.001x | 1.001x | 1.278x |
| chess960_materialize | 0.989x | 0.986x | 1.482x |
| chess960_perft_d3 | 0.994x | 0.984x | 1.500x |
| polyglot_key | 1.011x | 1.007x | 25.479x |
| polyglot_recompute | 0.999x | 0.998x | 1.090x |
| repetition_key | 1.012x | 0.998x | 19.110x |
| corpus_count | 0.918x | 0.914x | 1.250x |
| corpus_materialize | 0.976x | 0.974x | 1.204x |
| corpus_uci_sorted | 0.995x | 0.993x | 1.453x |
| replay_trusted_native | 1.006x | 1.005x | 0.636x |
| replay_checked_native | 1.001x | 0.994x | 1.136x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.97x).
