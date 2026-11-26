| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.982x | 1.457x |
| materialize | 1.003x | 0.994x | 1.247x |
| perft_bulk_d4 | 0.999x | 0.978x | 1.378x |
| perft_full_d3 | 0.998x | 0.990x | 1.279x |
| fen_parse | 1.122x | 1.087x | 1.234x |
| fen_write_native_ep | 0.999x | 0.978x | 1.727x |
| snapshot | 0.999x | 0.994x | 1.247x |
| san_parse_all | 0.990x | 0.979x | 2.010x |
| san_render_native | 0.999x | 0.981x | 2.216x |
| apply_children_full | 0.995x | 0.974x | 1.311x |
| make_unmake_full | 1.008x | 0.972x | 1.279x |
| startpos | 0.999x | 0.999x | 1.249x |
| chess960_materialize | 1.003x | 0.994x | 1.442x |
| chess960_perft_d3 | 0.996x | 0.992x | 1.514x |
| polyglot_key | 1.078x | 0.998x | 30.088x |
| polyglot_recompute | 1.003x | 0.998x | 1.039x |
| repetition_key | 1.029x | 1.013x | 21.059x |
| corpus_count | 1.017x | 1.008x | 1.390x |
| corpus_materialize | 1.008x | 1.007x | 1.148x |
| corpus_uci_sorted | 1.001x | 1.001x | 1.765x |
| replay_trusted_native | 0.961x | 0.934x | 0.869x |
| replay_checked_native | 0.961x | 0.937x | 1.056x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.04x).
