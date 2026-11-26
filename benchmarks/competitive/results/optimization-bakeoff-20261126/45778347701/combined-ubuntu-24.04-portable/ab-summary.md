| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.972x | 1.169x |
| materialize | 0.929x | 0.898x | 1.621x |
| perft_bulk_d4 | 0.938x | 0.870x | 1.089x |
| perft_full_d3 | 1.011x | 0.961x | 1.502x |
| fen_parse | 0.898x | 0.810x | 1.072x |
| fen_write_native_ep | 0.973x | 0.953x | 1.155x |
| snapshot | 1.118x | 1.107x | 1.245x |
| san_parse_all | 0.959x | 0.928x | 1.789x |
| san_render_native | 0.972x | 0.914x | 2.061x |
| apply_children_full | 0.697x | 0.680x | 0.749x |
| make_unmake_full | 0.859x | 0.819x | 1.010x |
| startpos | 1.155x | 1.155x | 1.488x |
| chess960_materialize | 0.991x | 0.989x | 1.403x |
| chess960_perft_d3 | 0.907x | 0.858x | 1.210x |
| polyglot_key | 0.910x | 0.787x | 37.139x |
| polyglot_recompute | 0.991x | 0.964x | 1.029x |
| repetition_key | 0.831x | 0.733x | 19.850x |
| corpus_count | 1.006x | 0.996x | 1.274x |
| corpus_materialize | 1.004x | 1.002x | 1.165x |
| corpus_uci_sorted | 0.992x | 0.992x | 1.580x |
| replay_trusted_native | 0.966x | 0.934x | 0.657x |
| replay_checked_native | 0.934x | 0.933x | 1.043x |

Cold first legal move list: 0.010 ms -> 0.009 ms (1.04x).
