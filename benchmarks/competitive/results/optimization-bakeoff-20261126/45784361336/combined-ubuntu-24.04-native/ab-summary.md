| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.990x | 0.970x | 1.314x |
| materialize | 0.998x | 0.967x | 1.123x |
| perft_bulk_d4 | 1.006x | 0.981x | 1.259x |
| perft_full_d3 | 0.994x | 0.976x | 1.002x |
| fen_parse | 1.110x | 1.076x | 1.249x |
| fen_write_native_ep | 0.993x | 0.975x | 1.854x |
| snapshot | 0.800x | 0.773x | 0.974x |
| san_parse_all | 0.985x | 0.948x | 2.137x |
| san_render_native | 0.998x | 0.977x | 2.131x |
| apply_children_full | 1.009x | 0.984x | 1.039x |
| make_unmake_full | 0.992x | 0.964x | 1.174x |
| startpos | 0.886x | 0.886x | 1.152x |
| chess960_materialize | 0.973x | 0.958x | 1.407x |
| chess960_perft_d3 | 0.967x | 0.947x | 1.431x |
| polyglot_key | 0.961x | 0.957x | 20.347x |
| polyglot_recompute | 0.922x | 0.918x | 0.983x |
| repetition_key | 0.961x | 0.955x | 19.175x |
| corpus_count | 0.959x | 0.948x | 1.322x |
| corpus_materialize | 0.993x | 0.987x | 1.230x |
| corpus_uci_sorted | 0.957x | 0.939x | 1.685x |
| replay_trusted_native | 0.962x | 0.959x | 0.731x |
| replay_checked_native | 0.975x | 0.974x | 0.936x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.12x).
