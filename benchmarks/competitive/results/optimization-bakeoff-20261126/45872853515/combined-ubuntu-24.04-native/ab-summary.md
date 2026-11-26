| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.990x | 1.517x |
| materialize | 0.989x | 0.959x | 1.559x |
| perft_bulk_d4 | 1.000x | 0.991x | 1.167x |
| perft_full_d3 | 1.242x | 1.055x | 1.671x |
| fen_parse | 0.932x | 0.831x | 1.000x |
| fen_write_native_ep | 0.995x | 0.991x | 2.135x |
| snapshot | 1.001x | 0.992x | 1.153x |
| san_parse_all | 1.003x | 0.960x | 1.884x |
| san_render_native | 0.998x | 0.990x | 2.140x |
| apply_children_full | 0.998x | 0.991x | 1.341x |
| make_unmake_full | 0.976x | 0.928x | 1.161x |
| startpos | 0.993x | 0.993x | 1.113x |
| chess960_materialize | 0.990x | 0.982x | 1.572x |
| chess960_perft_d3 | 0.988x | 0.982x | 1.487x |
| polyglot_key | 1.029x | 1.002x | 39.404x |
| polyglot_recompute | 1.024x | 1.022x | 1.017x |
| repetition_key | 1.022x | 0.998x | 19.162x |
| corpus_count | 1.015x | 1.008x | 1.095x |
| corpus_materialize | 1.007x | 1.004x | 1.045x |
| corpus_uci_sorted | 0.996x | 0.989x | 1.567x |
| replay_trusted_native | 0.998x | 0.995x | 0.874x |
| replay_checked_native | 1.025x | 1.007x | 1.143x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.88x).
