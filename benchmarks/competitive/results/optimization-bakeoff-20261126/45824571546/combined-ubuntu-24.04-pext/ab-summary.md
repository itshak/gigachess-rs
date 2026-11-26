| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.952x | 1.409x |
| materialize | 0.965x | 0.951x | 1.482x |
| perft_bulk_d4 | 0.975x | 0.960x | 1.026x |
| perft_full_d3 | 1.006x | 0.976x | 1.623x |
| fen_parse | 0.999x | 0.966x | 1.124x |
| fen_write_native_ep | 0.997x | 0.980x | 1.970x |
| snapshot | 1.000x | 0.999x | 1.331x |
| san_parse_all | 0.895x | 0.879x | 1.713x |
| san_render_native | 0.996x | 0.979x | 2.185x |
| apply_children_full | 1.007x | 0.996x | 1.222x |
| make_unmake_full | 0.972x | 0.902x | 1.107x |
| startpos | 1.001x | 1.001x | 1.249x |
| chess960_materialize | 1.016x | 1.007x | 1.449x |
| chess960_perft_d3 | 0.983x | 0.981x | 1.352x |
| polyglot_key | 1.028x | 1.001x | 39.440x |
| polyglot_recompute | 0.987x | 0.930x | 1.010x |
| repetition_key | 0.930x | 0.782x | 16.530x |
| corpus_count | 0.987x | 0.974x | 0.959x |
| corpus_materialize | 0.998x | 0.996x | 1.054x |
| corpus_uci_sorted | 1.016x | 1.013x | 1.579x |
| replay_trusted_native | 0.976x | 0.960x | 0.869x |
| replay_checked_native | 0.956x | 0.950x | 1.007x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.14x).
