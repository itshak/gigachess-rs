| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.516x |
| materialize | 1.047x | 1.025x | 1.185x |
| perft_bulk_d4 | 1.007x | 0.983x | 1.363x |
| perft_full_d3 | 0.994x | 0.974x | 1.216x |
| fen_parse | 1.004x | 0.978x | 1.188x |
| fen_write_native_ep | 0.992x | 0.982x | 1.854x |
| snapshot | 0.999x | 0.995x | 1.248x |
| san_parse_all | 0.995x | 0.919x | 2.272x |
| san_render_native | 0.995x | 0.980x | 2.094x |
| apply_children_full | 1.004x | 0.977x | 1.300x |
| make_unmake_full | 0.972x | 0.927x | 1.192x |
| startpos | 0.998x | 0.998x | 1.199x |
| chess960_materialize | 1.002x | 1.002x | 1.553x |
| chess960_perft_d3 | 1.003x | 0.999x | 1.610x |
| polyglot_key | 0.993x | 0.984x | 20.514x |
| polyglot_recompute | 0.993x | 0.852x | 1.014x |
| repetition_key | 0.999x | 0.998x | 19.098x |
| corpus_count | 1.001x | 0.999x | 1.304x |
| corpus_materialize | 0.998x | 0.996x | 1.232x |
| corpus_uci_sorted | 0.973x | 0.962x | 1.667x |
| replay_trusted_native | 1.083x | 1.013x | 1.194x |
| replay_checked_native | 1.005x | 1.000x | 1.125x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.88x).
