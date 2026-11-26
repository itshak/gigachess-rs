| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.925x | 1.300x |
| materialize | 1.020x | 0.980x | 1.215x |
| perft_bulk_d4 | 0.953x | 0.891x | 1.238x |
| perft_full_d3 | 0.963x | 0.912x | 1.174x |
| fen_parse | 0.914x | 0.889x | 1.090x |
| fen_write_native_ep | 1.132x | 1.077x | 1.779x |
| snapshot | 1.007x | 0.999x | 0.864x |
| san_parse_all | 0.992x | 0.964x | 1.971x |
| san_render_native | 1.097x | 1.065x | 2.318x |
| apply_children_full | 0.967x | 0.938x | 1.465x |
| make_unmake_full | 0.978x | 0.963x | 1.219x |
| startpos | 1.165x | 1.165x | 1.332x |
| chess960_materialize | 1.061x | 0.985x | 1.383x |
| chess960_perft_d3 | 1.025x | 0.939x | 1.389x |
| polyglot_key | 1.066x | 1.047x | 29.782x |
| polyglot_recompute | 0.977x | 0.976x | 1.010x |
| repetition_key | 1.013x | 1.008x | 21.147x |
| corpus_count | 0.912x | 0.821x | 1.220x |
| corpus_materialize | 0.944x | 0.918x | 1.125x |
| corpus_uci_sorted | 1.011x | 1.007x | 1.706x |
| replay_trusted_native | 0.998x | 0.985x | 0.704x |
| replay_checked_native | 1.001x | 0.988x | 1.032x |

Cold first legal move list: 0.014 ms -> 0.008 ms (1.86x).
