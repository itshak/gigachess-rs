| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.124x | 0.952x | 1.451x |
| materialize | 1.058x | 0.978x | 1.134x |
| perft_bulk_d4 | 1.066x | 1.000x | 1.334x |
| perft_full_d3 | 1.031x | 0.983x | 1.004x |
| fen_parse | 1.075x | 1.043x | 1.125x |
| fen_write_native_ep | 0.997x | 0.972x | 1.785x |
| snapshot | 0.998x | 0.988x | 1.112x |
| san_parse_all | 0.997x | 0.979x | 2.116x |
| san_render_native | 0.986x | 0.945x | 2.091x |
| apply_children_full | 0.994x | 0.966x | 1.174x |
| make_unmake_full | 1.009x | 1.001x | 1.226x |
| startpos | 0.999x | 0.999x | 1.182x |
| chess960_materialize | 1.084x | 1.039x | 1.547x |
| chess960_perft_d3 | 1.067x | 1.039x | 1.516x |
| polyglot_key | 0.997x | 0.995x | 20.537x |
| polyglot_recompute | 0.958x | 0.939x | 0.986x |
| repetition_key | 0.997x | 0.997x | 19.365x |
| corpus_count | 1.035x | 0.948x | 1.370x |
| corpus_materialize | 1.016x | 0.989x | 1.250x |
| corpus_uci_sorted | 0.968x | 0.968x | 1.623x |
| replay_trusted_native | 1.016x | 1.008x | 0.824x |
| replay_checked_native | 0.997x | 0.996x | 0.975x |

Cold first legal move list: 0.012 ms -> 0.011 ms (1.07x).
