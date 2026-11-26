| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.992x | 1.173x |
| materialize | 0.987x | 0.972x | 1.656x |
| perft_bulk_d4 | 1.000x | 0.992x | 1.157x |
| perft_full_d3 | 0.995x | 0.967x | 1.467x |
| fen_parse | 0.996x | 0.990x | 1.172x |
| fen_write_native_ep | 0.974x | 0.959x | 1.302x |
| snapshot | 0.998x | 0.994x | 0.996x |
| san_parse_all | 1.001x | 0.992x | 1.856x |
| san_render_native | 0.986x | 0.973x | 2.079x |
| apply_children_full | 1.004x | 0.995x | 1.105x |
| make_unmake_full | 1.005x | 1.000x | 1.198x |
| startpos | 0.996x | 0.996x | 1.287x |
| chess960_materialize | 0.998x | 0.987x | 1.407x |
| chess960_perft_d3 | 0.997x | 0.994x | 1.330x |
| polyglot_key | 1.009x | 0.998x | 42.957x |
| polyglot_recompute | 0.940x | 0.921x | 1.013x |
| repetition_key | 0.872x | 0.747x | 22.181x |
| corpus_count | 1.006x | 1.001x | 1.262x |
| corpus_materialize | 1.003x | 1.002x | 1.147x |
| corpus_uci_sorted | 0.996x | 0.990x | 1.551x |
| replay_trusted_native | 0.994x | 0.976x | 0.676x |
| replay_checked_native | 0.983x | 0.980x | 1.093x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.88x).
