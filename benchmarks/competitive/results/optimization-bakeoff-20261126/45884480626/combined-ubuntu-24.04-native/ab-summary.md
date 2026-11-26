| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.038x | 0.985x | 1.555x |
| materialize | 1.003x | 0.993x | 1.575x |
| perft_bulk_d4 | 1.023x | 1.006x | 1.193x |
| perft_full_d3 | 1.242x | 1.061x | 1.685x |
| fen_parse | 0.982x | 0.961x | 1.152x |
| fen_write_native_ep | 1.001x | 0.994x | 1.914x |
| snapshot | 0.998x | 0.996x | 1.151x |
| san_parse_all | 0.998x | 0.988x | 1.882x |
| san_render_native | 0.996x | 0.989x | 2.098x |
| apply_children_full | 1.001x | 1.000x | 1.337x |
| make_unmake_full | 1.000x | 0.994x | 1.196x |
| startpos | 0.963x | 0.963x | 1.078x |
| chess960_materialize | 0.999x | 0.995x | 1.550x |
| chess960_perft_d3 | 1.011x | 1.010x | 1.524x |
| polyglot_key | 1.025x | 0.999x | 38.984x |
| polyglot_recompute | 1.000x | 0.996x | 0.988x |
| repetition_key | 1.161x | 0.972x | 22.447x |
| corpus_count | 0.950x | 0.833x | 0.998x |
| corpus_materialize | 1.012x | 1.011x | 1.051x |
| corpus_uci_sorted | 1.003x | 1.003x | 1.569x |
| replay_trusted_native | 1.011x | 1.001x | 0.868x |
| replay_checked_native | 1.015x | 1.007x | 1.136x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.88x).
