| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.924x | 1.304x |
| materialize | 1.001x | 0.996x | 1.286x |
| perft_bulk_d4 | 0.993x | 0.986x | 1.005x |
| perft_full_d3 | 0.996x | 0.988x | 1.548x |
| fen_parse | 1.043x | 1.006x | 1.258x |
| fen_write_native_ep | 0.993x | 0.949x | 1.670x |
| snapshot | 0.991x | 0.878x | 1.114x |
| san_parse_all | 0.994x | 0.933x | 1.747x |
| san_render_native | 1.002x | 0.989x | 4.073x |
| apply_children_full | 0.991x | 0.944x | 1.308x |
| make_unmake_full | 0.997x | 0.987x | 1.202x |
| startpos | 1.004x | 1.004x | 1.010x |
| chess960_materialize | 0.990x | 0.985x | 1.256x |
| chess960_perft_d3 | 1.018x | 0.963x | 1.374x |
| polyglot_key | 0.998x | 0.994x | 40.717x |
| polyglot_recompute | 1.013x | 1.011x | 1.004x |
| repetition_key | 1.186x | 1.006x | 32.711x |
| corpus_count | 0.999x | 0.994x | 0.796x |
| corpus_materialize | 1.009x | 1.005x | 1.019x |
| corpus_uci_sorted | 1.015x | 1.007x | 1.528x |
| replay_trusted_native | 1.065x | 1.010x | 0.849x |
| replay_checked_native | 1.033x | 1.017x | 1.165x |

Cold first legal move list: 0.017 ms -> 0.017 ms (0.99x).
