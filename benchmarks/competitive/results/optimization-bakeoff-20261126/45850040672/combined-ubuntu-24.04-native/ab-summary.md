| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.938x | 1.489x |
| materialize | 1.000x | 0.981x | 1.576x |
| perft_bulk_d4 | 0.991x | 0.986x | 1.163x |
| perft_full_d3 | 1.001x | 0.997x | 1.333x |
| fen_parse | 1.002x | 0.983x | 1.110x |
| fen_write_native_ep | 0.964x | 0.916x | 1.829x |
| snapshot | 0.998x | 0.996x | 1.329x |
| san_parse_all | 1.085x | 1.052x | 1.880x |
| san_render_native | 1.007x | 0.999x | 2.152x |
| apply_children_full | 1.029x | 1.011x | 1.264x |
| make_unmake_full | 1.021x | 0.965x | 1.188x |
| startpos | 1.000x | 1.000x | 1.250x |
| chess960_materialize | 1.018x | 1.009x | 1.546x |
| chess960_perft_d3 | 0.994x | 0.994x | 1.508x |
| polyglot_key | 1.015x | 1.000x | 39.693x |
| polyglot_recompute | 1.001x | 0.997x | 1.024x |
| repetition_key | 1.030x | 0.990x | 23.736x |
| corpus_count | 0.983x | 0.975x | 1.083x |
| corpus_materialize | 1.000x | 0.998x | 1.070x |
| corpus_uci_sorted | 1.001x | 0.996x | 1.552x |
| replay_trusted_native | 1.052x | 1.005x | 0.913x |
| replay_checked_native | 1.108x | 1.077x | 1.156x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.88x).
