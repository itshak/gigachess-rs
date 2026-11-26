| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.974x | 1.468x |
| materialize | 1.042x | 0.983x | 1.201x |
| perft_bulk_d4 | 1.014x | 0.971x | 1.313x |
| perft_full_d3 | 0.965x | 0.907x | 0.989x |
| fen_parse | 0.947x | 0.926x | 1.109x |
| fen_write_native_ep | 1.002x | 0.993x | 1.931x |
| snapshot | 1.512x | 1.505x | 1.251x |
| san_parse_all | 0.984x | 0.968x | 2.121x |
| san_render_native | 1.006x | 0.974x | 2.150x |
| apply_children_full | 0.949x | 0.910x | 1.128x |
| make_unmake_full | 0.960x | 0.933x | 1.190x |
| startpos | 1.200x | 1.200x | 1.201x |
| chess960_materialize | 1.043x | 0.987x | 1.537x |
| chess960_perft_d3 | 0.999x | 0.996x | 1.503x |
| polyglot_key | 1.001x | 1.001x | 20.843x |
| polyglot_recompute | 0.995x | 0.992x | 1.018x |
| repetition_key | 0.695x | 0.659x | 13.409x |
| corpus_count | 1.002x | 1.000x | 1.466x |
| corpus_materialize | 1.003x | 1.001x | 1.322x |
| corpus_uci_sorted | 1.001x | 0.994x | 1.675x |
| replay_trusted_native | 0.958x | 0.948x | 0.800x |
| replay_checked_native | 0.980x | 0.967x | 0.988x |

Cold first legal move list: 0.012 ms -> 0.011 ms (1.11x).
