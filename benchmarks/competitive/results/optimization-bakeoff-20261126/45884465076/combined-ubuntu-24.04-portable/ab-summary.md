| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.967x | 1.251x |
| materialize | 0.982x | 0.972x | 1.722x |
| perft_bulk_d4 | 0.993x | 0.980x | 1.171x |
| perft_full_d3 | 0.991x | 0.972x | 1.304x |
| fen_parse | 1.001x | 0.988x | 1.043x |
| fen_write_native_ep | 0.996x | 0.981x | 1.782x |
| snapshot | 1.001x | 0.999x | 1.004x |
| san_parse_all | 1.006x | 0.997x | 1.906x |
| san_render_native | 1.017x | 1.002x | 2.191x |
| apply_children_full | 1.003x | 0.995x | 1.382x |
| make_unmake_full | 1.058x | 1.000x | 1.304x |
| startpos | 1.002x | 1.002x | 1.282x |
| chess960_materialize | 1.008x | 1.004x | 1.580x |
| chess960_perft_d3 | 1.001x | 0.989x | 1.508x |
| polyglot_key | 1.090x | 1.001x | 41.320x |
| polyglot_recompute | 1.019x | 1.010x | 0.994x |
| repetition_key | 1.010x | 0.984x | 20.051x |
| corpus_count | 0.963x | 0.932x | 1.087x |
| corpus_materialize | 0.998x | 0.993x | 1.133x |
| corpus_uci_sorted | 0.968x | 0.964x | 1.515x |
| replay_trusted_native | 1.022x | 1.020x | 0.894x |
| replay_checked_native | 1.014x | 1.012x | 1.201x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.01x).
