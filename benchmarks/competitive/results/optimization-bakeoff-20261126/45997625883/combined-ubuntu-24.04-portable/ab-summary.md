| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.997x | 1.273x |
| materialize | 0.990x | 0.979x | 1.713x |
| perft_bulk_d4 | 0.998x | 0.994x | 1.181x |
| perft_full_d3 | 1.002x | 1.000x | 1.341x |
| fen_parse | 0.925x | 0.815x | 1.036x |
| fen_write_native_ep | 1.011x | 0.986x | 1.819x |
| snapshot | 0.995x | 0.975x | 0.999x |
| san_parse_all | 0.995x | 0.975x | 1.900x |
| san_render_native | 1.000x | 0.993x | 2.185x |
| apply_children_full | 1.000x | 0.994x | 1.362x |
| make_unmake_full | 1.017x | 0.993x | 1.231x |
| startpos | 0.988x | 0.988x | 1.346x |
| chess960_materialize | 0.944x | 0.937x | 1.485x |
| chess960_perft_d3 | 0.998x | 0.992x | 1.513x |
| polyglot_key | 1.015x | 0.991x | 42.430x |
| polyglot_recompute | 1.043x | 1.034x | 1.047x |
| repetition_key | 0.946x | 0.911x | 21.381x |
| corpus_count | 1.000x | 0.999x | 1.128x |
| corpus_materialize | 0.999x | 0.988x | 1.146x |
| corpus_uci_sorted | 1.009x | 1.005x | 1.554x |
| replay_trusted_native | 0.989x | 0.961x | 0.855x |
| replay_checked_native | 1.007x | 1.000x | 1.142x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.86x).
