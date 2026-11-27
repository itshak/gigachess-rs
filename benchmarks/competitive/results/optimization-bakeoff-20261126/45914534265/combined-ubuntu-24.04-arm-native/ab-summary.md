| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.548x |
| materialize | 0.998x | 0.997x | 1.317x |
| perft_bulk_d4 | 1.004x | 1.000x | 1.396x |
| perft_full_d3 | 1.001x | 0.999x | 1.331x |
| fen_parse | 0.996x | 0.994x | 1.219x |
| fen_write_native_ep | 0.998x | 0.996x | 1.598x |
| snapshot | 1.003x | 1.001x | 0.949x |
| san_parse_all | 0.991x | 0.985x | 1.829x |
| san_render_native | 0.998x | 0.995x | 2.687x |
| apply_children_full | 1.001x | 0.998x | 1.157x |
| make_unmake_full | 1.003x | 0.994x | 1.557x |
| startpos | 1.000x | 1.000x | 1.302x |
| chess960_materialize | 1.000x | 1.000x | 1.493x |
| chess960_perft_d3 | 0.998x | 0.996x | 1.489x |
| polyglot_key | 1.010x | 0.998x | 25.795x |
| polyglot_recompute | 0.997x | 0.995x | 1.085x |
| repetition_key | 1.006x | 1.000x | 19.316x |
| corpus_count | 0.997x | 0.995x | 1.358x |
| corpus_materialize | 0.997x | 0.995x | 1.224x |
| corpus_uci_sorted | 1.000x | 1.000x | 1.472x |
| replay_trusted_native | 1.000x | 0.998x | 0.633x |
| replay_checked_native | 0.996x | 0.989x | 1.124x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.06x).
