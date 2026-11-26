| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.958x | 1.405x |
| materialize | 1.010x | 0.939x | 1.374x |
| perft_bulk_d4 | 1.012x | 0.981x | 1.284x |
| perft_full_d3 | 1.096x | 1.061x | 1.494x |
| fen_parse | 1.022x | 0.942x | 1.300x |
| fen_write_native_ep | 0.984x | 0.953x | 1.589x |
| snapshot | 1.126x | 1.048x | 0.986x |
| san_parse_all | 0.997x | 0.949x | 1.800x |
| san_render_native | 0.998x | 0.941x | 3.817x |
| apply_children_full | 0.773x | 0.718x | 0.833x |
| make_unmake_full | 1.002x | 0.938x | 1.073x |
| startpos | 0.956x | 0.956x | 1.056x |
| chess960_materialize | 0.957x | 0.944x | 1.387x |
| chess960_perft_d3 | 0.857x | 0.823x | 1.345x |
| polyglot_key | 0.965x | 0.900x | 43.486x |
| polyglot_recompute | 0.918x | 0.850x | 1.001x |
| repetition_key | 0.925x | 0.887x | 23.617x |
| corpus_count | 0.959x | 0.948x | 1.447x |
| corpus_materialize | 1.003x | 0.960x | 1.111x |
| corpus_uci_sorted | 1.017x | 0.979x | 1.577x |
| replay_trusted_native | 0.949x | 0.926x | 0.703x |
| replay_checked_native | 1.001x | 0.965x | 1.111x |

Cold first legal move list: 0.021 ms -> 0.022 ms (0.93x).
