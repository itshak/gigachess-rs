| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.996x | 1.558x |
| materialize | 1.024x | 0.992x | 1.145x |
| perft_bulk_d4 | 0.974x | 0.933x | 1.406x |
| perft_full_d3 | 1.005x | 0.997x | 1.237x |
| fen_parse | 1.134x | 1.075x | 1.504x |
| fen_write_native_ep | 0.983x | 0.966x | 1.573x |
| snapshot | 0.997x | 0.954x | 1.195x |
| san_parse_all | 1.022x | 0.963x | 1.972x |
| san_render_native | 1.012x | 0.999x | 3.311x |
| apply_children_full | 1.040x | 0.984x | 1.329x |
| make_unmake_full | 0.992x | 0.964x | 1.277x |
| startpos | 0.999x | 0.999x | 0.998x |
| chess960_materialize | 0.995x | 0.994x | 1.352x |
| chess960_perft_d3 | 0.813x | 0.673x | 1.299x |
| polyglot_key | 1.000x | 0.999x | 20.061x |
| polyglot_recompute | 1.035x | 1.019x | 0.979x |
| repetition_key | 1.022x | 1.012x | 19.186x |
| corpus_count | 0.994x | 0.988x | 1.307x |
| corpus_materialize | 0.999x | 0.998x | 1.183x |
| corpus_uci_sorted | 0.982x | 0.978x | 1.471x |
| replay_trusted_native | 0.979x | 0.949x | 1.137x |
| replay_checked_native | 1.084x | 1.063x | 1.091x |

Cold first legal move list: 0.016 ms -> 0.013 ms (1.17x).
