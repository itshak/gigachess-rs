| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.354x |
| materialize | 1.008x | 1.004x | 1.244x |
| perft_bulk_d4 | 1.002x | 0.999x | 1.355x |
| perft_full_d3 | 1.034x | 1.022x | 1.093x |
| fen_parse | 0.998x | 0.994x | 1.230x |
| fen_write_native_ep | 1.006x | 0.992x | 1.053x |
| snapshot | 0.982x | 0.979x | 0.936x |
| san_parse_all | 0.970x | 0.965x | 1.761x |
| san_render_native | 0.999x | 0.983x | 2.538x |
| apply_children_full | 1.112x | 1.072x | 1.140x |
| make_unmake_full | 1.006x | 0.983x | 1.425x |
| startpos | 1.113x | 1.113x | 1.274x |
| chess960_materialize | 1.001x | 0.997x | 1.332x |
| chess960_perft_d3 | 1.003x | 0.999x | 1.321x |
| polyglot_key | 1.000x | 0.997x | 25.288x |
| polyglot_recompute | 0.999x | 0.998x | 1.077x |
| repetition_key | 0.966x | 0.935x | 18.329x |
| corpus_count | 0.986x | 0.975x | 1.432x |
| corpus_materialize | 1.010x | 1.007x | 1.214x |
| corpus_uci_sorted | 1.000x | 0.997x | 1.457x |
| replay_trusted_native | 1.007x | 1.001x | 0.516x |
| replay_checked_native | 0.966x | 0.959x | 0.957x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.03x).
