| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.088x | 0.849x | 1.324x |
| materialize | 1.139x | 0.898x | 1.125x |
| perft_bulk_d4 | 0.920x | 0.670x | 1.167x |
| perft_full_d3 | 1.101x | 0.982x | 1.362x |
| fen_parse | 1.060x | 0.837x | 1.219x |
| fen_write_native_ep | 1.051x | 0.899x | 1.805x |
| snapshot | 0.992x | 0.892x | 0.928x |
| san_parse_all | 1.117x | 0.938x | 1.918x |
| san_render_native | 1.043x | 0.820x | 2.686x |
| apply_children_full | 1.077x | 0.980x | 0.972x |
| make_unmake_full | 1.028x | 0.849x | 1.240x |
| startpos | 1.368x | 1.368x | 1.221x |
| chess960_materialize | 1.058x | 0.894x | 1.316x |
| chess960_perft_d3 | 0.822x | 0.719x | 1.257x |
| polyglot_key | 0.912x | 0.803x | 42.422x |
| polyglot_recompute | 0.955x | 0.829x | 1.001x |
| repetition_key | 0.860x | 0.756x | 19.266x |
| corpus_count | 0.634x | 0.460x | 0.835x |
| corpus_materialize | 1.225x | 1.119x | 1.192x |
| corpus_uci_sorted | 1.118x | 0.976x | 1.521x |
| replay_trusted_native | 0.949x | 0.890x | 0.540x |
| replay_checked_native | 1.118x | 1.044x | 0.935x |

Cold first legal move list: 0.015 ms -> 0.007 ms (2.04x).
