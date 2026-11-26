| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.970x | 1.311x |
| materialize | 1.003x | 0.984x | 1.221x |
| perft_bulk_d4 | 0.991x | 0.974x | 1.234x |
| perft_full_d3 | 1.034x | 0.984x | 1.225x |
| fen_parse | 1.010x | 0.981x | 1.161x |
| fen_write_native_ep | 1.020x | 0.979x | 1.718x |
| snapshot | 0.779x | 0.778x | 0.975x |
| san_parse_all | 1.001x | 0.998x | 2.090x |
| san_render_native | 0.998x | 0.995x | 2.391x |
| apply_children_full | 1.000x | 0.994x | 1.289x |
| make_unmake_full | 1.010x | 0.993x | 1.260x |
| startpos | 1.000x | 1.000x | 1.250x |
| chess960_materialize | 0.986x | 0.976x | 1.331x |
| chess960_perft_d3 | 1.014x | 1.012x | 1.405x |
| polyglot_key | 0.779x | 0.637x | 23.180x |
| polyglot_recompute | 0.975x | 0.974x | 1.005x |
| repetition_key | 0.936x | 0.885x | 18.472x |
| corpus_count | 0.995x | 0.993x | 1.282x |
| corpus_materialize | 1.005x | 1.002x | 1.125x |
| corpus_uci_sorted | 0.996x | 0.992x | 1.701x |
| replay_trusted_native | 0.989x | 0.984x | 0.855x |
| replay_checked_native | 1.003x | 0.994x | 1.022x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.01x).
