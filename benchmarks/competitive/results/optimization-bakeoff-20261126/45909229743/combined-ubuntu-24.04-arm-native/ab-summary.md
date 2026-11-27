| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.978x | 1.562x |
| materialize | 1.013x | 1.005x | 1.343x |
| perft_bulk_d4 | 1.043x | 1.023x | 1.450x |
| perft_full_d3 | 1.124x | 1.002x | 1.497x |
| fen_parse | 0.998x | 0.994x | 1.215x |
| fen_write_native_ep | 1.011x | 1.007x | 1.572x |
| snapshot | 0.999x | 0.994x | 0.962x |
| san_parse_all | 0.998x | 0.992x | 1.834x |
| san_render_native | 1.005x | 0.992x | 2.696x |
| apply_children_full | 0.999x | 0.991x | 1.151x |
| make_unmake_full | 0.998x | 0.995x | 1.572x |
| startpos | 0.999x | 0.999x | 1.335x |
| chess960_materialize | 1.010x | 1.009x | 1.514x |
| chess960_perft_d3 | 1.016x | 1.001x | 1.523x |
| polyglot_key | 0.990x | 0.987x | 25.787x |
| polyglot_recompute | 0.997x | 0.995x | 1.091x |
| repetition_key | 1.001x | 1.000x | 19.536x |
| corpus_count | 1.093x | 1.067x | 1.487x |
| corpus_materialize | 1.012x | 1.010x | 1.251x |
| corpus_uci_sorted | 1.005x | 1.002x | 1.458x |
| replay_trusted_native | 0.998x | 0.998x | 0.625x |
| replay_checked_native | 1.006x | 1.005x | 1.135x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.86x).
