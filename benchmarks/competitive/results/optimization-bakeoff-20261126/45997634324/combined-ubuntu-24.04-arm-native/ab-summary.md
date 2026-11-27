| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.972x | 1.551x |
| materialize | 1.015x | 1.000x | 1.452x |
| perft_bulk_d4 | 1.033x | 1.001x | 1.570x |
| perft_full_d3 | 1.156x | 1.020x | 1.408x |
| fen_parse | 0.992x | 0.973x | 1.089x |
| fen_write_native_ep | 1.005x | 0.993x | 1.573x |
| snapshot | 1.001x | 0.999x | 0.986x |
| san_parse_all | 1.000x | 0.996x | 2.012x |
| san_render_native | 0.998x | 0.992x | 2.082x |
| apply_children_full | 1.001x | 0.997x | 1.030x |
| make_unmake_full | 1.003x | 0.998x | 1.696x |
| startpos | 0.998x | 0.998x | 1.333x |
| chess960_materialize | 1.010x | 1.000x | 1.513x |
| chess960_perft_d3 | 1.018x | 1.000x | 1.748x |
| polyglot_key | 1.003x | 1.000x | 39.594x |
| polyglot_recompute | 0.999x | 0.998x | 1.080x |
| repetition_key | 0.988x | 0.977x | 30.156x |
| corpus_count | 1.083x | 1.055x | 1.524x |
| corpus_materialize | 1.018x | 1.010x | 1.413x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.552x |
| replay_trusted_native | 1.016x | 1.009x | 0.674x |
| replay_checked_native | 1.029x | 1.021x | 1.275x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.83x).
