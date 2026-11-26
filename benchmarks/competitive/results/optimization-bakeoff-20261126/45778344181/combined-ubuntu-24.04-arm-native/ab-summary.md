| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 0.997x | 1.364x |
| materialize | 1.020x | 1.009x | 1.258x |
| perft_bulk_d4 | 0.902x | 0.874x | 1.216x |
| perft_full_d3 | 0.923x | 0.904x | 0.990x |
| fen_parse | 0.950x | 0.933x | 1.176x |
| fen_write_native_ep | 1.014x | 1.004x | 1.050x |
| snapshot | 1.043x | 1.038x | 0.893x |
| san_parse_all | 0.973x | 0.935x | 1.782x |
| san_render_native | 0.839x | 0.764x | 2.130x |
| apply_children_full | 0.866x | 0.795x | 0.895x |
| make_unmake_full | 0.844x | 0.775x | 1.196x |
| startpos | 0.734x | 0.734x | 0.838x |
| chess960_materialize | 1.021x | 1.020x | 1.348x |
| chess960_perft_d3 | 0.917x | 0.858x | 1.211x |
| polyglot_key | 1.030x | 0.994x | 26.459x |
| polyglot_recompute | 1.010x | 1.004x | 1.090x |
| repetition_key | 1.062x | 1.012x | 20.395x |
| corpus_count | 1.007x | 1.001x | 1.477x |
| corpus_materialize | 1.010x | 1.005x | 1.202x |
| corpus_uci_sorted | 1.001x | 0.997x | 1.462x |
| replay_trusted_native | 0.875x | 0.845x | 0.434x |
| replay_checked_native | 0.896x | 0.879x | 0.894x |

Cold first legal move list: 0.010 ms -> 0.012 ms (0.86x).
