| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.990x | 1.309x |
| materialize | 0.993x | 0.959x | 1.670x |
| perft_bulk_d4 | 1.002x | 0.984x | 1.191x |
| perft_full_d3 | 1.193x | 1.051x | 1.585x |
| fen_parse | 1.089x | 1.034x | 1.198x |
| fen_write_native_ep | 1.016x | 0.992x | 1.845x |
| snapshot | 1.000x | 0.997x | 1.000x |
| san_parse_all | 1.005x | 0.981x | 1.898x |
| san_render_native | 0.980x | 0.972x | 2.144x |
| apply_children_full | 1.004x | 0.999x | 1.365x |
| make_unmake_full | 1.033x | 0.986x | 1.243x |
| startpos | 1.000x | 1.000x | 1.357x |
| chess960_materialize | 0.951x | 0.946x | 1.494x |
| chess960_perft_d3 | 1.002x | 0.998x | 1.512x |
| polyglot_key | 1.025x | 0.999x | 44.426x |
| polyglot_recompute | 1.192x | 0.944x | 1.238x |
| repetition_key | 1.025x | 1.002x | 21.920x |
| corpus_count | 1.003x | 1.003x | 1.138x |
| corpus_materialize | 1.006x | 0.999x | 1.143x |
| corpus_uci_sorted | 0.992x | 0.985x | 1.543x |
| replay_trusted_native | 0.994x | 0.982x | 0.913x |
| replay_checked_native | 1.006x | 1.001x | 1.173x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.99x).
