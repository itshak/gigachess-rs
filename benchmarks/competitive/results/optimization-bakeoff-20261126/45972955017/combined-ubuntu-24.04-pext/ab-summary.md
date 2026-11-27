| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.992x | 1.549x |
| materialize | 0.990x | 0.982x | 1.600x |
| perft_bulk_d4 | 1.012x | 0.991x | 1.201x |
| perft_full_d3 | 1.015x | 1.010x | 1.372x |
| fen_parse | 1.001x | 0.965x | 1.114x |
| fen_write_native_ep | 1.007x | 0.970x | 1.870x |
| snapshot | 1.003x | 0.997x | 1.003x |
| san_parse_all | 1.001x | 0.968x | 1.837x |
| san_render_native | 0.975x | 0.962x | 2.095x |
| apply_children_full | 1.003x | 0.998x | 1.357x |
| make_unmake_full | 0.988x | 0.961x | 1.189x |
| startpos | 0.799x | 0.799x | 1.000x |
| chess960_materialize | 1.011x | 0.993x | 1.526x |
| chess960_perft_d3 | 0.995x | 0.987x | 1.471x |
| polyglot_key | 0.991x | 0.981x | 39.306x |
| polyglot_recompute | 0.970x | 0.960x | 0.969x |
| repetition_key | 0.960x | 0.924x | 22.783x |
| corpus_count | 0.996x | 0.984x | 1.124x |
| corpus_materialize | 1.007x | 1.003x | 1.092x |
| corpus_uci_sorted | 1.014x | 1.011x | 1.552x |
| replay_trusted_native | 0.969x | 0.918x | 0.881x |
| replay_checked_native | 0.987x | 0.986x | 1.131x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.09x).
