| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.960x | 1.175x |
| materialize | 1.012x | 0.999x | 1.411x |
| perft_bulk_d4 | 1.004x | 0.985x | 0.875x |
| perft_full_d3 | 0.999x | 0.989x | 0.663x |
| fen_parse | 0.895x | 0.856x | 1.041x |
| fen_write_native_ep | 0.997x | 0.991x | 1.233x |
| snapshot | 0.597x | 0.593x | 0.732x |
| san_parse_all | 0.997x | 0.988x | 0.908x |
| san_render_native | 0.999x | 0.992x | 1.054x |
| apply_children_full | 0.936x | 0.921x | 0.673x |
| make_unmake_full | 0.987x | 0.970x | 0.761x |
| startpos | 0.944x | 0.944x | 0.788x |
| chess960_materialize | 0.953x | 0.937x | 1.109x |
| chess960_perft_d3 | 0.983x | 0.979x | 0.933x |
| polyglot_key | 1.010x | 0.998x | 36.491x |
| polyglot_recompute | 0.943x | 0.933x | 0.961x |
| repetition_key | 1.155x | 1.033x | 16.473x |
| corpus_count | 1.017x | 1.001x | 1.182x |
| corpus_materialize | 1.010x | 1.006x | 1.090x |
| corpus_uci_sorted | 0.994x | 0.989x | 0.617x |
| replay_trusted_native | 1.034x | 1.027x | 0.520x |
| replay_checked_native | 0.992x | 0.977x | 0.735x |

Cold first legal move list: 139.423 ms → 140.328 ms (0.99x).
