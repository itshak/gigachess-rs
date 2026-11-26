| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.973x | 1.523x |
| materialize | 1.003x | 0.978x | 1.152x |
| perft_bulk_d4 | 1.054x | 1.013x | 1.460x |
| perft_full_d3 | 1.316x | 1.080x | 1.702x |
| fen_parse | 1.004x | 0.992x | 1.403x |
| fen_write_native_ep | 1.041x | 0.977x | 1.634x |
| snapshot | 1.025x | 0.991x | 1.102x |
| san_parse_all | 1.001x | 0.990x | 1.985x |
| san_render_native | 1.015x | 0.982x | 3.485x |
| apply_children_full | 1.002x | 0.980x | 1.311x |
| make_unmake_full | 0.998x | 0.990x | 1.303x |
| startpos | 0.996x | 0.996x | 0.999x |
| chess960_materialize | 1.010x | 1.008x | 1.365x |
| chess960_perft_d3 | 0.998x | 0.987x | 1.597x |
| polyglot_key | 1.007x | 0.989x | 20.217x |
| polyglot_recompute | 1.007x | 1.005x | 0.976x |
| repetition_key | 0.998x | 0.993x | 19.011x |
| corpus_count | 1.040x | 1.010x | 1.362x |
| corpus_materialize | 1.004x | 1.001x | 1.199x |
| corpus_uci_sorted | 1.002x | 1.001x | 1.501x |
| replay_trusted_native | 1.056x | 1.006x | 1.187x |
| replay_checked_native | 1.008x | 1.004x | 1.080x |

Cold first legal move list: 0.016 ms -> 0.013 ms (1.18x).
