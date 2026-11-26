| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 1.000x | 1.356x |
| materialize | 1.002x | 1.001x | 1.246x |
| perft_bulk_d4 | 1.000x | 0.998x | 1.346x |
| perft_full_d3 | 0.998x | 0.995x | 1.070x |
| fen_parse | 1.172x | 1.151x | 1.444x |
| fen_write_native_ep | 0.997x | 0.986x | 1.581x |
| snapshot | 0.986x | 0.985x | 0.915x |
| san_parse_all | 0.998x | 0.993x | 1.807x |
| san_render_native | 1.004x | 0.996x | 2.537x |
| apply_children_full | 0.999x | 0.991x | 1.034x |
| make_unmake_full | 0.998x | 0.994x | 1.432x |
| startpos | 1.000x | 1.000x | 1.182x |
| chess960_materialize | 1.001x | 0.998x | 1.326x |
| chess960_perft_d3 | 1.004x | 1.000x | 1.331x |
| polyglot_key | 0.983x | 0.978x | 25.515x |
| polyglot_recompute | 1.005x | 1.004x | 1.079x |
| repetition_key | 1.014x | 1.001x | 19.282x |
| corpus_count | 1.005x | 1.003x | 1.454x |
| corpus_materialize | 1.000x | 1.000x | 1.214x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.461x |
| replay_trusted_native | 1.007x | 1.001x | 0.503x |
| replay_checked_native | 0.999x | 0.998x | 0.989x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.06x).
