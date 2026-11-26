| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.997x | 1.546x |
| materialize | 0.993x | 0.980x | 1.317x |
| perft_bulk_d4 | 0.999x | 0.993x | 1.393x |
| perft_full_d3 | 1.000x | 0.995x | 0.833x |
| fen_parse | 1.007x | 1.002x | 1.218x |
| fen_write_native_ep | 1.003x | 0.999x | 1.606x |
| snapshot | 1.010x | 1.008x | 0.937x |
| san_parse_all | 0.997x | 0.991x | 1.815x |
| san_render_native | 1.005x | 0.999x | 2.540x |
| apply_children_full | 1.002x | 0.994x | 1.159x |
| make_unmake_full | 0.996x | 0.975x | 1.548x |
| startpos | 0.999x | 0.999x | 1.273x |
| chess960_materialize | 0.998x | 0.998x | 1.494x |
| chess960_perft_d3 | 1.000x | 0.997x | 1.511x |
| polyglot_key | 0.973x | 0.946x | 24.896x |
| polyglot_recompute | 0.992x | 0.984x | 1.079x |
| repetition_key | 0.986x | 0.971x | 18.992x |
| corpus_count | 1.002x | 1.002x | 1.386x |
| corpus_materialize | 1.001x | 0.998x | 1.218x |
| corpus_uci_sorted | 1.007x | 1.005x | 1.458x |
| replay_trusted_native | 1.013x | 1.007x | 0.626x |
| replay_checked_native | 1.014x | 1.011x | 1.063x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.09x).
