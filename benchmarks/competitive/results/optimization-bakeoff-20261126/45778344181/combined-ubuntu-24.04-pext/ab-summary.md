| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.986x | 0.973x | 1.458x |
| materialize | 1.006x | 0.990x | 1.580x |
| perft_bulk_d4 | 0.920x | 0.882x | 1.159x |
| perft_full_d3 | 1.001x | 0.956x | 1.528x |
| fen_parse | 1.013x | 0.959x | 1.095x |
| fen_write_native_ep | 0.964x | 0.930x | 1.265x |
| snapshot | 0.786x | 0.779x | 0.994x |
| san_parse_all | 1.007x | 0.954x | 1.926x |
| san_render_native | 0.965x | 0.927x | 2.033x |
| apply_children_full | 0.683x | 0.654x | 0.737x |
| make_unmake_full | 0.870x | 0.818x | 0.986x |
| startpos | 1.000x | 1.000x | 1.290x |
| chess960_materialize | 1.006x | 1.003x | 1.454x |
| chess960_perft_d3 | 0.919x | 0.894x | 1.276x |
| polyglot_key | 1.051x | 1.006x | 38.746x |
| polyglot_recompute | 0.977x | 0.975x | 1.006x |
| repetition_key | 1.001x | 0.990x | 22.317x |
| corpus_count | 0.982x | 0.954x | 1.371x |
| corpus_materialize | 1.016x | 1.014x | 1.129x |
| corpus_uci_sorted | 0.994x | 0.991x | 1.557x |
| replay_trusted_native | 0.984x | 0.966x | 0.686x |
| replay_checked_native | 0.975x | 0.963x | 1.032x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.07x).
