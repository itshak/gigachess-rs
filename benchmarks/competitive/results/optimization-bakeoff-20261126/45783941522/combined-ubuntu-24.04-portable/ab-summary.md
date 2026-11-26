| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.936x | 1.192x |
| materialize | 0.985x | 0.944x | 1.254x |
| perft_bulk_d4 | 1.000x | 0.979x | 1.211x |
| perft_full_d3 | 0.994x | 0.917x | 1.326x |
| fen_parse | 1.015x | 0.986x | 1.173x |
| fen_write_native_ep | 1.001x | 0.997x | 1.687x |
| snapshot | 0.999x | 0.998x | 0.976x |
| san_parse_all | 0.963x | 0.940x | 1.995x |
| san_render_native | 1.030x | 1.010x | 2.402x |
| apply_children_full | 0.983x | 0.951x | 1.157x |
| make_unmake_full | 1.001x | 0.977x | 1.319x |
| startpos | 0.971x | 0.971x | 1.291x |
| chess960_materialize | 0.984x | 0.984x | 1.346x |
| chess960_perft_d3 | 1.003x | 0.999x | 1.426x |
| polyglot_key | 1.072x | 1.043x | 27.679x |
| polyglot_recompute | 1.027x | 1.027x | 1.025x |
| repetition_key | 1.095x | 1.084x | 21.117x |
| corpus_count | 0.988x | 0.984x | 1.265x |
| corpus_materialize | 1.004x | 1.000x | 1.169x |
| corpus_uci_sorted | 0.994x | 0.983x | 1.746x |
| replay_trusted_native | 0.977x | 0.954x | 0.702x |
| replay_checked_native | 0.991x | 0.988x | 1.034x |

Cold first legal move list: 0.014 ms -> 0.014 ms (1.01x).
