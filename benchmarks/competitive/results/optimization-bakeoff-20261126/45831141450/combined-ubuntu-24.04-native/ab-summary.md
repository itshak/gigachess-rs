| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.963x | 0.929x | 1.243x |
| materialize | 0.958x | 0.933x | 1.134x |
| perft_bulk_d4 | 0.979x | 0.947x | 1.167x |
| perft_full_d3 | 0.923x | 0.851x | 1.098x |
| fen_parse | 0.932x | 0.828x | 1.145x |
| fen_write_native_ep | 0.923x | 0.853x | 1.593x |
| snapshot | 1.010x | 0.999x | 1.250x |
| san_parse_all | 1.009x | 1.002x | 2.033x |
| san_render_native | 0.977x | 0.939x | 2.117x |
| apply_children_full | 0.986x | 0.946x | 1.271x |
| make_unmake_full | 0.946x | 0.911x | 1.183x |
| startpos | 1.503x | 1.503x | 1.879x |
| chess960_materialize | 0.962x | 0.922x | 1.256x |
| chess960_perft_d3 | 0.951x | 0.940x | 1.326x |
| polyglot_key | 1.013x | 0.997x | 26.097x |
| polyglot_recompute | 0.967x | 0.966x | 1.018x |
| repetition_key | 1.004x | 0.997x | 19.736x |
| corpus_count | 1.006x | 1.000x | 1.169x |
| corpus_materialize | 0.985x | 0.985x | 1.089x |
| corpus_uci_sorted | 1.039x | 1.033x | 1.856x |
| replay_trusted_native | 0.875x | 0.810x | 0.731x |
| replay_checked_native | 1.045x | 1.031x | 1.037x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.96x).
