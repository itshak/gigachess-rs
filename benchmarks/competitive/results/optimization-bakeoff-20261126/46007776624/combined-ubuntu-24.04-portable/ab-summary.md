| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.054x | 1.000x | 1.350x |
| materialize | 1.088x | 1.056x | 1.764x |
| perft_bulk_d4 | 1.069x | 1.046x | 1.266x |
| perft_full_d3 | 1.179x | 1.046x | 1.564x |
| fen_parse | 0.975x | 0.949x | 1.140x |
| fen_write_native_ep | 0.979x | 0.965x | 1.971x |
| snapshot | 0.995x | 0.988x | 0.999x |
| san_parse_all | 0.998x | 0.992x | 1.907x |
| san_render_native | 0.979x | 0.968x | 2.105x |
| apply_children_full | 1.008x | 1.001x | 1.364x |
| make_unmake_full | 0.998x | 0.972x | 1.264x |
| startpos | 0.998x | 0.998x | 1.287x |
| chess960_materialize | 1.060x | 1.058x | 1.576x |
| chess960_perft_d3 | 1.020x | 1.011x | 1.529x |
| polyglot_key | 1.003x | 1.002x | 40.392x |
| polyglot_recompute | 0.883x | 0.844x | 0.858x |
| repetition_key | 0.857x | 0.733x | 19.886x |
| corpus_count | 1.143x | 1.117x | 1.343x |
| corpus_materialize | 0.997x | 0.992x | 1.132x |
| corpus_uci_sorted | 1.001x | 0.995x | 1.545x |
| replay_trusted_native | 1.012x | 0.995x | 0.914x |
| replay_checked_native | 0.992x | 0.978x | 1.155x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.00x).
