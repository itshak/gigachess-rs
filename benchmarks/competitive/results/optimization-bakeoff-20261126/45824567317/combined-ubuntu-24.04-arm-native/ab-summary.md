| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.382x |
| materialize | 1.007x | 1.003x | 1.249x |
| perft_bulk_d4 | 1.007x | 0.999x | 1.242x |
| perft_full_d3 | 1.009x | 1.004x | 1.073x |
| fen_parse | 1.016x | 1.014x | 1.253x |
| fen_write_native_ep | 0.998x | 0.997x | 1.596x |
| snapshot | 0.991x | 0.987x | 0.960x |
| san_parse_all | 0.999x | 0.993x | 1.815x |
| san_render_native | 1.060x | 1.024x | 2.663x |
| apply_children_full | 1.004x | 0.987x | 1.291x |
| make_unmake_full | 1.013x | 0.966x | 1.533x |
| startpos | 1.001x | 1.001x | 1.330x |
| chess960_materialize | 0.999x | 0.997x | 1.388x |
| chess960_perft_d3 | 1.001x | 0.989x | 1.352x |
| polyglot_key | 0.994x | 0.994x | 25.306x |
| polyglot_recompute | 1.007x | 1.006x | 1.072x |
| repetition_key | 0.998x | 0.997x | 18.292x |
| corpus_count | 0.986x | 0.977x | 1.180x |
| corpus_materialize | 0.999x | 0.993x | 1.168x |
| corpus_uci_sorted | 0.997x | 0.995x | 1.451x |
| replay_trusted_native | 1.006x | 0.987x | 0.612x |
| replay_checked_native | 1.047x | 1.036x | 1.109x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.98x).
