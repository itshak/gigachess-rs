| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.967x | 1.190x |
| materialize | 0.990x | 0.959x | 1.256x |
| perft_bulk_d4 | 0.998x | 0.962x | 1.231x |
| perft_full_d3 | 1.000x | 0.898x | 1.282x |
| fen_parse | 0.936x | 0.930x | 1.126x |
| fen_write_native_ep | 1.023x | 1.006x | 1.682x |
| snapshot | 0.999x | 0.997x | 1.016x |
| san_parse_all | 0.974x | 0.961x | 1.943x |
| san_render_native | 0.997x | 0.983x | 2.393x |
| apply_children_full | 1.000x | 0.996x | 1.157x |
| make_unmake_full | 1.018x | 1.012x | 1.353x |
| startpos | 0.996x | 0.996x | 1.272x |
| chess960_materialize | 0.998x | 0.994x | 1.423x |
| chess960_perft_d3 | 0.992x | 0.983x | 1.401x |
| polyglot_key | 1.101x | 1.041x | 23.518x |
| polyglot_recompute | 0.958x | 0.943x | 0.957x |
| repetition_key | 1.034x | 0.970x | 18.092x |
| corpus_count | 0.996x | 0.992x | 1.303x |
| corpus_materialize | 1.002x | 0.999x | 1.176x |
| corpus_uci_sorted | 1.151x | 1.136x | 1.841x |
| replay_trusted_native | 0.983x | 0.966x | 0.690x |
| replay_checked_native | 1.010x | 1.006x | 1.070x |

Cold first legal move list: 0.014 ms -> 0.014 ms (1.01x).
