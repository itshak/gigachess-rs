| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.026x | 0.977x | 1.152x |
| materialize | 1.006x | 0.974x | 1.254x |
| perft_bulk_d4 | 1.011x | 1.002x | 1.177x |
| perft_full_d3 | 1.462x | 1.330x | 1.267x |
| fen_parse | 1.071x | 1.053x | 1.306x |
| fen_write_native_ep | 1.000x | 0.988x | 0.954x |
| snapshot | 0.999x | 0.998x | 0.984x |
| san_parse_all | 0.980x | 0.967x | 1.823x |
| san_render_native | 0.967x | 0.951x | 2.356x |
| apply_children_full | 0.980x | 0.950x | 0.930x |
| make_unmake_full | 0.990x | 0.986x | 1.071x |
| startpos | 1.032x | 1.032x | 1.276x |
| chess960_materialize | 1.004x | 0.994x | 1.282x |
| chess960_perft_d3 | 0.987x | 0.979x | 1.279x |
| polyglot_key | 0.953x | 0.934x | 22.434x |
| polyglot_recompute | 0.985x | 0.981x | 1.016x |
| repetition_key | 0.989x | 0.952x | 17.918x |
| corpus_count | 1.022x | 1.013x | 1.259x |
| corpus_materialize | 0.994x | 0.990x | 1.206x |
| corpus_uci_sorted | 1.005x | 0.994x | 1.012x |
| replay_trusted_native | 1.013x | 1.005x | 0.604x |
| replay_checked_native | 0.977x | 0.975x | 0.971x |

Cold first legal move list: 0.013 ms -> 0.014 ms (0.92x).
