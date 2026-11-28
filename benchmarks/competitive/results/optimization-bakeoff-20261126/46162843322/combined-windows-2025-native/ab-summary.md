| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.141x | 1.044x | 1.570x |
| materialize | 1.018x | 0.948x | 1.203x |
| perft_bulk_d4 | 1.059x | 1.018x | 1.387x |
| perft_full_d3 | 1.209x | 1.086x | 1.548x |
| fen_parse | 1.049x | 1.010x | 1.370x |
| fen_write_native_ep | 1.036x | 1.008x | 1.624x |
| snapshot | 1.007x | 0.992x | 0.997x |
| san_parse_all | 1.008x | 0.970x | 2.086x |
| san_render_native | 1.033x | 0.998x | 3.929x |
| apply_children_full | 0.952x | 0.861x | 1.314x |
| make_unmake_full | 0.998x | 0.913x | 1.317x |
| startpos | 0.999x | 0.999x | 1.245x |
| chess960_materialize | 0.997x | 0.995x | 1.370x |
| chess960_perft_d3 | 0.995x | 0.992x | 1.528x |
| polyglot_key | 0.994x | 0.983x | 30.392x |
| polyglot_recompute | 1.102x | 0.991x | 1.139x |
| repetition_key | 0.991x | 0.990x | 22.331x |
| corpus_count | 1.084x | 1.069x | 1.461x |
| corpus_materialize | 0.979x | 0.973x | 1.131x |
| corpus_uci_sorted | 0.978x | 0.950x | 1.448x |
| replay_trusted_native | 0.966x | 0.940x | 0.823x |
| replay_checked_native | 0.933x | 0.885x | 1.034x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.03x).
