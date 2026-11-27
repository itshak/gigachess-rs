| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.034x | 0.985x | 1.487x |
| materialize | 1.006x | 0.989x | 1.282x |
| perft_bulk_d4 | 0.997x | 0.950x | 1.347x |
| perft_full_d3 | 0.987x | 0.974x | 1.312x |
| fen_parse | 0.997x | 0.972x | 1.275x |
| fen_write_native_ep | 0.989x | 0.981x | 1.718x |
| snapshot | 1.003x | 0.978x | 1.000x |
| san_parse_all | 1.007x | 0.997x | 2.149x |
| san_render_native | 1.001x | 0.998x | 3.801x |
| apply_children_full | 0.997x | 0.994x | 1.404x |
| make_unmake_full | 1.004x | 0.996x | 1.367x |
| startpos | 0.994x | 0.994x | 1.238x |
| chess960_materialize | 0.990x | 0.967x | 1.285x |
| chess960_perft_d3 | 0.994x | 0.992x | 1.545x |
| polyglot_key | 0.869x | 0.754x | 25.734x |
| polyglot_recompute | 1.001x | 1.000x | 1.016x |
| repetition_key | 0.958x | 0.926x | 20.779x |
| corpus_count | 0.990x | 0.983x | 1.350x |
| corpus_materialize | 0.999x | 0.999x | 1.196x |
| corpus_uci_sorted | 0.994x | 0.992x | 1.400x |
| replay_trusted_native | 1.004x | 0.995x | 0.845x |
| replay_checked_native | 0.983x | 0.977x | 1.068x |

Cold first legal move list: 0.017 ms -> 0.018 ms (0.93x).
