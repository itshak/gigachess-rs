| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.995x | 1.305x |
| materialize | 1.018x | 1.006x | 1.335x |
| perft_bulk_d4 | 0.994x | 0.983x | 1.300x |
| perft_full_d3 | 1.161x | 1.043x | 1.476x |
| fen_parse | 1.099x | 1.057x | 1.276x |
| fen_write_native_ep | 1.037x | 1.002x | 1.660x |
| snapshot | 0.987x | 0.923x | 0.991x |
| san_parse_all | 0.995x | 0.981x | 2.024x |
| san_render_native | 1.018x | 0.969x | 2.297x |
| apply_children_full | 1.010x | 0.991x | 1.420x |
| make_unmake_full | 0.970x | 0.943x | 1.399x |
| startpos | 0.999x | 0.999x | 1.332x |
| chess960_materialize | 0.984x | 0.981x | 1.456x |
| chess960_perft_d3 | 1.017x | 1.015x | 1.592x |
| polyglot_key | 0.923x | 0.852x | 25.622x |
| polyglot_recompute | 1.041x | 1.004x | 1.059x |
| repetition_key | 0.995x | 0.943x | 19.624x |
| corpus_count | 0.995x | 0.993x | 1.272x |
| corpus_materialize | 0.993x | 0.992x | 1.180x |
| corpus_uci_sorted | 1.002x | 0.998x | 1.727x |
| replay_trusted_native | 0.970x | 0.962x | 0.862x |
| replay_checked_native | 0.995x | 0.991x | 1.134x |

Cold first legal move list: 0.007 ms -> 0.009 ms (0.86x).
