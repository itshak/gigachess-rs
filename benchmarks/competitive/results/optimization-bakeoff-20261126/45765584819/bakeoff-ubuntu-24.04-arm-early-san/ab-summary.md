| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.085x |
| materialize | 0.997x | 0.994x | 1.176x |
| perft_bulk_d4 | 1.023x | 1.008x | 0.910x |
| perft_full_d3 | 0.998x | 0.995x | 0.619x |
| fen_parse | 0.975x | 0.963x | 1.186x |
| fen_write_native_ep | 1.007x | 1.001x | 1.095x |
| snapshot | 1.015x | 0.996x | 0.768x |
| san_parse_all | 0.997x | 0.991x | 0.913x |
| san_render_native | 1.585x | 1.169x | 1.823x |
| apply_children_full | 0.990x | 0.979x | 0.649x |
| make_unmake_full | 1.000x | 0.996x | 0.892x |
| startpos | 0.895x | 0.895x | 0.731x |
| chess960_materialize | 0.989x | 0.985x | 1.103x |
| chess960_perft_d3 | 0.986x | 0.986x | 0.829x |
| polyglot_key | 1.000x | 0.980x | 23.337x |
| polyglot_recompute | 1.003x | 0.994x | 1.135x |
| repetition_key | 0.993x | 0.988x | 14.002x |
| corpus_count | 1.000x | 0.986x | 1.217x |
| corpus_materialize | 0.994x | 0.989x | 1.139x |
| corpus_uci_sorted | 1.008x | 1.006x | 0.574x |
| replay_trusted_native | 0.998x | 0.998x | 0.316x |
| replay_checked_native | 0.992x | 0.991x | 0.656x |

Cold first legal move list: 153.984 ms → 153.968 ms (1.00x).
