| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.829x | 1.341x |
| materialize | 0.955x | 0.781x | 1.025x |
| perft_bulk_d4 | 1.188x | 1.004x | 1.347x |
| perft_full_d3 | 1.248x | 0.984x | 1.394x |
| fen_parse | 1.099x | 0.943x | 1.192x |
| fen_write_native_ep | 1.011x | 0.864x | 1.775x |
| snapshot | 1.065x | 0.666x | 0.978x |
| san_parse_all | 1.102x | 1.047x | 1.920x |
| san_render_native | 1.040x | 0.763x | 2.514x |
| apply_children_full | 1.089x | 0.918x | 0.943x |
| make_unmake_full | 1.028x | 0.923x | 1.139x |
| startpos | 0.833x | 0.833x | 1.291x |
| chess960_materialize | 1.004x | 0.972x | 1.414x |
| chess960_perft_d3 | 0.932x | 0.894x | 1.398x |
| polyglot_key | 0.859x | 0.711x | 37.718x |
| polyglot_recompute | 1.017x | 0.994x | 1.055x |
| repetition_key | 1.000x | 0.982x | 19.074x |
| corpus_count | 0.888x | 0.850x | 1.176x |
| corpus_materialize | 0.996x | 0.915x | 1.182x |
| corpus_uci_sorted | 0.802x | 0.789x | 1.311x |
| replay_trusted_native | 0.807x | 0.748x | 0.509x |
| replay_checked_native | 0.720x | 0.642x | 0.836x |

Cold first legal move list: 0.015 ms -> 0.016 ms (0.90x).
