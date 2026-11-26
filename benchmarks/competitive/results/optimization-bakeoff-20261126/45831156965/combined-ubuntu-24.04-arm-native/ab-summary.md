| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.998x | 1.377x |
| materialize | 0.999x | 0.995x | 1.249x |
| perft_bulk_d4 | 0.999x | 0.996x | 1.242x |
| perft_full_d3 | 1.010x | 1.005x | 1.060x |
| fen_parse | 1.012x | 1.004x | 1.227x |
| fen_write_native_ep | 1.001x | 0.998x | 1.569x |
| snapshot | 0.997x | 0.993x | 0.946x |
| san_parse_all | 1.000x | 0.990x | 1.834x |
| san_render_native | 1.002x | 0.979x | 2.524x |
| apply_children_full | 0.990x | 0.975x | 1.143x |
| make_unmake_full | 1.012x | 0.989x | 1.558x |
| startpos | 0.999x | 0.999x | 1.302x |
| chess960_materialize | 1.007x | 1.007x | 1.394x |
| chess960_perft_d3 | 0.989x | 0.973x | 1.332x |
| polyglot_key | 1.023x | 1.008x | 25.931x |
| polyglot_recompute | 1.002x | 1.000x | 1.086x |
| repetition_key | 1.006x | 1.003x | 19.330x |
| corpus_count | 1.001x | 0.997x | 1.193x |
| corpus_materialize | 1.005x | 1.001x | 1.172x |
| corpus_uci_sorted | 0.997x | 0.996x | 1.454x |
| replay_trusted_native | 1.013x | 1.005x | 0.614x |
| replay_checked_native | 0.988x | 0.975x | 1.039x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.06x).
