| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.990x | 1.312x |
| materialize | 0.997x | 0.967x | 1.345x |
| perft_bulk_d4 | 1.009x | 1.000x | 1.285x |
| perft_full_d3 | 1.004x | 0.996x | 1.296x |
| fen_parse | 0.886x | 0.851x | 1.042x |
| fen_write_native_ep | 0.993x | 0.967x | 1.706x |
| snapshot | 1.000x | 0.997x | 0.993x |
| san_parse_all | 0.983x | 0.974x | 2.133x |
| san_render_native | 0.951x | 0.895x | 2.125x |
| apply_children_full | 1.017x | 0.991x | 1.242x |
| make_unmake_full | 1.016x | 1.001x | 1.398x |
| startpos | 1.001x | 1.001x | 1.336x |
| chess960_materialize | 0.941x | 0.939x | 1.509x |
| chess960_perft_d3 | 1.020x | 0.997x | 1.572x |
| polyglot_key | 0.995x | 0.990x | 21.825x |
| polyglot_recompute | 1.006x | 1.003x | 0.973x |
| repetition_key | 1.005x | 0.999x | 20.152x |
| corpus_count | 1.004x | 1.000x | 1.227x |
| corpus_materialize | 0.987x | 0.976x | 1.327x |
| corpus_uci_sorted | 0.987x | 0.984x | 1.587x |
| replay_trusted_native | 1.026x | 0.998x | 0.963x |
| replay_checked_native | 1.006x | 0.997x | 1.090x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.96x).
