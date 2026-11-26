| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.999x | 1.203x |
| materialize | 1.080x | 1.054x | 1.646x |
| perft_bulk_d4 | 1.008x | 0.994x | 1.170x |
| perft_full_d3 | 1.003x | 0.993x | 1.492x |
| fen_parse | 0.905x | 0.863x | 1.119x |
| fen_write_native_ep | 1.005x | 0.998x | 1.165x |
| snapshot | 1.002x | 0.996x | 1.002x |
| san_parse_all | 0.930x | 0.911x | 1.728x |
| san_render_native | 1.010x | 0.996x | 2.156x |
| apply_children_full | 1.002x | 0.979x | 1.110x |
| make_unmake_full | 1.007x | 0.992x | 1.208x |
| startpos | 1.001x | 1.001x | 1.284x |
| chess960_materialize | 0.991x | 0.978x | 1.439x |
| chess960_perft_d3 | 0.999x | 0.993x | 1.338x |
| polyglot_key | 1.110x | 0.998x | 41.073x |
| polyglot_recompute | 0.994x | 0.988x | 1.022x |
| repetition_key | 1.054x | 1.009x | 24.541x |
| corpus_count | 0.999x | 0.998x | 1.245x |
| corpus_materialize | 0.998x | 0.998x | 1.174x |
| corpus_uci_sorted | 0.989x | 0.986x | 1.541x |
| replay_trusted_native | 1.008x | 1.003x | 0.723x |
| replay_checked_native | 1.001x | 0.991x | 1.139x |

Cold first legal move list: 0.010 ms -> 0.009 ms (1.13x).
