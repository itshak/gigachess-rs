| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.951x | 1.521x |
| materialize | 1.001x | 0.978x | 1.137x |
| perft_bulk_d4 | 1.031x | 0.993x | 1.425x |
| perft_full_d3 | 1.294x | 1.091x | 1.608x |
| fen_parse | 0.969x | 0.958x | 1.361x |
| fen_write_native_ep | 0.984x | 0.973x | 1.579x |
| snapshot | 1.004x | 0.995x | 1.166x |
| san_parse_all | 0.998x | 0.983x | 1.959x |
| san_render_native | 1.020x | 1.000x | 3.391x |
| apply_children_full | 1.007x | 0.993x | 1.315x |
| make_unmake_full | 0.986x | 0.939x | 1.285x |
| startpos | 0.997x | 0.997x | 1.001x |
| chess960_materialize | 1.019x | 1.001x | 1.361x |
| chess960_perft_d3 | 0.995x | 0.985x | 1.586x |
| polyglot_key | 0.998x | 0.996x | 20.370x |
| polyglot_recompute | 0.980x | 0.978x | 0.968x |
| repetition_key | 1.016x | 0.997x | 19.232x |
| corpus_count | 1.014x | 1.007x | 1.302x |
| corpus_materialize | 1.009x | 0.994x | 1.186x |
| corpus_uci_sorted | 1.009x | 0.987x | 1.474x |
| replay_trusted_native | 1.082x | 1.042x | 1.039x |
| replay_checked_native | 0.954x | 0.916x | 1.049x |

Cold first legal move list: 0.017 ms -> 0.013 ms (1.26x).
