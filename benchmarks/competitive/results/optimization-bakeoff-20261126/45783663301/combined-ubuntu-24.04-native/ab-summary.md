| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.992x | 1.409x |
| materialize | 0.932x | 0.885x | 1.477x |
| perft_bulk_d4 | 0.992x | 0.983x | 1.201x |
| perft_full_d3 | 1.006x | 0.948x | 1.481x |
| fen_parse | 1.131x | 1.096x | 1.077x |
| fen_write_native_ep | 0.995x | 0.989x | 1.379x |
| snapshot | 1.289x | 1.281x | 1.327x |
| san_parse_all | 0.990x | 0.977x | 1.876x |
| san_render_native | 1.007x | 0.995x | 2.123x |
| apply_children_full | 1.037x | 1.004x | 1.154x |
| make_unmake_full | 0.991x | 0.970x | 1.145x |
| startpos | 1.371x | 1.371x | 1.258x |
| chess960_materialize | 1.000x | 0.999x | 1.440x |
| chess960_perft_d3 | 1.004x | 1.002x | 1.389x |
| polyglot_key | 0.985x | 0.970x | 39.089x |
| polyglot_recompute | 1.038x | 1.032x | 1.014x |
| repetition_key | 0.998x | 0.987x | 19.072x |
| corpus_count | 1.003x | 1.001x | 1.330x |
| corpus_materialize | 1.003x | 1.000x | 1.082x |
| corpus_uci_sorted | 0.997x | 0.994x | 1.574x |
| replay_trusted_native | 1.000x | 0.989x | 0.725x |
| replay_checked_native | 1.006x | 0.996x | 1.137x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.00x).
