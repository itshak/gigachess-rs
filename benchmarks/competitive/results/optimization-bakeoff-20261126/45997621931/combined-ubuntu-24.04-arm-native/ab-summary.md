| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.550x |
| materialize | 0.996x | 0.989x | 1.315x |
| perft_bulk_d4 | 1.001x | 0.997x | 1.404x |
| perft_full_d3 | 0.997x | 0.971x | 1.327x |
| fen_parse | 0.991x | 0.977x | 1.217x |
| fen_write_native_ep | 0.997x | 0.990x | 1.573x |
| snapshot | 1.008x | 1.004x | 0.946x |
| san_parse_all | 1.006x | 0.989x | 1.830x |
| san_render_native | 1.007x | 0.974x | 2.683x |
| apply_children_full | 1.003x | 0.963x | 1.165x |
| make_unmake_full | 1.009x | 0.970x | 1.569x |
| startpos | 1.000x | 1.000x | 1.297x |
| chess960_materialize | 0.984x | 0.982x | 1.474x |
| chess960_perft_d3 | 0.988x | 0.985x | 1.481x |
| polyglot_key | 0.973x | 0.954x | 25.232x |
| polyglot_recompute | 0.996x | 0.994x | 1.086x |
| repetition_key | 0.987x | 0.974x | 19.065x |
| corpus_count | 0.999x | 0.999x | 1.386x |
| corpus_materialize | 0.994x | 0.993x | 1.225x |
| corpus_uci_sorted | 0.993x | 0.991x | 1.469x |
| replay_trusted_native | 0.942x | 0.924x | 0.606x |
| replay_checked_native | 1.002x | 0.992x | 1.140x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.93x).
