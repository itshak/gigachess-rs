| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.981x | 1.489x |
| materialize | 0.978x | 0.960x | 1.151x |
| perft_bulk_d4 | 0.995x | 0.961x | 1.319x |
| perft_full_d3 | 1.000x | 0.953x | 0.918x |
| fen_parse | 0.946x | 0.913x | 1.121x |
| fen_write_native_ep | 1.002x | 0.998x | 1.929x |
| snapshot | 0.999x | 0.997x | 1.250x |
| san_parse_all | 0.984x | 0.963x | 2.240x |
| san_render_native | 0.999x | 0.939x | 2.086x |
| apply_children_full | 1.030x | 0.937x | 1.312x |
| make_unmake_full | 0.978x | 0.925x | 1.217x |
| startpos | 1.003x | 1.003x | 1.203x |
| chess960_materialize | 0.997x | 0.990x | 1.549x |
| chess960_perft_d3 | 0.992x | 0.989x | 1.596x |
| polyglot_key | 1.006x | 1.000x | 20.581x |
| polyglot_recompute | 1.088x | 1.004x | 1.064x |
| repetition_key | 1.009x | 1.001x | 18.915x |
| corpus_count | 1.001x | 0.996x | 1.289x |
| corpus_materialize | 0.993x | 0.992x | 1.224x |
| corpus_uci_sorted | 0.988x | 0.988x | 1.648x |
| replay_trusted_native | 0.828x | 0.749x | 0.957x |
| replay_checked_native | 1.042x | 1.015x | 1.088x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.04x).
