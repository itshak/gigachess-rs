| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.056x | 0.997x | 1.375x |
| materialize | 1.001x | 0.993x | 1.779x |
| perft_bulk_d4 | 1.064x | 1.035x | 1.274x |
| perft_full_d3 | 1.168x | 1.048x | 1.550x |
| fen_parse | 1.002x | 0.962x | 1.222x |
| fen_write_native_ep | 1.010x | 0.985x | 1.769x |
| snapshot | 0.999x | 0.996x | 1.000x |
| san_parse_all | 0.923x | 0.883x | 1.752x |
| san_render_native | 0.997x | 0.986x | 2.146x |
| apply_children_full | 1.000x | 0.997x | 1.360x |
| make_unmake_full | 1.054x | 1.007x | 1.266x |
| startpos | 0.998x | 0.998x | 1.352x |
| chess960_materialize | 0.939x | 0.935x | 1.456x |
| chess960_perft_d3 | 1.010x | 0.988x | 1.512x |
| polyglot_key | 1.012x | 0.998x | 40.875x |
| polyglot_recompute | 1.024x | 1.015x | 0.988x |
| repetition_key | 1.226x | 1.137x | 24.383x |
| corpus_count | 1.181x | 1.112x | 1.342x |
| corpus_materialize | 0.990x | 0.979x | 1.134x |
| corpus_uci_sorted | 0.988x | 0.984x | 1.542x |
| replay_trusted_native | 0.991x | 0.970x | 0.908x |
| replay_checked_native | 1.006x | 0.989x | 1.132x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.97x).
