| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.939x | 0.875x | 1.293x |
| materialize | 0.964x | 0.958x | 1.443x |
| perft_bulk_d4 | 0.975x | 0.959x | 0.988x |
| perft_full_d3 | 0.959x | 0.912x | 1.374x |
| fen_parse | 1.049x | 1.005x | 1.126x |
| fen_write_native_ep | 0.922x | 0.855x | 1.895x |
| snapshot | 0.548x | 0.546x | 0.999x |
| san_parse_all | 0.948x | 0.911x | 1.832x |
| san_render_native | 1.014x | 0.978x | 2.099x |
| apply_children_full | 0.960x | 0.926x | 1.279x |
| make_unmake_full | 0.916x | 0.705x | 1.073x |
| startpos | 0.997x | 0.997x | 1.495x |
| chess960_materialize | 0.937x | 0.911x | 1.388x |
| chess960_perft_d3 | 0.985x | 0.980x | 1.357x |
| polyglot_key | 1.064x | 1.001x | 38.962x |
| polyglot_recompute | 0.993x | 0.993x | 1.013x |
| repetition_key | 0.891x | 0.715x | 19.615x |
| corpus_count | 0.971x | 0.965x | 0.851x |
| corpus_materialize | 1.002x | 1.000x | 1.027x |
| corpus_uci_sorted | 1.050x | 1.049x | 1.571x |
| replay_trusted_native | 0.986x | 0.976x | 0.864x |
| replay_checked_native | 0.963x | 0.960x | 0.989x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.91x).
