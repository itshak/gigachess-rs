| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.548x |
| materialize | 1.003x | 1.000x | 1.322x |
| perft_bulk_d4 | 1.002x | 0.999x | 1.396x |
| perft_full_d3 | 1.000x | 0.995x | 1.329x |
| fen_parse | 1.000x | 0.996x | 1.217x |
| fen_write_native_ep | 1.001x | 0.998x | 1.575x |
| snapshot | 0.991x | 0.988x | 0.959x |
| san_parse_all | 1.000x | 0.996x | 1.837x |
| san_render_native | 1.001x | 0.995x | 2.672x |
| apply_children_full | 1.001x | 0.984x | 1.156x |
| make_unmake_full | 1.015x | 1.004x | 1.582x |
| startpos | 1.002x | 1.002x | 1.333x |
| chess960_materialize | 1.011x | 1.007x | 1.500x |
| chess960_perft_d3 | 0.999x | 0.999x | 1.499x |
| polyglot_key | 0.997x | 0.996x | 25.337x |
| polyglot_recompute | 0.993x | 0.988x | 1.082x |
| repetition_key | 0.997x | 0.992x | 18.933x |
| corpus_count | 0.993x | 0.988x | 1.378x |
| corpus_materialize | 0.986x | 0.984x | 1.222x |
| corpus_uci_sorted | 0.995x | 0.991x | 1.453x |
| replay_trusted_native | 1.021x | 1.004x | 0.637x |
| replay_checked_native | 0.988x | 0.986x | 1.115x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).
