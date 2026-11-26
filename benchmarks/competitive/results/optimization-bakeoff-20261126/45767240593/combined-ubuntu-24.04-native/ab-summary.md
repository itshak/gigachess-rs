| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.966x | 1.195x |
| materialize | 0.985x | 0.957x | 1.183x |
| perft_bulk_d4 | 1.149x | 1.125x | 1.121x |
| perft_full_d3 | 1.259x | 1.212x | 0.776x |
| fen_parse | 1.089x | 1.057x | 1.157x |
| fen_write_native_ep | 1.006x | 0.984x | 0.916x |
| snapshot | 1.225x | 1.219x | 1.036x |
| san_parse_all | 1.908x | 1.813x | 1.895x |
| san_render_native | 1.766x | 1.449x | 2.129x |
| apply_children_full | 1.260x | 1.220x | 0.926x |
| make_unmake_full | 1.156x | 1.120x | 0.980x |
| startpos | 1.524x | 1.524x | 1.110x |
| chess960_materialize | 1.062x | 0.964x | 1.219x |
| chess960_perft_d3 | 1.198x | 1.105x | 1.141x |
| polyglot_key | 0.897x | 0.812x | 25.317x |
| polyglot_recompute | 0.994x | 0.990x | 1.012x |
| repetition_key | 1.316x | 1.244x | 19.310x |
| corpus_count | 1.075x | 1.051x | 1.277x |
| corpus_materialize | 1.000x | 0.999x | 1.157x |
| corpus_uci_sorted | 1.200x | 1.171x | 0.782x |
| replay_trusted_native | 1.242x | 1.192x | 0.620x |
| replay_checked_native | 1.328x | 1.233x | 0.905x |

Cold first legal move list: 282.039 ms → 0.012 ms (22851.99x).
