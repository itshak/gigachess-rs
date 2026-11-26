| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.966x | 1.477x |
| materialize | 0.951x | 0.912x | 1.526x |
| perft_bulk_d4 | 0.993x | 0.983x | 1.278x |
| perft_full_d3 | 1.132x | 0.942x | 1.449x |
| fen_parse | 1.018x | 0.974x | 1.128x |
| fen_write_native_ep | 1.003x | 0.932x | 1.833x |
| snapshot | 0.600x | 0.599x | 0.999x |
| san_parse_all | 1.185x | 1.119x | 1.986x |
| san_render_native | 0.995x | 0.989x | 2.118x |
| apply_children_full | 0.983x | 0.931x | 1.048x |
| make_unmake_full | 0.934x | 0.907x | 1.067x |
| startpos | 0.743x | 0.743x | 1.167x |
| chess960_materialize | 0.927x | 0.904x | 1.337x |
| chess960_perft_d3 | 0.980x | 0.963x | 1.359x |
| polyglot_key | 1.001x | 1.000x | 39.442x |
| polyglot_recompute | 0.968x | 0.963x | 0.974x |
| repetition_key | 1.017x | 1.012x | 19.314x |
| corpus_count | 0.994x | 0.990x | 1.393x |
| corpus_materialize | 1.002x | 1.002x | 1.110x |
| corpus_uci_sorted | 1.013x | 1.012x | 1.597x |
| replay_trusted_native | 0.916x | 0.890x | 0.655x |
| replay_checked_native | 0.981x | 0.948x | 1.091x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.87x).
