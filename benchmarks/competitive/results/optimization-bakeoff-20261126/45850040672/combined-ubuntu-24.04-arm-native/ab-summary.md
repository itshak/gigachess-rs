| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.992x | 1.551x |
| materialize | 0.997x | 0.990x | 1.315x |
| perft_bulk_d4 | 0.998x | 0.992x | 1.395x |
| perft_full_d3 | 0.997x | 0.995x | 1.335x |
| fen_parse | 1.013x | 1.000x | 1.228x |
| fen_write_native_ep | 0.996x | 0.995x | 1.583x |
| snapshot | 1.004x | 0.999x | 0.934x |
| san_parse_all | 1.000x | 0.993x | 1.837x |
| san_render_native | 1.058x | 1.025x | 2.691x |
| apply_children_full | 1.002x | 0.989x | 1.162x |
| make_unmake_full | 1.026x | 0.991x | 1.553x |
| startpos | 0.999x | 0.999x | 1.278x |
| chess960_materialize | 0.993x | 0.990x | 1.487x |
| chess960_perft_d3 | 1.004x | 1.000x | 1.494x |
| polyglot_key | 1.001x | 0.995x | 25.249x |
| polyglot_recompute | 1.004x | 1.000x | 1.088x |
| repetition_key | 1.008x | 1.000x | 19.120x |
| corpus_count | 0.996x | 0.995x | 1.361x |
| corpus_materialize | 1.004x | 1.001x | 1.229x |
| corpus_uci_sorted | 0.993x | 0.986x | 1.459x |
| replay_trusted_native | 1.018x | 1.017x | 0.631x |
| replay_checked_native | 1.074x | 1.057x | 1.139x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.05x).
