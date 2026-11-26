| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.548x |
| materialize | 1.000x | 0.999x | 1.315x |
| perft_bulk_d4 | 1.003x | 0.996x | 1.395x |
| perft_full_d3 | 1.004x | 1.001x | 1.338x |
| fen_parse | 1.000x | 0.990x | 1.213x |
| fen_write_native_ep | 1.003x | 0.998x | 1.588x |
| snapshot | 1.001x | 0.999x | 0.949x |
| san_parse_all | 1.002x | 0.993x | 1.840x |
| san_render_native | 1.004x | 0.994x | 2.555x |
| apply_children_full | 0.994x | 0.974x | 1.149x |
| make_unmake_full | 1.039x | 1.000x | 1.579x |
| startpos | 1.001x | 1.001x | 1.303x |
| chess960_materialize | 1.004x | 1.001x | 1.498x |
| chess960_perft_d3 | 1.013x | 1.009x | 1.508x |
| polyglot_key | 1.022x | 0.994x | 25.525x |
| polyglot_recompute | 0.996x | 0.996x | 1.082x |
| repetition_key | 1.028x | 1.000x | 19.112x |
| corpus_count | 1.006x | 1.006x | 1.375x |
| corpus_materialize | 1.003x | 1.003x | 1.223x |
| corpus_uci_sorted | 1.005x | 1.003x | 1.464x |
| replay_trusted_native | 1.008x | 0.994x | 0.640x |
| replay_checked_native | 1.045x | 1.043x | 1.094x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.08x).
