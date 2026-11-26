| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.957x | 0.931x | 1.183x |
| materialize | 0.925x | 0.855x | 1.069x |
| perft_bulk_d4 | 0.990x | 0.980x | 0.963x |
| perft_full_d3 | 1.063x | 1.023x | 0.630x |
| fen_parse | 1.036x | 0.991x | 1.195x |
| fen_write_native_ep | 1.002x | 0.984x | 1.238x |
| snapshot | 0.880x | 0.850x | 0.753x |
| san_parse_all | 1.005x | 0.984x | 0.969x |
| san_render_native | 1.006x | 0.991x | 1.131x |
| apply_children_full | 1.005x | 0.983x | 0.715x |
| make_unmake_full | 1.023x | 1.016x | 0.860x |
| startpos | 1.125x | 1.125x | 0.875x |
| chess960_materialize | 1.038x | 1.038x | 1.265x |
| chess960_perft_d3 | 1.007x | 1.001x | 1.022x |
| polyglot_key | 0.974x | 0.953x | 22.316x |
| polyglot_recompute | 0.985x | 0.980x | 1.039x |
| repetition_key | 1.000x | 0.999x | 13.062x |
| corpus_count | 0.972x | 0.963x | 1.208x |
| corpus_materialize | 1.017x | 1.010x | 1.215x |
| corpus_uci_sorted | 1.005x | 1.003x | 0.666x |
| replay_trusted_native | 1.043x | 1.026x | 0.571x |
| replay_checked_native | 1.046x | 1.039x | 0.729x |

Cold first legal move list: 168.274 ms → 0.011 ms (15177.57x).
