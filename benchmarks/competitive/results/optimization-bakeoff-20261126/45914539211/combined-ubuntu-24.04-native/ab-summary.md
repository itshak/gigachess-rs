| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.054x | 1.023x | 1.519x |
| materialize | 1.016x | 1.004x | 1.267x |
| perft_bulk_d4 | 1.032x | 0.989x | 1.406x |
| perft_full_d3 | 1.235x | 1.039x | 1.630x |
| fen_parse | 1.061x | 1.051x | 1.175x |
| fen_write_native_ep | 1.010x | 0.993x | 1.787x |
| snapshot | 1.000x | 1.000x | 1.251x |
| san_parse_all | 0.992x | 0.977x | 2.044x |
| san_render_native | 1.002x | 0.993x | 2.231x |
| apply_children_full | 1.040x | 1.018x | 1.375x |
| make_unmake_full | 1.008x | 0.953x | 1.299x |
| startpos | 1.001x | 1.001x | 1.251x |
| chess960_materialize | 1.007x | 0.994x | 1.406x |
| chess960_perft_d3 | 0.992x | 0.984x | 1.512x |
| polyglot_key | 0.992x | 0.976x | 25.047x |
| polyglot_recompute | 1.004x | 1.000x | 1.034x |
| repetition_key | 0.992x | 0.991x | 19.317x |
| corpus_count | 1.068x | 1.043x | 1.458x |
| corpus_materialize | 1.013x | 1.011x | 1.145x |
| corpus_uci_sorted | 1.002x | 0.992x | 1.735x |
| replay_trusted_native | 0.968x | 0.947x | 0.874x |
| replay_checked_native | 0.997x | 0.990x | 1.080x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.93x).
