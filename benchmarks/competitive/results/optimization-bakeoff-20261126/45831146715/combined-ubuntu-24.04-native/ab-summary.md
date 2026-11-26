| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.957x | 1.372x |
| materialize | 1.005x | 0.995x | 1.542x |
| perft_bulk_d4 | 0.994x | 0.979x | 1.008x |
| perft_full_d3 | 0.951x | 0.895x | 1.488x |
| fen_parse | 1.053x | 0.984x | 1.085x |
| fen_write_native_ep | 1.005x | 0.966x | 2.152x |
| snapshot | 0.995x | 0.982x | 1.322x |
| san_parse_all | 1.010x | 0.990x | 1.950x |
| san_render_native | 0.974x | 0.953x | 2.096x |
| apply_children_full | 0.915x | 0.842x | 1.184x |
| make_unmake_full | 0.876x | 0.825x | 1.026x |
| startpos | 0.894x | 0.894x | 1.003x |
| chess960_materialize | 0.959x | 0.901x | 1.376x |
| chess960_perft_d3 | 0.980x | 0.938x | 1.362x |
| polyglot_key | 1.170x | 0.997x | 41.180x |
| polyglot_recompute | 0.996x | 0.991x | 1.051x |
| repetition_key | 0.912x | 0.712x | 20.067x |
| corpus_count | 0.987x | 0.979x | 0.875x |
| corpus_materialize | 1.003x | 1.001x | 1.041x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.541x |
| replay_trusted_native | 0.846x | 0.773x | 0.768x |
| replay_checked_native | 0.935x | 0.888x | 1.047x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.89x).
