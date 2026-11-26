| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.028x | 0.983x | 1.329x |
| materialize | 1.027x | 0.991x | 1.204x |
| perft_bulk_d4 | 0.989x | 0.969x | 1.222x |
| perft_full_d3 | 1.050x | 1.006x | 1.251x |
| fen_parse | 0.918x | 0.901x | 1.026x |
| fen_write_native_ep | 1.029x | 1.003x | 1.704x |
| snapshot | 1.000x | 0.997x | 1.248x |
| san_parse_all | 0.984x | 0.962x | 1.997x |
| san_render_native | 1.016x | 0.972x | 2.156x |
| apply_children_full | 1.036x | 1.004x | 1.351x |
| make_unmake_full | 1.059x | 1.033x | 1.318x |
| startpos | 1.001x | 1.001x | 1.250x |
| chess960_materialize | 0.994x | 0.984x | 1.309x |
| chess960_perft_d3 | 1.008x | 1.003x | 1.396x |
| polyglot_key | 1.002x | 1.000x | 25.092x |
| polyglot_recompute | 1.013x | 1.012x | 1.030x |
| repetition_key | 1.003x | 0.997x | 18.990x |
| corpus_count | 1.006x | 1.000x | 1.287x |
| corpus_materialize | 0.988x | 0.987x | 1.089x |
| corpus_uci_sorted | 1.000x | 0.990x | 1.681x |
| replay_trusted_native | 1.048x | 1.024x | 0.922x |
| replay_checked_native | 0.971x | 0.960x | 1.001x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.07x).
