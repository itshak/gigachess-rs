| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.985x | 1.563x |
| materialize | 1.017x | 1.003x | 1.355x |
| perft_bulk_d4 | 1.041x | 1.020x | 1.453x |
| perft_full_d3 | 1.131x | 1.003x | 1.512x |
| fen_parse | 1.004x | 1.001x | 1.225x |
| fen_write_native_ep | 0.993x | 0.980x | 1.605x |
| snapshot | 1.001x | 1.000x | 0.947x |
| san_parse_all | 0.999x | 0.995x | 1.865x |
| san_render_native | 0.997x | 0.983x | 2.667x |
| apply_children_full | 0.992x | 0.983x | 1.151x |
| make_unmake_full | 1.007x | 0.998x | 1.570x |
| startpos | 0.999x | 0.999x | 1.304x |
| chess960_materialize | 1.008x | 1.007x | 1.507x |
| chess960_perft_d3 | 1.015x | 1.000x | 1.522x |
| polyglot_key | 0.998x | 0.990x | 25.301x |
| polyglot_recompute | 0.998x | 0.996x | 1.087x |
| repetition_key | 1.008x | 0.999x | 19.052x |
| corpus_count | 1.088x | 1.053x | 1.494x |
| corpus_materialize | 0.994x | 0.994x | 1.245x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.459x |
| replay_trusted_native | 0.990x | 0.982x | 0.634x |
| replay_checked_native | 1.021x | 1.011x | 1.155x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.00x).
