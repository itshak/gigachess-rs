| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.996x | 1.354x |
| materialize | 0.998x | 0.993x | 1.230x |
| perft_bulk_d4 | 1.000x | 0.997x | 1.345x |
| perft_full_d3 | 1.000x | 0.999x | 1.052x |
| fen_parse | 0.984x | 0.976x | 1.214x |
| fen_write_native_ep | 0.988x | 0.979x | 1.555x |
| snapshot | 1.000x | 0.998x | 0.895x |
| san_parse_all | 1.001x | 0.995x | 1.805x |
| san_render_native | 0.998x | 0.991x | 2.528x |
| apply_children_full | 0.995x | 0.991x | 1.020x |
| make_unmake_full | 0.998x | 0.994x | 1.439x |
| startpos | 1.001x | 1.001x | 1.187x |
| chess960_materialize | 0.999x | 0.997x | 1.321x |
| chess960_perft_d3 | 1.001x | 1.000x | 1.330x |
| polyglot_key | 0.999x | 0.994x | 25.379x |
| polyglot_recompute | 0.961x | 0.957x | 1.029x |
| repetition_key | 0.989x | 0.980x | 18.894x |
| corpus_count | 0.996x | 0.996x | 1.474x |
| corpus_materialize | 0.994x | 0.994x | 1.201x |
| corpus_uci_sorted | 0.992x | 0.989x | 1.454x |
| replay_trusted_native | 1.005x | 1.004x | 0.512x |
| replay_checked_native | 1.002x | 1.000x | 0.993x |

Cold first legal move list: 0.009 ms -> 0.011 ms (0.82x).
