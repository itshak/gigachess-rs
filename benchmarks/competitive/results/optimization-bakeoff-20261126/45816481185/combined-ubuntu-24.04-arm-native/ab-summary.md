| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 1.000x | 1.381x |
| materialize | 0.997x | 0.991x | 1.245x |
| perft_bulk_d4 | 1.000x | 0.986x | 1.239x |
| perft_full_d3 | 1.000x | 0.997x | 1.077x |
| fen_parse | 0.999x | 0.993x | 1.249x |
| fen_write_native_ep | 0.997x | 0.975x | 1.579x |
| snapshot | 1.010x | 1.006x | 0.938x |
| san_parse_all | 1.000x | 0.988x | 1.810x |
| san_render_native | 0.999x | 0.990x | 2.525x |
| apply_children_full | 0.997x | 0.986x | 1.180x |
| make_unmake_full | 0.998x | 0.991x | 1.473x |
| startpos | 1.001x | 1.001x | 1.273x |
| chess960_materialize | 1.001x | 1.001x | 1.382x |
| chess960_perft_d3 | 0.996x | 0.994x | 1.335x |
| polyglot_key | 1.005x | 1.003x | 25.483x |
| polyglot_recompute | 1.000x | 0.999x | 1.072x |
| repetition_key | 0.994x | 0.981x | 18.112x |
| corpus_count | 1.016x | 0.999x | 1.242x |
| corpus_materialize | 1.005x | 0.995x | 1.167x |
| corpus_uci_sorted | 0.996x | 0.991x | 1.470x |
| replay_trusted_native | 0.997x | 0.993x | 0.545x |
| replay_checked_native | 0.997x | 0.993x | 1.022x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.05x).
