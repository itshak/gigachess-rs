| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.379x |
| materialize | 1.006x | 0.995x | 1.245x |
| perft_bulk_d4 | 1.003x | 0.999x | 1.241x |
| perft_full_d3 | 0.992x | 0.979x | 1.061x |
| fen_parse | 1.000x | 0.998x | 1.232x |
| fen_write_native_ep | 0.984x | 0.969x | 1.608x |
| snapshot | 1.101x | 1.095x | 0.923x |
| san_parse_all | 1.013x | 1.002x | 1.846x |
| san_render_native | 1.004x | 0.995x | 2.530x |
| apply_children_full | 0.931x | 0.901x | 1.202x |
| make_unmake_full | 1.007x | 0.993x | 1.518x |
| startpos | 1.000x | 1.000x | 1.184x |
| chess960_materialize | 0.997x | 0.994x | 1.373x |
| chess960_perft_d3 | 0.992x | 0.988x | 1.341x |
| polyglot_key | 0.996x | 0.977x | 25.660x |
| polyglot_recompute | 1.004x | 1.002x | 1.070x |
| repetition_key | 1.033x | 0.997x | 19.125x |
| corpus_count | 1.003x | 1.002x | 1.232x |
| corpus_materialize | 1.007x | 1.005x | 1.175x |
| corpus_uci_sorted | 1.004x | 1.000x | 1.454x |
| replay_trusted_native | 0.979x | 0.977x | 0.599x |
| replay_checked_native | 0.994x | 0.990x | 1.056x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.06x).
