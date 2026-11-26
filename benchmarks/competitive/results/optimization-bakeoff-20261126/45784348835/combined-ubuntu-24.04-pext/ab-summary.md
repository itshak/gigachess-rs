| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.995x | 1.433x |
| materialize | 1.039x | 0.997x | 1.170x |
| perft_bulk_d4 | 0.980x | 0.969x | 1.308x |
| perft_full_d3 | 1.025x | 0.995x | 0.995x |
| fen_parse | 0.985x | 0.969x | 1.086x |
| fen_write_native_ep | 0.998x | 0.990x | 1.843x |
| snapshot | 0.997x | 0.971x | 1.108x |
| san_parse_all | 1.006x | 0.998x | 2.183x |
| san_render_native | 1.013x | 0.998x | 2.183x |
| apply_children_full | 1.080x | 1.058x | 1.178x |
| make_unmake_full | 0.995x | 0.953x | 1.210x |
| startpos | 0.409x | 0.409x | 0.483x |
| chess960_materialize | 0.997x | 0.987x | 1.488x |
| chess960_perft_d3 | 1.000x | 0.998x | 1.480x |
| polyglot_key | 0.989x | 0.980x | 20.263x |
| polyglot_recompute | 0.993x | 0.990x | 1.019x |
| repetition_key | 1.000x | 1.000x | 18.888x |
| corpus_count | 1.016x | 1.010x | 1.473x |
| corpus_materialize | 1.039x | 1.023x | 1.282x |
| corpus_uci_sorted | 1.019x | 1.012x | 1.683x |
| replay_trusted_native | 1.174x | 1.087x | 0.808x |
| replay_checked_native | 1.051x | 1.030x | 0.952x |

Cold first legal move list: 0.011 ms -> 0.011 ms (0.97x).
