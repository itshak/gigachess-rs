| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.029x | 0.979x | 1.205x |
| materialize | 0.989x | 0.950x | 1.647x |
| perft_bulk_d4 | 0.910x | 0.868x | 1.055x |
| perft_full_d3 | 0.971x | 0.922x | 1.474x |
| fen_parse | 1.022x | 0.966x | 1.155x |
| fen_write_native_ep | 0.997x | 0.962x | 1.894x |
| snapshot | 1.116x | 1.105x | 0.997x |
| san_parse_all | 1.011x | 0.972x | 1.938x |
| san_render_native | 0.994x | 0.978x | 2.172x |
| apply_children_full | 0.999x | 0.989x | 1.278x |
| make_unmake_full | 1.022x | 1.006x | 1.219x |
| startpos | 0.995x | 0.995x | 1.351x |
| chess960_materialize | 1.025x | 0.981x | 1.443x |
| chess960_perft_d3 | 1.048x | 0.985x | 1.393x |
| polyglot_key | 1.073x | 1.005x | 42.613x |
| polyglot_recompute | 0.968x | 0.963x | 1.009x |
| repetition_key | 1.058x | 1.006x | 24.976x |
| corpus_count | 0.816x | 0.796x | 1.039x |
| corpus_materialize | 0.931x | 0.915x | 1.087x |
| corpus_uci_sorted | 0.987x | 0.980x | 1.519x |
| replay_trusted_native | 0.983x | 0.967x | 0.770x |
| replay_checked_native | 1.000x | 0.979x | 1.128x |

Cold first legal move list: 0.010 ms -> 0.005 ms (1.92x).
