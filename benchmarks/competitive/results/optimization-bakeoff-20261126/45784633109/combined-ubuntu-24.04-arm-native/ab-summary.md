| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.939x | 1.378x |
| materialize | 1.014x | 0.981x | 1.247x |
| perft_bulk_d4 | 0.917x | 0.896x | 1.240x |
| perft_full_d3 | 1.003x | 0.996x | 1.058x |
| fen_parse | 0.994x | 0.990x | 1.238x |
| fen_write_native_ep | 1.001x | 0.997x | 1.592x |
| snapshot | 1.067x | 1.061x | 0.996x |
| san_parse_all | 1.000x | 0.995x | 1.840x |
| san_render_native | 0.996x | 0.985x | 2.549x |
| apply_children_full | 1.007x | 0.993x | 1.059x |
| make_unmake_full | 0.990x | 0.966x | 1.424x |
| startpos | 1.038x | 1.038x | 1.171x |
| chess960_materialize | 1.045x | 0.997x | 1.387x |
| chess960_perft_d3 | 1.015x | 0.964x | 1.345x |
| polyglot_key | 0.996x | 0.996x | 25.736x |
| polyglot_recompute | 1.000x | 0.995x | 1.074x |
| repetition_key | 1.008x | 1.001x | 19.373x |
| corpus_count | 0.825x | 0.795x | 1.206x |
| corpus_materialize | 0.972x | 0.963x | 1.171x |
| corpus_uci_sorted | 0.998x | 0.998x | 1.454x |
| replay_trusted_native | 0.995x | 0.988x | 0.492x |
| replay_checked_native | 0.990x | 0.986x | 0.981x |

Cold first legal move list: 0.011 ms -> 0.005 ms (2.09x).
