| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.998x | 1.484x |
| materialize | 0.996x | 0.978x | 1.599x |
| perft_bulk_d4 | 0.998x | 0.987x | 1.264x |
| perft_full_d3 | 0.935x | 0.914x | 1.482x |
| fen_parse | 0.953x | 0.879x | 0.967x |
| fen_write_native_ep | 1.000x | 0.980x | 1.933x |
| snapshot | 1.002x | 0.987x | 0.999x |
| san_parse_all | 1.160x | 1.095x | 1.887x |
| san_render_native | 1.010x | 1.002x | 2.185x |
| apply_children_full | 1.002x | 0.976x | 1.070x |
| make_unmake_full | 0.982x | 0.948x | 1.122x |
| startpos | 0.799x | 0.799x | 1.006x |
| chess960_materialize | 1.056x | 1.050x | 1.477x |
| chess960_perft_d3 | 1.000x | 0.997x | 1.375x |
| polyglot_key | 1.065x | 0.997x | 41.535x |
| polyglot_recompute | 0.960x | 0.948x | 1.014x |
| repetition_key | 1.190x | 1.039x | 22.706x |
| corpus_count | 1.001x | 0.999x | 1.419x |
| corpus_materialize | 1.002x | 1.002x | 1.115x |
| corpus_uci_sorted | 1.002x | 1.002x | 1.585x |
| replay_trusted_native | 1.039x | 1.034x | 0.743x |
| replay_checked_native | 1.024x | 1.023x | 1.093x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.94x).
