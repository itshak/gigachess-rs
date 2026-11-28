| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.056x | 0.998x | 1.367x |
| materialize | 1.001x | 0.993x | 1.744x |
| perft_bulk_d4 | 1.068x | 1.045x | 1.267x |
| perft_full_d3 | 1.181x | 1.041x | 1.585x |
| fen_parse | 1.008x | 0.963x | 1.213x |
| fen_write_native_ep | 0.998x | 0.990x | 1.914x |
| snapshot | 1.003x | 0.998x | 1.002x |
| san_parse_all | 1.007x | 1.002x | 1.933x |
| san_render_native | 0.976x | 0.962x | 2.130x |
| apply_children_full | 0.983x | 0.956x | 1.356x |
| make_unmake_full | 0.966x | 0.922x | 1.250x |
| startpos | 0.999x | 0.999x | 1.355x |
| chess960_materialize | 0.987x | 0.982x | 1.553x |
| chess960_perft_d3 | 1.012x | 0.999x | 1.515x |
| polyglot_key | 0.979x | 0.956x | 40.634x |
| polyglot_recompute | 1.614x | 1.298x | 1.607x |
| repetition_key | 0.895x | 0.747x | 21.192x |
| corpus_count | 1.145x | 1.120x | 1.341x |
| corpus_materialize | 0.998x | 0.996x | 1.128x |
| corpus_uci_sorted | 1.004x | 0.998x | 1.545x |
| replay_trusted_native | 1.006x | 0.899x | 0.936x |
| replay_checked_native | 1.010x | 0.983x | 1.212x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.81x).
