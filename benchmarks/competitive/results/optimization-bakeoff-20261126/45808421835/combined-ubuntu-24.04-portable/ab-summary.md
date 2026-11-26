| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.990x | 0.956x | 1.207x |
| materialize | 0.998x | 0.988x | 1.288x |
| perft_bulk_d4 | 1.015x | 0.992x | 1.237x |
| perft_full_d3 | 1.017x | 1.000x | 1.348x |
| fen_parse | 0.950x | 0.926x | 1.204x |
| fen_write_native_ep | 1.012x | 1.002x | 1.728x |
| snapshot | 1.007x | 0.998x | 0.971x |
| san_parse_all | 1.025x | 1.010x | 2.057x |
| san_render_native | 1.000x | 0.987x | 2.409x |
| apply_children_full | 0.980x | 0.956x | 1.224x |
| make_unmake_full | 1.017x | 0.984x | 1.333x |
| startpos | 0.998x | 0.998x | 1.279x |
| chess960_materialize | 0.994x | 0.985x | 1.320x |
| chess960_perft_d3 | 0.999x | 0.994x | 1.392x |
| polyglot_key | 1.003x | 1.002x | 24.090x |
| polyglot_recompute | 1.003x | 1.003x | 1.038x |
| repetition_key | 0.970x | 0.940x | 18.171x |
| corpus_count | 0.994x | 0.993x | 1.288x |
| corpus_materialize | 0.992x | 0.991x | 1.232x |
| corpus_uci_sorted | 0.996x | 0.991x | 1.805x |
| replay_trusted_native | 1.043x | 1.042x | 0.800x |
| replay_checked_native | 1.053x | 1.051x | 1.076x |

Cold first legal move list: 0.014 ms -> 0.014 ms (1.00x).
