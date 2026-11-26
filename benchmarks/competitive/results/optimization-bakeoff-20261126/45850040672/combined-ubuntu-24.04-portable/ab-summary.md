| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.292x |
| materialize | 0.929x | 0.901x | 1.646x |
| perft_bulk_d4 | 0.996x | 0.990x | 1.183x |
| perft_full_d3 | 0.998x | 0.994x | 1.319x |
| fen_parse | 1.012x | 0.983x | 1.144x |
| fen_write_native_ep | 0.998x | 0.988x | 1.865x |
| snapshot | 0.996x | 0.991x | 0.997x |
| san_parse_all | 1.032x | 1.006x | 1.927x |
| san_render_native | 1.005x | 0.998x | 2.156x |
| apply_children_full | 1.033x | 1.014x | 1.363x |
| make_unmake_full | 0.970x | 0.934x | 1.186x |
| startpos | 1.000x | 1.000x | 1.352x |
| chess960_materialize | 1.002x | 0.999x | 1.547x |
| chess960_perft_d3 | 1.005x | 0.999x | 1.511x |
| polyglot_key | 1.016x | 1.000x | 41.004x |
| polyglot_recompute | 1.059x | 1.041x | 1.023x |
| repetition_key | 1.244x | 1.140x | 24.161x |
| corpus_count | 0.965x | 0.936x | 1.143x |
| corpus_materialize | 1.007x | 1.002x | 1.152x |
| corpus_uci_sorted | 0.981x | 0.975x | 1.518x |
| replay_trusted_native | 1.001x | 0.978x | 0.885x |
| replay_checked_native | 1.077x | 1.036x | 1.161x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.87x).
