| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.355x |
| materialize | 1.011x | 1.000x | 1.243x |
| perft_bulk_d4 | 0.997x | 0.993x | 1.353x |
| perft_full_d3 | 1.010x | 0.989x | 1.074x |
| fen_parse | 0.998x | 0.991x | 1.248x |
| fen_write_native_ep | 1.003x | 0.999x | 1.590x |
| snapshot | 1.112x | 1.111x | 0.887x |
| san_parse_all | 1.001x | 0.999x | 1.840x |
| san_render_native | 1.001x | 0.993x | 2.552x |
| apply_children_full | 1.006x | 0.985x | 1.031x |
| make_unmake_full | 1.003x | 0.992x | 1.440x |
| startpos | 1.076x | 1.076x | 1.183x |
| chess960_materialize | 0.996x | 0.991x | 1.323x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.328x |
| polyglot_key | 1.002x | 0.994x | 24.771x |
| polyglot_recompute | 0.992x | 0.990x | 1.065x |
| repetition_key | 0.987x | 0.967x | 18.316x |
| corpus_count | 0.997x | 0.993x | 1.490x |
| corpus_materialize | 0.997x | 0.993x | 1.226x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.459x |
| replay_trusted_native | 0.984x | 0.977x | 0.495x |
| replay_checked_native | 1.005x | 1.002x | 0.990x |

Cold first legal move list: 0.013 ms -> 0.014 ms (0.98x).
