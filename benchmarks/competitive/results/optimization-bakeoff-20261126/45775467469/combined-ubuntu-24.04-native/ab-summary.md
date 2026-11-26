| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.989x | 1.374x |
| materialize | 0.998x | 0.991x | 1.474x |
| perft_bulk_d4 | 1.000x | 0.996x | 1.170x |
| perft_full_d3 | 1.625x | 1.481x | 1.327x |
| fen_parse | 0.967x | 0.885x | 1.029x |
| fen_write_native_ep | 1.058x | 1.022x | 1.227x |
| snapshot | 0.707x | 0.704x | 1.155x |
| san_parse_all | 1.025x | 1.003x | 1.713x |
| san_render_native | 0.995x | 0.974x | 2.125x |
| apply_children_full | 1.002x | 0.976x | 0.937x |
| make_unmake_full | 0.998x | 0.991x | 0.919x |
| startpos | 1.022x | 1.022x | 1.290x |
| chess960_materialize | 0.956x | 0.949x | 1.336x |
| chess960_perft_d3 | 1.001x | 0.997x | 1.313x |
| polyglot_key | 1.017x | 1.000x | 39.149x |
| polyglot_recompute | 1.057x | 1.043x | 1.027x |
| repetition_key | 0.921x | 0.869x | 20.529x |
| corpus_count | 1.002x | 0.999x | 1.307x |
| corpus_materialize | 1.007x | 1.004x | 1.106x |
| corpus_uci_sorted | 0.997x | 0.996x | 0.974x |
| replay_trusted_native | 0.999x | 0.985x | 0.619x |
| replay_checked_native | 1.030x | 1.030x | 0.961x |

Cold first legal move list: 0.009 ms -> 0.011 ms (0.83x).
