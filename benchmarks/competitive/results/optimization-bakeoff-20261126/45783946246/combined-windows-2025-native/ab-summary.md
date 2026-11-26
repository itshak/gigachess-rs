| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.985x | 1.443x |
| materialize | 1.004x | 0.960x | 1.342x |
| perft_bulk_d4 | 1.007x | 0.989x | 1.230x |
| perft_full_d3 | 0.986x | 0.965x | 1.561x |
| fen_parse | 0.960x | 0.934x | 1.262x |
| fen_write_native_ep | 0.994x | 0.985x | 1.718x |
| snapshot | 1.043x | 1.034x | 1.128x |
| san_parse_all | 1.044x | 1.021x | 1.784x |
| san_render_native | 1.003x | 0.986x | 4.083x |
| apply_children_full | 1.011x | 0.999x | 1.123x |
| make_unmake_full | 0.967x | 0.952x | 1.067x |
| startpos | 1.000x | 1.000x | 0.985x |
| chess960_materialize | 1.006x | 0.974x | 1.352x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.377x |
| polyglot_key | 0.996x | 0.992x | 41.677x |
| polyglot_recompute | 1.020x | 1.006x | 1.041x |
| repetition_key | 1.017x | 0.913x | 24.983x |
| corpus_count | 1.018x | 0.995x | 1.316x |
| corpus_materialize | 1.041x | 1.026x | 1.128x |
| corpus_uci_sorted | 1.018x | 1.010x | 1.535x |
| replay_trusted_native | 1.017x | 0.997x | 0.696x |
| replay_checked_native | 1.023x | 1.017x | 1.085x |

Cold first legal move list: 0.038 ms -> 0.033 ms (1.18x).
