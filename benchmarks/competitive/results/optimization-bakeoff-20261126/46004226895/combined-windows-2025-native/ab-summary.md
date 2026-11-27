| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.038x | 0.998x | 1.589x |
| materialize | 1.039x | 1.018x | 1.146x |
| perft_bulk_d4 | 1.078x | 1.009x | 1.480x |
| perft_full_d3 | 1.345x | 1.076x | 1.704x |
| fen_parse | 0.936x | 0.915x | 1.328x |
| fen_write_native_ep | 0.983x | 0.943x | 1.544x |
| snapshot | 1.011x | 0.996x | 1.145x |
| san_parse_all | 0.980x | 0.961x | 1.900x |
| san_render_native | 1.047x | 0.993x | 3.375x |
| apply_children_full | 1.010x | 0.999x | 1.329x |
| make_unmake_full | 1.034x | 0.973x | 1.283x |
| startpos | 1.000x | 1.000x | 1.000x |
| chess960_materialize | 1.001x | 0.986x | 1.366x |
| chess960_perft_d3 | 1.007x | 1.000x | 1.603x |
| polyglot_key | 1.003x | 1.003x | 20.314x |
| polyglot_recompute | 1.030x | 1.016x | 1.006x |
| repetition_key | 0.995x | 0.994x | 19.215x |
| corpus_count | 1.064x | 1.044x | 1.381x |
| corpus_materialize | 1.009x | 1.004x | 1.200x |
| corpus_uci_sorted | 1.014x | 1.012x | 1.523x |
| replay_trusted_native | 1.034x | 0.986x | 1.013x |
| replay_checked_native | 1.002x | 0.995x | 1.110x |

Cold first legal move list: 0.014 ms -> 0.013 ms (1.09x).
