| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.549x |
| materialize | 1.001x | 1.000x | 1.319x |
| perft_bulk_d4 | 0.998x | 0.994x | 1.391x |
| perft_full_d3 | 1.003x | 0.997x | 1.334x |
| fen_parse | 1.007x | 0.999x | 1.222x |
| fen_write_native_ep | 1.000x | 0.991x | 1.572x |
| snapshot | 1.000x | 0.994x | 0.944x |
| san_parse_all | 0.999x | 0.994x | 1.843x |
| san_render_native | 1.000x | 0.985x | 2.526x |
| apply_children_full | 0.991x | 0.976x | 1.146x |
| make_unmake_full | 0.978x | 0.937x | 1.485x |
| startpos | 0.999x | 0.999x | 1.298x |
| chess960_materialize | 1.006x | 1.003x | 1.503x |
| chess960_perft_d3 | 1.013x | 1.008x | 1.508x |
| polyglot_key | 0.985x | 0.979x | 25.527x |
| polyglot_recompute | 0.995x | 0.993x | 1.083x |
| repetition_key | 0.994x | 0.988x | 19.216x |
| corpus_count | 1.001x | 0.995x | 1.383x |
| corpus_materialize | 0.998x | 0.996x | 1.218x |
| corpus_uci_sorted | 0.998x | 0.996x | 1.461x |
| replay_trusted_native | 0.967x | 0.946x | 0.593x |
| replay_checked_native | 0.999x | 0.994x | 1.046x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.94x).
