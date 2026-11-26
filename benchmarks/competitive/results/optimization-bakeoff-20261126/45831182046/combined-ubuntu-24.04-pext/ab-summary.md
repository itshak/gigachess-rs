| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 0.980x | 1.448x |
| materialize | 1.100x | 1.026x | 1.531x |
| perft_bulk_d4 | 1.048x | 0.965x | 1.112x |
| perft_full_d3 | 1.022x | 0.989x | 1.511x |
| fen_parse | 1.086x | 1.016x | 1.077x |
| fen_write_native_ep | 1.036x | 0.998x | 2.163x |
| snapshot | 1.004x | 0.996x | 1.155x |
| san_parse_all | 0.995x | 0.976x | 1.862x |
| san_render_native | 1.008x | 0.998x | 2.135x |
| apply_children_full | 1.027x | 1.008x | 1.337x |
| make_unmake_full | 1.006x | 0.953x | 1.175x |
| startpos | 1.002x | 1.002x | 1.121x |
| chess960_materialize | 1.129x | 1.071x | 1.465x |
| chess960_perft_d3 | 1.033x | 0.998x | 1.431x |
| polyglot_key | 0.976x | 0.953x | 39.363x |
| polyglot_recompute | 1.003x | 0.999x | 1.025x |
| repetition_key | 0.975x | 0.966x | 18.741x |
| corpus_count | 1.018x | 1.010x | 0.954x |
| corpus_materialize | 0.996x | 0.995x | 1.063x |
| corpus_uci_sorted | 1.016x | 1.015x | 1.589x |
| replay_trusted_native | 0.959x | 0.933x | 0.879x |
| replay_checked_native | 1.079x | 1.057x | 1.189x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.11x).
