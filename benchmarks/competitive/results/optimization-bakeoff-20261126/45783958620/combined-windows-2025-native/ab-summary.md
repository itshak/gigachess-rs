| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.996x | 1.364x |
| materialize | 0.984x | 0.950x | 1.085x |
| perft_bulk_d4 | 0.996x | 0.935x | 1.357x |
| perft_full_d3 | 0.889x | 0.831x | 0.959x |
| fen_parse | 0.945x | 0.885x | 1.187x |
| fen_write_native_ep | 0.994x | 0.975x | 1.674x |
| snapshot | 0.322x | 0.286x | 0.870x |
| san_parse_all | 0.987x | 0.929x | 1.975x |
| san_render_native | 1.105x | 1.014x | 3.364x |
| apply_children_full | 0.933x | 0.893x | 0.958x |
| make_unmake_full | 0.862x | 0.823x | 1.065x |
| startpos | 1.000x | 1.000x | 2.741x |
| chess960_materialize | 0.936x | 0.870x | 1.146x |
| chess960_perft_d3 | 0.984x | 0.961x | 1.375x |
| polyglot_key | 1.000x | 0.999x | 22.030x |
| polyglot_recompute | 0.998x | 0.997x | 0.981x |
| repetition_key | 1.058x | 0.993x | 17.408x |
| corpus_count | 1.018x | 1.018x | 1.544x |
| corpus_materialize | 0.948x | 0.896x | 1.125x |
| corpus_uci_sorted | 0.994x | 0.971x | 1.526x |
| replay_trusted_native | 0.875x | 0.838x | 0.651x |
| replay_checked_native | 0.970x | 0.929x | 0.915x |

Cold first legal move list: 0.027 ms -> 0.028 ms (0.96x).
