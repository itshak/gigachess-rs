| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.084x |
| materialize | 1.001x | 0.994x | 1.171x |
| perft_bulk_d4 | 1.099x | 1.050x | 0.999x |
| perft_full_d3 | 0.999x | 0.997x | 0.612x |
| fen_parse | 0.996x | 0.991x | 1.198x |
| fen_write_native_ep | 1.006x | 0.997x | 1.057x |
| snapshot | 0.999x | 0.978x | 0.757x |
| san_parse_all | 0.990x | 0.980x | 0.914x |
| san_render_native | 1.003x | 0.998x | 1.147x |
| apply_children_full | 1.007x | 1.002x | 0.657x |
| make_unmake_full | 1.003x | 0.998x | 0.890x |
| startpos | 0.993x | 0.993x | 0.767x |
| chess960_materialize | 1.005x | 1.002x | 1.128x |
| chess960_perft_d3 | 1.079x | 1.027x | 0.908x |
| polyglot_key | 1.048x | 1.015x | 23.170x |
| polyglot_recompute | 1.000x | 0.999x | 1.126x |
| repetition_key | 1.041x | 1.002x | 13.974x |
| corpus_count | 1.013x | 1.005x | 1.206x |
| corpus_materialize | 1.000x | 0.999x | 1.107x |
| corpus_uci_sorted | 0.999x | 0.998x | 0.569x |
| replay_trusted_native | 1.000x | 0.999x | 0.314x |
| replay_checked_native | 0.999x | 0.999x | 0.656x |

Cold first legal move list: 153.949 ms → 154.006 ms (1.00x).
