| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.931x | 1.471x |
| materialize | 0.918x | 0.768x | 1.180x |
| perft_bulk_d4 | 1.011x | 0.989x | 1.397x |
| perft_full_d3 | 0.984x | 0.897x | 1.283x |
| fen_parse | 0.979x | 0.954x | 1.247x |
| fen_write_native_ep | 0.990x | 0.968x | 1.587x |
| snapshot | 1.003x | 0.980x | 0.994x |
| san_parse_all | 0.999x | 0.952x | 1.984x |
| san_render_native | 0.979x | 0.952x | 3.620x |
| apply_children_full | 0.991x | 0.943x | 1.336x |
| make_unmake_full | 1.014x | 0.965x | 1.351x |
| startpos | 0.856x | 0.856x | 1.073x |
| chess960_materialize | 1.028x | 1.004x | 1.354x |
| chess960_perft_d3 | 1.049x | 0.997x | 1.546x |
| polyglot_key | 1.130x | 1.022x | 29.468x |
| polyglot_recompute | 0.978x | 0.975x | 0.991x |
| repetition_key | 1.025x | 0.994x | 21.770x |
| corpus_count | 1.003x | 1.002x | 1.370x |
| corpus_materialize | 1.069x | 1.042x | 1.146x |
| corpus_uci_sorted | 1.021x | 0.985x | 1.392x |
| replay_trusted_native | 1.036x | 1.026x | 0.875x |
| replay_checked_native | 1.026x | 0.986x | 1.012x |

Cold first legal move list: 0.024 ms -> 0.018 ms (1.35x).
