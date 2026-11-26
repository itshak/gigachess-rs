| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.161x |
| materialize | 0.962x | 0.924x | 1.233x |
| perft_bulk_d4 | 0.993x | 0.982x | 1.173x |
| perft_full_d3 | 1.019x | 1.001x | 1.140x |
| fen_parse | 1.003x | 0.987x | 1.267x |
| fen_write_native_ep | 0.994x | 0.954x | 1.187x |
| snapshot | 1.000x | 0.998x | 0.928x |
| san_parse_all | 0.994x | 0.982x | 2.191x |
| san_render_native | 1.043x | 1.014x | 2.386x |
| apply_children_full | 0.995x | 0.975x | 1.095x |
| make_unmake_full | 1.009x | 0.970x | 1.272x |
| startpos | 1.000x | 1.000x | 1.311x |
| chess960_materialize | 1.009x | 1.008x | 1.459x |
| chess960_perft_d3 | 0.992x | 0.989x | 1.410x |
| polyglot_key | 1.001x | 0.999x | 22.195x |
| polyglot_recompute | 0.996x | 0.990x | 0.994x |
| repetition_key | 0.999x | 0.998x | 20.995x |
| corpus_count | 1.000x | 0.998x | 1.221x |
| corpus_materialize | 0.997x | 0.993x | 1.307x |
| corpus_uci_sorted | 0.985x | 0.980x | 1.587x |
| replay_trusted_native | 1.011x | 0.994x | 0.692x |
| replay_checked_native | 0.979x | 0.975x | 0.973x |

Cold first legal move list: 0.012 ms -> 0.011 ms (1.02x).
