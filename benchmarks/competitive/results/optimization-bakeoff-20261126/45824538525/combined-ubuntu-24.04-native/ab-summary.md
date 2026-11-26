| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.994x | 1.371x |
| materialize | 0.991x | 0.934x | 1.505x |
| perft_bulk_d4 | 1.015x | 0.987x | 1.021x |
| perft_full_d3 | 0.976x | 0.933x | 1.433x |
| fen_parse | 1.040x | 0.979x | 1.126x |
| fen_write_native_ep | 1.007x | 1.001x | 1.922x |
| snapshot | 1.000x | 1.000x | 1.154x |
| san_parse_all | 1.005x | 0.990x | 1.907x |
| san_render_native | 1.015x | 1.007x | 2.132x |
| apply_children_full | 1.052x | 1.043x | 1.357x |
| make_unmake_full | 0.938x | 0.900x | 1.098x |
| startpos | 1.119x | 1.119x | 1.118x |
| chess960_materialize | 0.979x | 0.948x | 1.403x |
| chess960_perft_d3 | 0.989x | 0.983x | 1.403x |
| polyglot_key | 1.004x | 1.000x | 40.032x |
| polyglot_recompute | 1.001x | 0.993x | 1.031x |
| repetition_key | 0.894x | 0.782x | 17.137x |
| corpus_count | 1.000x | 0.998x | 0.856x |
| corpus_materialize | 1.002x | 1.000x | 1.027x |
| corpus_uci_sorted | 1.004x | 1.003x | 1.546x |
| replay_trusted_native | 0.971x | 0.930x | 0.845x |
| replay_checked_native | 1.018x | 0.988x | 1.112x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.02x).
