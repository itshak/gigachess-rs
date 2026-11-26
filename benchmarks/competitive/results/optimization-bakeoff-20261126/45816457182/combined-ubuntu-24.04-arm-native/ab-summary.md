| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.388x |
| materialize | 0.997x | 0.994x | 1.249x |
| perft_bulk_d4 | 0.995x | 0.990x | 1.233x |
| perft_full_d3 | 1.000x | 0.999x | 1.072x |
| fen_parse | 1.009x | 1.003x | 1.258x |
| fen_write_native_ep | 0.999x | 0.997x | 1.590x |
| snapshot | 0.999x | 0.997x | 0.955x |
| san_parse_all | 1.002x | 0.996x | 1.813x |
| san_render_native | 1.067x | 1.034x | 2.694x |
| apply_children_full | 1.000x | 0.997x | 1.180x |
| make_unmake_full | 0.997x | 0.985x | 1.483x |
| startpos | 0.999x | 0.999x | 1.329x |
| chess960_materialize | 0.998x | 0.995x | 1.385x |
| chess960_perft_d3 | 1.009x | 1.004x | 1.354x |
| polyglot_key | 0.990x | 0.989x | 25.019x |
| polyglot_recompute | 0.999x | 0.995x | 1.072x |
| repetition_key | 0.992x | 0.982x | 18.185x |
| corpus_count | 0.994x | 0.990x | 1.220x |
| corpus_materialize | 0.997x | 0.996x | 1.161x |
| corpus_uci_sorted | 1.002x | 1.000x | 1.448x |
| replay_trusted_native | 0.996x | 0.995x | 0.540x |
| replay_checked_native | 0.990x | 0.984x | 1.027x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.97x).
