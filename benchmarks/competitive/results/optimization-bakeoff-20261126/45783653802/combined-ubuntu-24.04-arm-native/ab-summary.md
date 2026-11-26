| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.995x | 1.355x |
| materialize | 0.995x | 0.976x | 1.227x |
| perft_bulk_d4 | 1.000x | 0.997x | 1.346x |
| perft_full_d3 | 1.004x | 0.997x | 1.072x |
| fen_parse | 0.999x | 0.970x | 1.234x |
| fen_write_native_ep | 0.999x | 0.990x | 1.050x |
| snapshot | 0.940x | 0.933x | 0.933x |
| san_parse_all | 1.002x | 1.000x | 1.808x |
| san_render_native | 1.006x | 0.999x | 2.544x |
| apply_children_full | 0.990x | 0.980x | 1.033x |
| make_unmake_full | 1.010x | 1.004x | 1.431x |
| startpos | 0.963x | 0.963x | 1.184x |
| chess960_materialize | 1.002x | 1.000x | 1.313x |
| chess960_perft_d3 | 1.010x | 1.007x | 1.325x |
| polyglot_key | 1.003x | 0.992x | 25.758x |
| polyglot_recompute | 0.970x | 0.962x | 1.044x |
| repetition_key | 1.009x | 1.001x | 19.417x |
| corpus_count | 1.006x | 1.002x | 1.472x |
| corpus_materialize | 1.001x | 0.999x | 1.191x |
| corpus_uci_sorted | 1.000x | 0.998x | 1.458x |
| replay_trusted_native | 0.998x | 0.994x | 0.500x |
| replay_checked_native | 1.002x | 0.999x | 0.994x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.96x).
