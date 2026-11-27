| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.964x | 0.831x | 1.429x |
| materialize | 0.986x | 0.937x | 1.268x |
| perft_bulk_d4 | 1.047x | 1.010x | 1.438x |
| perft_full_d3 | 1.213x | 1.037x | 1.633x |
| fen_parse | 0.987x | 0.796x | 1.267x |
| fen_write_native_ep | 0.942x | 0.891x | 1.597x |
| snapshot | 0.997x | 0.985x | 1.001x |
| san_parse_all | 1.000x | 0.990x | 2.066x |
| san_render_native | 0.998x | 0.933x | 3.934x |
| apply_children_full | 0.994x | 0.929x | 1.439x |
| make_unmake_full | 0.972x | 0.841x | 1.348x |
| startpos | 1.004x | 1.004x | 1.312x |
| chess960_materialize | 0.973x | 0.957x | 1.282x |
| chess960_perft_d3 | 0.968x | 0.938x | 1.576x |
| polyglot_key | 0.986x | 0.978x | 26.974x |
| polyglot_recompute | 0.985x | 0.966x | 1.073x |
| repetition_key | 0.914x | 0.909x | 19.072x |
| corpus_count | 1.014x | 0.951x | 1.521x |
| corpus_materialize | 0.976x | 0.945x | 1.134x |
| corpus_uci_sorted | 0.959x | 0.925x | 1.396x |
| replay_trusted_native | 0.954x | 0.918x | 0.867x |
| replay_checked_native | 0.993x | 0.978x | 1.094x |

Cold first legal move list: 0.015 ms -> 0.017 ms (0.93x).
