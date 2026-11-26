| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.978x | 0.966x | 1.075x |
| materialize | 1.043x | 1.032x | 1.525x |
| perft_bulk_d4 | 1.073x | 1.052x | 1.026x |
| perft_full_d3 | 0.846x | 0.783x | 0.529x |
| fen_parse | 1.036x | 1.011x | 1.053x |
| fen_write_native_ep | 1.044x | 1.025x | 1.181x |
| snapshot | 1.118x | 1.109x | 0.500x |
| san_parse_all | 1.593x | 1.511x | 1.164x |
| san_render_native | 1.291x | 1.221x | 1.064x |
| apply_children_full | 1.039x | 0.991x | 0.452x |
| make_unmake_full | 1.127x | 1.089x | 0.859x |
| startpos | 0.873x | 0.873x | 0.751x |
| chess960_materialize | 1.155x | 1.066x | 1.394x |
| chess960_perft_d3 | 1.138x | 1.056x | 1.131x |
| polyglot_key | 0.967x | 0.914x | 34.691x |
| polyglot_recompute | 1.000x | 0.996x | 0.962x |
| repetition_key | 1.220x | 1.045x | 17.379x |
| corpus_count | 0.999x | 0.995x | 1.185x |
| corpus_materialize | 1.011x | 1.005x | 1.155x |
| corpus_uci_sorted | 1.178x | 1.176x | 0.760x |
| replay_trusted_native | 1.064x | 0.990x | 0.551x |
| replay_checked_native | 1.092x | 1.015x | 0.872x |

Cold first legal move list: 120.940 ms → 0.009 ms (13376.88x).
