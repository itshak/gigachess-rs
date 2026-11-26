| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.113x | 0.986x | 1.549x |
| materialize | 1.004x | 0.941x | 1.396x |
| perft_bulk_d4 | 1.001x | 0.949x | 1.179x |
| perft_full_d3 | 0.960x | 0.802x | 1.412x |
| fen_parse | 0.893x | 0.850x | 1.028x |
| fen_write_native_ep | 1.009x | 0.961x | 1.691x |
| snapshot | 0.901x | 0.857x | 0.977x |
| san_parse_all | 0.986x | 0.930x | 1.772x |
| san_render_native | 0.996x | 0.842x | 4.158x |
| apply_children_full | 1.000x | 0.854x | 1.329x |
| make_unmake_full | 1.011x | 0.964x | 1.206x |
| startpos | 0.995x | 0.995x | 0.990x |
| chess960_materialize | 1.018x | 0.997x | 1.402x |
| chess960_perft_d3 | 0.996x | 0.986x | 1.527x |
| polyglot_key | 0.972x | 0.969x | 40.980x |
| polyglot_recompute | 1.046x | 0.967x | 1.056x |
| repetition_key | 1.176x | 0.987x | 32.943x |
| corpus_count | 0.913x | 0.871x | 0.907x |
| corpus_materialize | 0.996x | 0.993x | 1.096x |
| corpus_uci_sorted | 0.979x | 0.953x | 1.664x |
| replay_trusted_native | 0.919x | 0.884x | 0.773x |
| replay_checked_native | 0.894x | 0.843x | 0.992x |

Cold first legal move list: 0.017 ms -> 0.018 ms (0.95x).
