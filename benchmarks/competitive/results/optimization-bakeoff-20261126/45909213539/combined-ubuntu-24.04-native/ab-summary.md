| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.996x | 1.499x |
| materialize | 1.025x | 1.001x | 1.576x |
| perft_bulk_d4 | 1.004x | 1.000x | 1.168x |
| perft_full_d3 | 1.008x | 1.001x | 1.358x |
| fen_parse | 0.999x | 0.988x | 1.175x |
| fen_write_native_ep | 0.998x | 0.994x | 1.919x |
| snapshot | 0.998x | 0.993x | 0.998x |
| san_parse_all | 1.002x | 0.999x | 1.876x |
| san_render_native | 0.986x | 0.979x | 2.084x |
| apply_children_full | 1.000x | 0.995x | 1.353x |
| make_unmake_full | 0.999x | 0.994x | 1.195x |
| startpos | 0.893x | 0.893x | 0.998x |
| chess960_materialize | 0.995x | 0.994x | 1.522x |
| chess960_perft_d3 | 0.999x | 0.998x | 1.505x |
| polyglot_key | 0.977x | 0.954x | 39.012x |
| polyglot_recompute | 1.001x | 0.999x | 1.009x |
| repetition_key | 1.002x | 0.999x | 19.705x |
| corpus_count | 0.994x | 0.989x | 1.080x |
| corpus_materialize | 0.996x | 0.993x | 1.038x |
| corpus_uci_sorted | 1.001x | 0.997x | 1.565x |
| replay_trusted_native | 1.002x | 0.999x | 0.857x |
| replay_checked_native | 0.999x | 0.996x | 1.119x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.01x).
