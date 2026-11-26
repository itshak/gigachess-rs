| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 1.001x | 1.364x |
| materialize | 1.017x | 1.013x | 1.253x |
| perft_bulk_d4 | 0.897x | 0.869x | 1.208x |
| perft_full_d3 | 0.947x | 0.916x | 1.007x |
| fen_parse | 0.948x | 0.929x | 1.170x |
| fen_write_native_ep | 0.990x | 0.984x | 1.034x |
| snapshot | 1.308x | 1.268x | 1.260x |
| san_parse_all | 0.970x | 0.944x | 1.753x |
| san_render_native | 0.829x | 0.772x | 2.109x |
| apply_children_full | 0.916x | 0.832x | 0.936x |
| make_unmake_full | 0.825x | 0.751x | 1.175x |
| startpos | 1.034x | 1.034x | 1.168x |
| chess960_materialize | 1.015x | 1.007x | 1.337x |
| chess960_perft_d3 | 0.913x | 0.848x | 1.202x |
| polyglot_key | 1.047x | 0.958x | 26.544x |
| polyglot_recompute | 0.996x | 0.992x | 1.073x |
| repetition_key | 1.183x | 1.128x | 21.736x |
| corpus_count | 1.011x | 1.009x | 1.501x |
| corpus_materialize | 1.008x | 1.002x | 1.215x |
| corpus_uci_sorted | 1.003x | 1.000x | 1.462x |
| replay_trusted_native | 0.895x | 0.888x | 0.461x |
| replay_checked_native | 0.889x | 0.882x | 0.883x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.97x).
