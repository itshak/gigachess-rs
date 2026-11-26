| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.984x | 1.494x |
| materialize | 0.963x | 0.938x | 1.232x |
| perft_bulk_d4 | 0.995x | 0.983x | 1.370x |
| perft_full_d3 | 0.994x | 0.981x | 1.321x |
| fen_parse | 0.851x | 0.709x | 1.120x |
| fen_write_native_ep | 0.994x | 0.854x | 1.537x |
| snapshot | 0.999x | 0.981x | 0.993x |
| san_parse_all | 0.993x | 0.985x | 2.089x |
| san_render_native | 0.963x | 0.933x | 3.660x |
| apply_children_full | 1.015x | 0.987x | 1.378x |
| make_unmake_full | 0.970x | 0.943x | 1.330x |
| startpos | 1.002x | 1.002x | 1.250x |
| chess960_materialize | 0.981x | 0.977x | 1.323x |
| chess960_perft_d3 | 0.984x | 0.977x | 1.520x |
| polyglot_key | 1.004x | 1.000x | 27.386x |
| polyglot_recompute | 1.000x | 0.993x | 1.013x |
| repetition_key | 0.976x | 0.971x | 20.589x |
| corpus_count | 0.996x | 0.985x | 1.343x |
| corpus_materialize | 1.001x | 0.997x | 1.199x |
| corpus_uci_sorted | 0.984x | 0.984x | 1.393x |
| replay_trusted_native | 0.999x | 0.994x | 0.834x |
| replay_checked_native | 0.987x | 0.973x | 0.992x |

Cold first legal move list: 0.016 ms -> 0.018 ms (0.87x).
