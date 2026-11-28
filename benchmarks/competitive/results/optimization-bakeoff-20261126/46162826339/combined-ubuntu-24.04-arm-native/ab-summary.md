| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.985x | 1.569x |
| materialize | 1.014x | 1.002x | 1.339x |
| perft_bulk_d4 | 1.038x | 1.016x | 1.451x |
| perft_full_d3 | 1.135x | 1.005x | 1.512x |
| fen_parse | 1.015x | 0.942x | 1.230x |
| fen_write_native_ep | 0.994x | 0.991x | 1.553x |
| snapshot | 0.989x | 0.984x | 0.946x |
| san_parse_all | 0.997x | 0.990x | 1.829x |
| san_render_native | 1.002x | 0.997x | 2.685x |
| apply_children_full | 0.996x | 0.968x | 1.152x |
| make_unmake_full | 1.011x | 0.983x | 1.573x |
| startpos | 0.999x | 0.999x | 1.301x |
| chess960_materialize | 1.008x | 1.007x | 1.507x |
| chess960_perft_d3 | 1.017x | 1.005x | 1.513x |
| polyglot_key | 1.032x | 1.011x | 25.459x |
| polyglot_recompute | 1.296x | 1.215x | 1.403x |
| repetition_key | 1.012x | 0.999x | 19.036x |
| corpus_count | 1.087x | 1.052x | 1.514x |
| corpus_materialize | 0.998x | 0.996x | 1.233x |
| corpus_uci_sorted | 1.001x | 1.000x | 1.459x |
| replay_trusted_native | 0.998x | 0.991x | 0.621x |
| replay_checked_native | 1.016x | 1.011x | 1.146x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.86x).
