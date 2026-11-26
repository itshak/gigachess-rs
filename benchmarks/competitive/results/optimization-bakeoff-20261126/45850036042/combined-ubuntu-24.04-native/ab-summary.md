| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.998x | 1.506x |
| materialize | 1.000x | 0.976x | 1.605x |
| perft_bulk_d4 | 1.001x | 0.991x | 1.177x |
| perft_full_d3 | 1.013x | 1.003x | 1.373x |
| fen_parse | 0.996x | 0.974x | 1.111x |
| fen_write_native_ep | 1.025x | 0.997x | 1.922x |
| snapshot | 0.999x | 0.990x | 1.151x |
| san_parse_all | 1.079x | 1.035x | 1.858x |
| san_render_native | 0.972x | 0.951x | 2.066x |
| apply_children_full | 0.997x | 0.993x | 1.297x |
| make_unmake_full | 1.004x | 0.999x | 1.177x |
| startpos | 0.894x | 0.894x | 1.117x |
| chess960_materialize | 1.091x | 1.076x | 1.562x |
| chess960_perft_d3 | 1.004x | 1.001x | 1.508x |
| polyglot_key | 1.072x | 1.000x | 39.916x |
| polyglot_recompute | 0.997x | 0.994x | 1.023x |
| repetition_key | 0.986x | 0.966x | 22.726x |
| corpus_count | 1.004x | 0.997x | 1.104x |
| corpus_materialize | 0.998x | 0.998x | 1.063x |
| corpus_uci_sorted | 1.008x | 1.006x | 1.569x |
| replay_trusted_native | 0.993x | 0.992x | 0.879x |
| replay_checked_native | 1.005x | 1.001x | 1.124x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.16x).
