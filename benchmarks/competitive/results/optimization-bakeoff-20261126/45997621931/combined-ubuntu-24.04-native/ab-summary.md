| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.998x | 1.517x |
| materialize | 0.992x | 0.981x | 1.575x |
| perft_bulk_d4 | 0.998x | 0.990x | 1.171x |
| perft_full_d3 | 1.012x | 0.997x | 1.467x |
| fen_parse | 0.935x | 0.840x | 1.083x |
| fen_write_native_ep | 1.001x | 0.991x | 1.883x |
| snapshot | 1.001x | 0.993x | 1.151x |
| san_parse_all | 0.930x | 0.918x | 1.748x |
| san_render_native | 0.987x | 0.974x | 2.118x |
| apply_children_full | 0.996x | 0.977x | 1.333x |
| make_unmake_full | 0.948x | 0.922x | 1.139x |
| startpos | 0.895x | 0.895x | 1.122x |
| chess960_materialize | 0.919x | 0.919x | 1.439x |
| chess960_perft_d3 | 1.001x | 0.998x | 1.510x |
| polyglot_key | 0.997x | 0.994x | 39.663x |
| polyglot_recompute | 1.025x | 1.022x | 1.016x |
| repetition_key | 1.001x | 0.992x | 19.435x |
| corpus_count | 1.002x | 1.000x | 1.087x |
| corpus_materialize | 1.013x | 1.012x | 1.067x |
| corpus_uci_sorted | 1.014x | 1.007x | 1.559x |
| replay_trusted_native | 0.978x | 0.949x | 0.862x |
| replay_checked_native | 0.972x | 0.967x | 1.171x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.03x).
