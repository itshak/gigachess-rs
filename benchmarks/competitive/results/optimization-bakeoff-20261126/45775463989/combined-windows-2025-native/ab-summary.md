| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.994x | 1.312x |
| materialize | 0.994x | 0.976x | 1.091x |
| perft_bulk_d4 | 1.007x | 0.992x | 1.332x |
| perft_full_d3 | 0.972x | 0.952x | 0.610x |
| fen_parse | 0.976x | 0.964x | 1.426x |
| fen_write_native_ep | 0.992x | 0.963x | 1.206x |
| snapshot | 0.659x | 0.643x | 0.987x |
| san_parse_all | 1.004x | 0.996x | 1.718x |
| san_render_native | 1.027x | 1.001x | 3.467x |
| apply_children_full | 1.122x | 1.101x | 0.826x |
| make_unmake_full | 0.978x | 0.949x | 0.901x |
| startpos | 2.450x | 2.450x | 1.165x |
| chess960_materialize | 1.019x | 1.017x | 1.193x |
| chess960_perft_d3 | 0.977x | 0.961x | 1.358x |
| polyglot_key | 0.997x | 0.988x | 20.304x |
| polyglot_recompute | 1.000x | 0.993x | 0.986x |
| repetition_key | 1.058x | 0.996x | 17.521x |
| corpus_count | 0.962x | 0.953x | 1.250x |
| corpus_materialize | 0.942x | 0.917x | 1.089x |
| corpus_uci_sorted | 0.984x | 0.973x | 0.973x |
| replay_trusted_native | 0.999x | 0.996x | 0.515x |
| replay_checked_native | 0.779x | 0.718x | 0.696x |

Cold first legal move list: 0.029 ms -> 0.029 ms (0.99x).
